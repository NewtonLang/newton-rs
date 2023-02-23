pub mod ast;
pub mod codegen;
pub mod error;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod types;

use ast::ast::*;
use parser::span::*;
use types::types::*;

use ansi_term::Colour::*;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub code: String,
}

impl Source {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_owned(),
            code: code.to_owned(),
        }
    }

    pub fn slice(&self, span: Span) -> &str {
        &self.code[span.start..=span.end]
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Source) -> bool {
        self.name == other.name
    }
}

impl Eq for Source {}

impl std::borrow::Borrow<str> for &Source {
    fn borrow(&self) -> &str {
        self.name.as_str()
    }
}

impl std::hash::Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub struct UserTypeDefinition<'a> {
    pub name: &'a str,
    pub fields: std::collections::HashMap<&'a str, (u32, Spanned<Type<'a>>)>,
}

impl<'a> std::fmt::Display for UserTypeDefinition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fields = self
            .fields
            .iter()
            .map(|(name, (_, Spanned { node, .. }))| format!("    {}: {}", name, node))
            .collect::<Vec<String>>()
            .join(",\n");

        write!(f, "type {} struct {{\n{}\n}}", self.name, fields)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition<'a> {
    name: &'a str,
    return_type: Spanned<Type<'a>>,
    parameters: Vec<Spanned<Type<'a>>>,
    varargs: bool,
}

impl<'a> FunctionDefinition<'a> {
    pub fn number_of_parameters_without_varargs(&self) -> usize {
        if self.varargs {
            if self.parameters.len() > 0 {
                return self.parameters.len() - 1;
            } else {
                return 0;
            }
        }

        self.parameters.len()
    }
}

impl<'a> Default for FunctionDefinition<'a> {
    fn default() -> Self {
        Self {
            name: "",
            return_type: Spanned::new(0, 0, Type::Simple(Simple::Void)),
            parameters: vec![],
            varargs: false,
        }
    }
}

pub type UserTypeMap<'a> = std::collections::HashMap<&'a str, UserTypeDefinition<'a>>;
pub type FunctionMap<'a> = std::collections::HashMap<&'a str, FunctionDefinition<'a>>;

fn find_errors(program: &Program) -> Vec<(Span, String)> {
    fn find_errors_recursive(statement: &Statement, errors: &mut Vec<(Span, String)>) {
        match statement {
            Statement::VariableDeclaration(declaration) => {
                if declaration.value.node.is_error() {
                    errors.push((declaration.value.span, declaration.value.node.to_string()))
                }
            }

            Statement::ExpressionStatement(Spanned { node: expression, span }) => {
                if expression.is_error() {
                    errors.push((*span, expression.to_string()));
                }
            }

            Statement::DeleteStatement(expression) => {
                if expression.node.is_error() {
                    errors.push((expression.span, expression.node.to_string()));
                }
            }

            Statement::ReturnStatement(expression) => {
                if let Some(Spanned { node: expression, span }) = expression {
                    if expression.is_error() {
                        errors.push((*span, expression.to_string()));
                    }
                }
            }

            Statement::WhileStatement(statement) => {
                let WhileStatement { condition: Spanned { node: condition, span, }, body, } = statement.as_ref();

                if condition.is_error() {
                    errors.push((*span, condition.to_string()));
                }

                for statement in &body.0 {
                    find_errors_recursive(statement, errors);
                }
            }

            Statement::IfStatement(statement) => {
                let IfStatement { condition: Spanned { node: condition, span, }, then_block, else_branch } = statement.as_ref();

                if condition.is_error() {
                    errors.push((*span, condition.to_string()));
                }

                for statement in &then_block.0 {
                    find_errors_recursive(statement, errors);
                }

                if let Some(else_branch) = else_branch {
                    match else_branch.as_ref() {
                        Else::IfStatement(statement) => find_errors_recursive(statement, errors),
                        Else::Block(block) => {
                            for statement in &block.0 {
                                find_errors_recursive(statement, errors);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut errors = vec![];
    for top_level in &program.0 {
        match top_level {
            TopLevel::FunctionDeclaration { body, .. } => {
                for statement in &body.0 {
                    find_errors_recursive(statement, &mut errors);
                }
            }

            TopLevel::Error { error } => {
                errors.push((error.span, error.node.to_string()));
            }

            TopLevel::TypeDeclaration { .. } | TopLevel::Import { .. } => {}
        }
    }

    errors
}

pub fn print_error<W: std::io::Write>(msg: &str, writer: &mut W) -> std::io::Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()?;

    Ok(())
}

pub fn report_errors<W: std::io::Write>(
    source: &Source,
    program: &Program,
    writer: &mut W,
) -> std::io::Result<()> {
    for (span, message) in find_errors(program) {
        print_error(&format_error(source, span, span, &message), writer)?;
    }

    Ok(())
}

pub fn format_error(
    source: &Source,
    expression_span: Span,
    error_token: Span,
    message: &str,
) -> String {
    let (line_number, index) = find_line_index(source, error_token.start);

    format!(
        "error: {}\n--> {}:{}:{}\n{}",
        message,
        source.name,
        line_number,
        index,
        error_to_string(source, expression_span, error_token, line_number, false)
    )
}

pub fn find_line_index(source: &Source, start: usize) -> (usize, usize) {
    let slice = &source.code[..start];
    let line_number = slice.chars().filter(|c| *c == '\n').count() + 1;
    let index = slice.chars().rev().take_while(|c| *c != '\n').count() + 1;

    (line_number, index)
}

fn find_distance(source: &Source, start: usize) -> usize {
    let slice = &source.code[..start];
    let slice = slice
        .chars()
        .rev()
        .take_while(|c| *c != '\n')
        .collect::<String>();
    let tabs = slice.chars().filter(|c| *c == '\t').count() * 4;

    UnicodeWidthStr::width(slice.as_str()) + tabs
}

pub fn error_to_string(
    source: &Source,
    expression_span: Span,
    error_token: Span,
    line_number: usize,
    warning: bool,
) -> String {
    let (starting_line, _) = find_line_index(source, expression_span.start);
    let (ending_line, _) = find_line_index(source, expression_span.end);
    let starting_line = starting_line - 1;
    let line_number_length = line_number.to_string().len();
    let filler = " ".repeat(line_number_length + 1);
    let slice = &source.code[error_token.start..error_token.end];
    let length = UnicodeWidthStr::width(slice) + 1;
    let distance = find_distance(source, error_token.start);
    let marker = format!("{}{}", " ".repeat(distance), "^".repeat(length));

    let marker = if warning {
        Yellow.paint(marker)
    } else {
        Red.paint(marker)
    };

    let lines: Vec<String> = source
        .code
        .lines()
        .enumerate()
        .skip(starting_line)
        .take(ending_line - starting_line)
        .map(|(n, l)| (n, l.replace("\t", "    ")))
        .map(|(n, l)| {
            if n + 1 == line_number {
                format!("{}|\n{} |{}\n{}|{}", filler, line_number, l, filler, marker)
            } else {
                format!("{}|{}", filler, l)
            }
        })
        .collect();

    lines.join("\n")
}

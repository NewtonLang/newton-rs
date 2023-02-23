use crate::error_to_string;
use crate::find_line_index;
use crate::format_error;
use crate::lexer::token::*;
use crate::types::types::*;
use crate::Source;
use crate::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct ResolverError<'a> {
    pub source: &'a Source,
    pub error: ResolveErrorType<'a>,
    pub error_span: Span,
    pub expression_span: Span,
}

impl<'a> ResolverError<'a> {
    fn error_token(&self) -> &'a str {
        &self.source.code[self.error_span.start..=self.error_span.end]
    }

    fn format_error(&self, message: &str) -> String {
        format_error(self.source, self.expression_span, self.error_span, message)
    }
}

impl<'a> std::fmt::Display for ResolverError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let binoperr = |error: &BinaryOperationError| {
            format!(
                "{} - not allowed",
                self.format_error(&format!(
                    "binary operation '{}' cannot be applied to '{}' and '{}'",
                    self.error_token(),
                    error.left_type,
                    error.right_type
                ))
            )
        };

        let result = match &self.error {
            ResolveErrorType::IllegalAssignment(error) => {
                let AssignmentError {
                    name,
                    definition_span,
                    ref binary_operator_error,
                } = error.as_ref();
                let (line_number, _) = find_line_index(self.source, definition_span.start);
                let span = *definition_span;
                let reason = format!(
                    "{} - '{}' was defined as '{}' here",
                    error_to_string(self.source, span, span, line_number, true),
                    name,
                    binary_operator_error.left_type
                );

                format!("{}\n\nreason:\n{}", binoperr(binary_operator_error), reason)
            }

            ResolveErrorType::NotDefined(DefinitionError { name }) => {
                self.format_error(&format!("'{}' is not defined in the current scope", name))
            }

            ResolveErrorType::IllegalOperation(ref error) => binoperr(error),

            ResolveErrorType::IllegalType(IllegalTypeError {
                expected_type,
                actual_type,
                name,
            }) => self.format_error(&format!(
                "{} must be of type '{}', but the actual type was '{}'",
                name, expected_type, actual_type
            )),

            ResolveErrorType::NoSuchField(StructFieldError {
                struct_name,
                field_name,
            }) => self.format_error(&format!(
                "'{}' has no field named '{}'",
                struct_name, field_name
            )),

            ResolveErrorType::SelfImport(_) => {
                self.format_error("cannot recursively import the current module")
            }

            ResolveErrorType::Inference(_) => self.format_error("type cannot be inferred"),

            ResolveErrorType::Dereference(NonPointerError(ty)) => {
                self.format_error(&format!("{} cannot be dereferenced", ty))
            }

            ResolveErrorType::Delete(NonPointerError(ty)) => self.format_error(&format!(
                "non-heap allocated pointer {} cannot be deleted",
                ty
            )),

            ResolveErrorType::NotArithmetic(ref error) => self.format_error(&format!(
                "cannot use operator '{}' on an expression of type '{}'",
                error.operator, error.ty
            )),

            ResolveErrorType::CallNonFunction(ref error) => self.format_error(&format!(
                "tried to call variable of type '{}', but ufcs is not yet supported",
                error.0
            )),
        };

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolveErrorType<'a> {
    IllegalAssignment(Box<AssignmentError<'a>>),
    NotDefined(DefinitionError<'a>),
    IllegalOperation(BinaryOperationError<'a>),
    IllegalType(IllegalTypeError<'a>),
    NoSuchField(StructFieldError<'a>),
    SelfImport(SelfImportError),
    Inference(TypeInferenceError),
    Dereference(NonPointerError<'a>),
    Delete(NonPointerError<'a>),
    NotArithmetic(ArithmeticError<'a>),
    CallNonFunction(NonFunctionError<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AssignmentError<'a> {
    pub name: &'a str,
    pub definition_span: Span,
    pub binary_operator_error: BinaryOperationError<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DefinitionError<'a> {
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryOperationError<'a> {
    pub left_type: Type<'a>,
    pub right_type: Type<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalTypeError<'a> {
    pub expected_type: Type<'a>,
    pub actual_type: Type<'a>,
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructFieldError<'a> {
    pub struct_name: &'a str,
    pub field_name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelfImportError;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeInferenceError;

#[derive(Debug, PartialEq, Eq)]
pub struct NonPointerError<'a>(pub Type<'a>);

#[derive(Debug, PartialEq, Eq)]
pub struct ArithmeticError<'a> {
    ty: Type<'a>,
    operator: TokenType<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NonFunctionError<'a>(pub Type<'a>);

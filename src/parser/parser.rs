use crate::Source;
use super::span::*;
use super::error::*;
use crate::ast::ast::*;
use crate::lexer::token::*;
use crate::types::types::*;
use crate::lexer::lexer::*;

type ParseResult<'a, T> = Result<T, Spanned<ParseError<'a>>>;

type TopLevelResult<'a> = ParseResult<'a, TopLevel<'a>>;
type StatementResult<'a> = ParseResult<'a, Statement<'a>>;
type ExpressionResult<'a> = ParseResult<'a, Spanned<Expression<'a>>>;

fn error_statement(error: Spanned<ParseError>) -> Statement {
    Statement::ExpressionStatement(Spanned::new(error.span.start, error.span.end, Expression::new(ExpressionKind::Error(error.node))))
}

pub struct Parser<'a, T> where T: Scanner<'static> {
    pub(crate) source: &'a Source<'a>,
    pub(crate) error_count: usize,

    scanner: std::iter::Peekable<T>,
}

impl<'a, T> Parser<'a, T> where T: Scanner<'a> {
    pub fn new(scanner: T) -> Parser<'a, T> {
        let source = scanner.source();
        let peekable = scanner.peekable();

        Self {
            source,
            error_count: 0,
            scanner: peekable,
        }
    }

    pub fn parse(&mut self) -> Program<'static> {
        let mut top_level_declarations = vec![];

        while self.scanner.peek().is_some() {
            let declaration = self.top_level_declaration();

            if let Ok(declaration) = declaration {
                top_level_declarations.push(declaration);
            } else if let Err(error) = declaration {
                top_level_declarations.push(TopLevel::Error { error });
                self.error_count += 1;

                while !(self.peek_equals(&TokenType::Fn) || self.peek_equals(&TokenType::Type) || self.at_end()) {
                    if let Err(error) = self.advance() {
                        panic!("error in {}: {:?}", self.source.name, error);
                    }
                }
            }
        }

        Program(top_level_declarations)
    }

    fn parse_expression(&mut self, precedence: Precedence, no_struct: bool) -> ExpressionResult<'static> {
        let token = self.advance()?;
        let mut left = self.prefix(&token, no_struct)?;

        while self.next_higher_precedence(precedence, no_struct) {
            let token = self.advance()?;
            left = self.infix(&token, left, no_struct)?;
        }

        Ok(left)
    }

    pub fn expression(&mut self, no_struct: bool) -> ExpressionResult<'static> {
        let mut left = self.parse_expression(Precedence::Assignment, no_struct)?;

        while self.peek_equals(&TokenType::Equals) {
            let eq = self.consume(TokenType::Equals)?;
            let value = Box::new(self.expression(no_struct)?);

            left = Spanned::new(left.span.start, value.span.end, Expression::new(ExpressionKind::Assignment { left: Box::new(left), eq, value }));
        }

        Ok(left)
    }

    fn statement(&mut self) -> StatementResult<'static> {
        if let Some(Ok(Spanned { node, .. })) = self.scanner.peek() {
            match node {
                TokenType::Let => {
                    let declaration = self.let_declaration()?;
                    self.consume(TokenType::Semicolon)?;

                    return Ok(declaration);
                },

                TokenType::If => return Ok(self.if_statement()?),
                TokenType::Return => return Ok(self.return_statement()?),
                TokenType::While => return Ok(self.while_statement()?),
                TokenType::Delete => return Ok(self.delete_statement()?),

                _ => {}
            }
        }

        let expression = self.expression(false)?;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::ExpressionStatement(expression ))
    }

    fn let_declaration(&mut self) -> StatementResult<'static> {
        self.consume(TokenType::Let)?;

        let name = self.consume_identifier()?;
        let ty = if let Ok(true) = self.match_token(TokenType::Colon) {
            Some(self.consume_type()?)
        } else {
            None
        };

        let ty = std::cell::RefCell::new(ty);
        let eq = self.consume(TokenType::Equals)?;
        let value = self.expression(false)?;

        Ok(Statement::VariableDeclaration(Box::new(VariableDeclaration { name, value, eq, ty })))
    }

    fn if_statement(&mut self) -> StatementResult<'static> {
        self.consume(TokenType::If)?;

        let condition = self.expression(true)?;
        let then_block = self.block()?;
        let else_branch = if self.peek_equals(&TokenType::Else) {
            self.consume(TokenType::Else)?;

            let else_branch = if self.peek_equals(&TokenType::If) {
                let else_if = Box::new(self.if_statement()?);
                Else::IfStatement(else_if)
            } else {
                let block = self.block()?;
                Else::Block(block)
            };

            Some(Box::new(else_branch))
        } else {
            None
        };

        Ok(Statement::IfStatement(Box::new(IfStatement { condition, then_block, else_branch })))
    }

    fn return_statement(&mut self) -> StatementResult<'static> {
        self.consume(TokenType::Return)?;

        let ret = Ok(Statement::ReturnStatement(if self.peek_equals(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression(false)?)
        }));

        self.consume(TokenType::Semicolon)?;

        ret
    }

    fn while_statement(&mut self) -> StatementResult<'static> {
        self.consume(TokenType::While)?;

        let condition = self.expression(true)?;
        let body = self.block()?;

        Ok(Statement::WhileStatement(Box::new(WhileStatement { condition, body })))
    }

    fn delete_statement(&mut self) -> StatementResult<'static> {
        self.consume(TokenType::Delete)?;
        let expression = self.expression(false)?;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::DeleteStatement(Box::new(expression)))
    }

    fn next_higher_precedence(&mut self, precedence: Precedence, no_struct: bool) -> bool {
        self.scanner.peek().map_or(false, | scanned | {
            if let Ok(spanned) = scanned {
                if let TokenType::LeftBrace = spanned.node {
                    return !no_struct && spanned.node.precedence() > precedence;
                }

                spanned.node.precedence() > precedence
            } else {
                false
            }
        })
    }



    fn top_level_declaration(&mut self) -> TopLevelResult<'static> {
        if self.peek_equals(&TokenType::Import) {
            return self.import_statement();
        }

        if self.peek_equals(&TokenType::Type) {
            return self.type_declaration_statement();
        }

        let is_external = self.peek_equals(&TokenType::Extern);
        if is_external {
            self.consume(TokenType::Extern)?;
        }

        self.consume(TokenType::Fn)?;

        let name = self.consume_identifier()?;
        let arguments = self.parameter_list(is_external)?;

        self.consume(TokenType::Colon)?;

        let return_type = self.consume_type()?;
        let body = if is_external {
            self.consume(TokenType::Semicolon)?;
            Block::default()
        } else {
            self.block()?
        };

        Ok(TopLevel::FunctionDeclaration { name, arguments, body, return_type, is_external })
    }

    fn import_statement(&mut self) -> TopLevelResult<'static> {
        self.consume(TokenType::Import)?;
        let name = self.consume_string()?;
        self.consume(TokenType::Semicolon)?;

        Ok(TopLevel::Import { name })
    }

    fn type_declaration_statement(&mut self) -> TopLevelResult<'static> {
        self.consume(TokenType::Type)?;

        let name = self.consume_identifier()?;

        self.consume(TokenType::Struct)?;
        self.consume(TokenType::LeftBrace)?;

        let mut fields = Vec::new();
        if !self.at_end() && !self.peek_equals(&TokenType::RightBrace) {
            loop {
                let field_name = self.consume_identifier()?;
                self.consume(TokenType::Colon)?;

                let field_type = self.consume_type()?;
                fields.push((field_name, field_type));

                if self.at_end() || self.peek_equals(&TokenType::RightBrace) {
                    break;
                }

                self.consume(TokenType::Comma)?;
            }
        }

        self.consume(TokenType::RightBrace)?;

        Ok(TopLevel::TypeDeclaration { ty: TypeDeclaration::StructDefinition { name, fields } })
    }

    fn parameter_list(&mut self, is_external: bool) -> ParseResult<'static, ParameterList<'static>> {
        self.consume(TokenType::LeftParen)?;

        let mut varargs = false;
        if self.match_token(TokenType::RightParen)? {
            return Ok(ParameterList::default());
        }

        let mut parameters = vec![];

        while !self.peek_equals(&TokenType::RightParen) {
            if self.peek_equals(&TokenType::Varargs) {
                if !is_external {
                    panic!("varargs are only supported in external functions");
                }

                let varargs_token = self.consume(TokenType::Varargs)?;
                varargs = true;

                let spanned = Spanned::new_from_span(varargs_token.span, "...");
                parameters.push(Parameter::new(spanned, Spanned::new_from_span(varargs_token.span, Type::Simple(Simple::VarArgs))));

                break;
            }

            let identifier = self.consume_identifier()?;
            self.consume(TokenType::Colon)?;

            let ty = self.consume_type()?;
            parameters.push(Parameter::new(identifier, ty));

            if !self.peek_equals(&TokenType::RightParen) {
                self.consume(TokenType::Comma)?;
            }
        }

        self.consume(TokenType::RightParen)?;

        Ok(ParameterList { varargs, parameters })
    }

    fn argument_list(&mut self) -> ParseResult<'static, ArgumentList<'static>> {
        let mut arguments = vec![];
        
        while !self.at_end() && !self.peek_equals(&TokenType::RightParen) {
            arguments.push(self.expression(false)?);

            if !self.peek_equals(&TokenType::RightParen) {
                self.consume(TokenType::Comma)?;
            }
        }

        Ok(ArgumentList(arguments))
    }

    fn initializer_list(&mut self) -> ParseResult<'static, InitializerList<'static>> {
        let mut inits = vec![];

        while !self.at_end() && !self.peek_equals(&TokenType::RightBrace) {
            let identifier = self.consume_identifier()?;
            self.consume(TokenType::Colon)?;

            let expression = self.expression(false)?;
            inits.push((identifier, expression));

            if !self.peek_equals(&TokenType::RightBrace) {
                self.consume(TokenType::Comma)?;
            }
        }

        Ok(InitializerList(inits))
    }

    fn block(&mut self) -> ParseResult<'static, Block<'static>> {
        self.consume(TokenType::LeftBrace)?;

        let mut statements = vec![];
        while !self.at_end() && !self.peek_equals(&TokenType::RightBrace) {
            let statement = self.statement();
            if let Ok(statement) = statement {
                statements.push(statement);
            } else if let Err(error) = statement {
                self.error_count += 1;
                self.sync();
                statements.push(error_statement(error));
            }
        }

        if !self.at_end() {
            self.consume(TokenType::RightBrace)?;
        }

        Ok(Block(statements))
    }

    fn consume_identifier(&mut self) -> ParseResult<'static, Spanned<&'static str>> {
        if let Some(peek) = self.scanner.peek().clone() {
            return match peek {
                Ok(peek) => {
                    if let Spanned { node: TokenType::Identifier(identifier), span } = peek {
                        
                        return Ok(Spanned::new_from_span(*span, identifier));
                    } else {
                        let token = Spanned::clone(&peek);
                        return Err(self.consume_error(&token, "identifier".to_owned()).unwrap_err());
                    }
                },

                Err(error) => Err(Spanned::new(0, 0, error.node.clone())),
            };
        }

        Err(self.eof().unwrap_err())
    }

    fn consume_string(&mut self) -> ParseResult<'static, Spanned<&'static str>> {
        if let Some(peek) = self.scanner.peek().cloned() {
            return match peek {
                Ok(peek) => {
                    if let Spanned { node: TokenType::StringLiteral(literal), span } = peek {
                        self.advance()?;
                        return Ok(Spanned::new_from_span(span, literal));
                    } else {
                        let token = Spanned::clone(&peek);
                        return Err(self.consume_error(&token, "string".to_owned()).unwrap_err());
                    }
                },

                Err(error) => Err(error),
            };
        }

        Err(self.eof().unwrap_err())
    }

    fn user_identifier(&self, expression: &Spanned<Expression<'static>>) -> ParseResult<'static, UserIdentifier<'static>> {
        Ok(match expression.node.kind() {
            ExpressionKind::Identifier(identifier) => UserIdentifier::new(&self.source.name, identifier),
            ExpressionKind::Access { left, identifier } => {
                if let ExpressionKind::Identifier(left) = left.node.kind() {
                    UserIdentifier::new(left, identifier.node)
                } else {
                    return Err(Spanned::new_from_span(left.span, ParseError::InternalError("the left side of this expression has to be an identifier")));
                }
            },

            _ => {
                return Err(Spanned::new_from_span(expression.span, ParseError::InternalError("the expression for a user identifier has to be an identifier or access expression")));
            }
        })
    }

    fn consume_type(&mut self) -> ParseResult<'static, Spanned<Type<'static>>> {
        if let Some(peek) = self.scanner.peek().cloned() {
            return match peek {
                Ok(peek) => {
                    match peek {
                        Spanned { node: TokenType::TypeIdentifier(ty), span } => {
                            self.advance()?;
                            Ok(Spanned::new_from_span(span, Type::Simple(ty)))
                        },

                        Spanned { node: TokenType::Identifier(_), .. } => {
                            let expression = self.parse_expression(Precedence::Assignment, true)?;
                            let identifier = self.user_identifier(&expression)?;

                            Ok(Spanned::new_from_span(expression.span, Type::Simple(Simple::UserDefinedType(identifier))))
                        },

                        Spanned { node: TokenType::Star, .. } => {
                            let mut counter = 1;
                            let start = self.advance()?.span.start;

                            while self.match_token(TokenType::Star)? {
                                counter += 1;
                            }

                            let ty = self.consume_type()?;
                            let (inner, end) = if let Type::Simple(s) = ty.node {
                                (s, ty.span.end)
                            } else {
                                return Err(Spanned::new_from_span(ty.span, ParseError::InternalError("reached unreachable code while attempting to parse a pointer type")));
                            };

                            Ok(Spanned::new(start, end, Type::Complex(Complex::Pointer(Pointer::new(inner, counter)))))
                        },

                        _ => {
                            let token = Spanned::clone(&peek);
                            Err(self.consume_error(&token, "type".to_owned()).unwrap_err())
                        }
                    }
                },

                Err(error) => Err(error)
            };
        }

        Err(self.eof().unwrap_err())
    }

    fn prefix(&mut self, token: &Spanned<TokenType<'static>>, no_struct: bool) -> ExpressionResult<'static> {
        let ok_spanned = | kind | Ok(Spanned::new_from_span(token.span, Expression::new(kind)));

        match token.node {
            TokenType::NullLiteral => ok_spanned(ExpressionKind::NullLiteral),
            TokenType::DecLiteral(literal) => ok_spanned(ExpressionKind::DecLiteral(literal)),
            TokenType::FloatLiteral(literal) => ok_spanned(ExpressionKind::FloatLiteral(literal)),
            TokenType::StringLiteral(literal) => ok_spanned(ExpressionKind::StringLiteral(literal)),
            TokenType::Char(literal) => ok_spanned(ExpressionKind::Char(literal)),

            TokenType::Sizeof => {
                let ty = self.consume_type()?;
                let sizeof = ExpressionKind::SizeOf(ty.node);

                Ok(Spanned::new_from_span(ty.span, Expression::new(sizeof)))
            },

            TokenType::New => {
                let expression = self.expression(no_struct)?;
                let new = ExpressionKind::New(Box::new(Spanned::new_from_span(expression.span, expression.node)));

                Ok(Spanned::new_from_span(expression.span, Expression::new(new)))
            },

            TokenType::LeftParen => {
                let mut expression = self.expression(false)?;

                self.consume(TokenType::RightParen)?;
                expression.span.start -= 1;
                expression.span.end += 1;

                Ok(expression)
            },

            TokenType::Minus => {
                let next = self.parse_expression(Precedence::Unary, no_struct)?;

                Ok(Spanned::new(token.span.start, next.span.end, Expression::new(ExpressionKind::Negate(token.clone(), Box::new(next)))))
            },

            TokenType::Ampersand => {
                let next = self.parse_expression(Precedence::Unary, no_struct)?;

                Ok(Spanned::new(token.span.start, next.span.end, Expression::new(ExpressionKind::Reference(token.clone(), Box::new(next)))))
            },

            TokenType::Star => {
                let next = self.parse_expression(Precedence::Unary, no_struct)?;

                Ok(Spanned::new(token.span.start, next.span.end, Expression::new(ExpressionKind::Dereference(token.clone(), Box::new(next)))))
            },

            TokenType::Bang => {
                let next = self.parse_expression(Precedence::Unary, no_struct)?;

                Ok(Spanned::new(token.span.start, next.span.end, Expression::new(ExpressionKind::BoolNegate(token.clone(), Box::new(next)))))
            },

            TokenType::Identifier(ref name) => {
                if !no_struct && self.match_token(TokenType::LeftBrace)? {
                    let init_list = self.initializer_list()?;
                    let brace = self.consume(TokenType::RightBrace)?;

                    Ok(Spanned::new(token.span.start, brace.span.end, Expression::new(ExpressionKind::StructInitialization { 
                        identifier: Spanned::new_from_span(token.span, UserIdentifier::new(&self.source.name, name)), 
                        fields: init_list 
                    })))
                } else {
                    ok_spanned(ExpressionKind::Identifier(name))
                }
            },

            _ => self.prefix_error(token),
        }
    }

    fn infix(&mut self, token: &Spanned<TokenType<'static>>, left: Spanned<Expression<'static>>, no_struct: bool) -> ExpressionResult<'static> {
        let tok = &token.node;

        match tok {
            TokenType::EqualsEquals | 
            TokenType::BangEquals | 
            TokenType::Smaller | 
            TokenType::SmallerEquals | 
            TokenType::Greater | 
            TokenType::GreaterEquals | 
            TokenType::AmpersandAmpersand | 
            TokenType::PipePipe | 
            TokenType::Plus | 
            TokenType::Minus | 
            TokenType::Star | 
            TokenType::Slash | 
            TokenType::Percent => {
                let right = self.parse_expression(tok.precedence(), no_struct)?;
                let right_span = right.span;
                let left_span = left.span;

                let expression = match tok {
                    TokenType::EqualsEquals |
                    TokenType::BangEquals |
                    TokenType::Smaller |
                    TokenType::SmallerEquals |
                    TokenType::Greater |
                    TokenType::GreaterEquals |
                    TokenType::AmpersandAmpersand |
                    TokenType::PipePipe => {
                        ExpressionKind::BoolBinary(Box::new(left), token.clone(), Box::new(right))
                    },

                    _ => {
                        ExpressionKind::Binary(Box::new(left), token.clone(), Box::new(Spanned::new_from_span(right.span, right.node)))
                    }
                };

                Ok(Spanned::new(left_span.start, right_span.end, Expression::new(expression)))
            },

            TokenType::As => {
                let expression = Box::new(left);
                let ty = self.consume_type()?;

                Ok(Spanned::new(expression.span.start, ty.span.end, Expression::new(ExpressionKind::Cast(expression, token.clone(), ty))))
            },

            TokenType::Dot => {
                let identifier = self.consume_identifier()?;
                Ok(Spanned::new(left.span.start, identifier.span.end, Expression::new(ExpressionKind::Access { left: Box::new(left), identifier })))
            },

            TokenType::LeftBrace => {
                let initializer_list = self.initializer_list()?;
                let brace = self.consume(TokenType::RightBrace)?;
                let span = Span::new(token.span.start, brace.span.end);
                let identifier = self.user_identifier(&left)?;

                Ok(Spanned::new_from_span(span, Expression::new(ExpressionKind::StructInitialization { identifier: Spanned::new_from_span(left.span, identifier), fields: initializer_list })))
            },

            TokenType::LeftParen => {
                let argument_list = self.argument_list()?;
                let end = self.consume(TokenType::RightParen)?.span.end;
                let span = Span::new(left.span.start, end);
                let (module, callee) = self.get_info_about_callee(left);

                Ok(Spanned::new_from_span(span, Expression::new(ExpressionKind::Call { module, callee: Box::new(callee), arguments: argument_list })))
            },

            _ => self.infix_error(token),
        }
    }

    fn get_info_about_callee(&self, expression: Spanned<Expression<'static>>) -> (&'static str, Spanned<Expression<'static>>) {
        if let ExpressionKind::Access { left, identifier } = expression.node.kind() {
            if let ExpressionKind::Identifier(module) = left.node.kind() {
                return (module, Spanned::new_from_span(identifier.span, Expression::new(ExpressionKind::Identifier(identifier.node))));
            }
        }

        (self.source.name, expression)
    }

    fn eof(&mut self) -> Scanned<'static> {
        let length = self.source.code.len();
        let span = Span::new(length, length);

        self.lexer_error(span, "unexpected eof")
    }

    fn advance(&mut self) -> Scanned<'static> {
        self.scanner.next().unwrap_or_else(|| self.eof())
    }

    fn match_token(&mut self, expected: TokenType<'static>) -> ParseResult<'static, bool> {
        if self.peek_equals(&expected) {
            self.consume(expected)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn peek_equals(&mut self, expected: &TokenType<'static>) -> bool {
        self.scanner.peek().map_or(false, | peek | match peek {
            Ok(Spanned { node, .. }) => *node == *expected,

            _ => false,
        })
    }

    fn consume(&mut self, expected: TokenType<'static>) -> Scanned<'static> {
        if let Some(peek) = self.scanner.peek() {
            if let Ok(peek) = peek {
                if peek.node == expected {
                    let next = self.advance()?;
                    return Ok(next);
                } else {
                    let token = Spanned::clone(peek);
                    return self.consume_error(&token, expected.to_string());
                }
            } else {
                return peek.clone();
            }
        }

        self.eof()
    }

    fn lexer_error(&mut self, span: Span, cause: &'static str) -> Scanned<'static> {
        self.error_count += 1;

        Err(Spanned { span, node: ParseError::LexingError(LexingError::with_cause(cause)) })
    }

    fn prefix_error(&mut self, token: &Spanned<TokenType<'static>>) -> ExpressionResult<'static> {
        self.error_count += 1;

        let s = format!("invalid token in prefix expression '{}'", token.node);
        Err(Spanned { span: token.span, node: ParseError::PrefixError(s) })
    }

    fn infix_error(&mut self, token: &Spanned<TokenType<'static>>) -> ExpressionResult<'static> {
        self.error_count += 1;

        let s = format!("invalid token in infix expression '{}'", token.node);
        Err(Spanned { span: token.span, node: ParseError::InfixError(s) })
    }

    fn consume_error(&mut self, actual: &Spanned<TokenType<'static>>, expected: String) -> Scanned<'static> {
        self.error_count += 1;

        Err(Spanned { span: actual.span, node: ParseError::ConsumeError { actual: actual.node.clone(), expected } })
    }

    fn sync(&mut self) {
        let mut previous = self.advance();

        while let Some(Ok(peek)) = self.scanner.peek() {
            if let Ok(Spanned { node: TokenType::Semicolon, .. }) = previous {
                break;
            }

            match peek.node {
                TokenType::Type | TokenType::Fn | TokenType::If | TokenType::Let | TokenType::Return => return,

                _ => {},
            }

            previous = self.advance();
        }
    }

    fn at_end(&mut self) -> bool {
        self.scanner.peek().is_none()
    }
}
use crate::types::types::*;
use crate::parser::span::*;
use crate::lexer::token::*;
use crate::parser::error::*;

#[derive(Debug, PartialEq)]
pub enum ExpressionKind<'a> {
    Error(ParseError<'a>),
    NullLiteral,
    DecLiteral(&'static str),
    FloatLiteral(&'static str),
    StringLiteral(&'static str),
    Char(&'static str),
    Reference(Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    Dereference(Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    Negate(Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    BoolNegate(Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    Binary(Box<Spanned<Expression<'a>>>, Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    BoolBinary(Box<Spanned<Expression<'a>>>, Spanned<TokenType<'a>>, Box<Spanned<Expression<'a>>>),
    Cast(Box<Spanned<Expression<'a>>>, Spanned<TokenType<'a>>, Spanned<Type<'a>>),
    Identifier(&'static str),
    New(Box<Spanned<Expression<'a>>>),
    SizeOf(Type<'a>),

    Assignment {
        left: Box<Spanned<Expression<'a>>>,
        eq: Spanned<TokenType<'a>>,
        value: Box<Spanned<Expression<'a>>>,
    },

    Call {
        module: &'static str,
        callee: Box<Spanned<Expression<'a>>>,
        arguments: ArgumentList<'a>,
    },

    Access {
        left: Box<Spanned<Expression<'a>>>,
        identifier: Spanned<&'static str>,
    },

    StructInitialization {
        identifier: Spanned<UserIdentifier<'a>>,
        fields: InitializerList<'a>,
    },
}

#[derive(Debug)]
pub struct Expression<'a> {
    ty: std::cell::RefCell<Option<Type<'a>>>,
    kind: ExpressionKind<'a>,
}

impl<'a> Expression<'a> {
    pub fn new(kind: ExpressionKind<'a>) -> Self {
        Self {
            ty: std::cell::RefCell::new(None),
            kind,
        }
    }

    pub fn new_with_ty(ty: Type<'a>, kind: ExpressionKind<'a>) -> Self {
        Self {
            ty: std::cell::RefCell::new(Some(ty)),
            kind,
        }
    }

    pub fn is_error(&self) -> bool {
        if let ExpressionKind::Error(..) = self.kind {
            return true;
        }

        false
    }

    #[inline]
    pub fn ty(&self) -> std::cell::Ref<Option<Type>> {
        self.ty.borrow()
    }

    #[inline]
    pub fn clone_ty(&self) -> Option<Type> {
        self.ty.borrow().clone()
    }

    #[inline]
    pub fn kind(&self) -> &ExpressionKind<'a> {
        &self.kind
    }

    pub fn set_ty(&self, ty: Type<'a>) {
        self.ty.replace(Some(ty));
    }

    pub fn sub_expressions(&mut self) -> Vec<&Spanned<Expression<'a>>> {
        match self.kind() {
            ExpressionKind::Error(_) => panic!(),
            ExpressionKind::NullLiteral | ExpressionKind::DecLiteral(_) | ExpressionKind::FloatLiteral(_) | ExpressionKind::StringLiteral(_) | ExpressionKind::Char(_) | ExpressionKind::SizeOf(_) | ExpressionKind::Identifier(_) => vec![],
            ExpressionKind::New(expr) | ExpressionKind::Negate(_, expr) | ExpressionKind::BoolNegate(_, expr) | ExpressionKind::Reference(_, expr) | ExpressionKind::Dereference(_, expr) => vec![ &expr ],
            ExpressionKind::Binary(left, _, right) => vec![ &left, &right ],
            ExpressionKind::BoolBinary(left, _, right) => vec![ &left, &right ],
            ExpressionKind::Cast(e, _, _) => vec![ &e ],

            ExpressionKind::Assignment { left, value, .. } => vec![ &left, &value ],

            ExpressionKind::Call { arguments, callee, .. } => {
                if let ExpressionKind::Identifier(_) = callee.node.kind {
                    arguments.0.iter().collect()
                } else {
                    let mut vec = Vec::with_capacity(arguments.0.len() + 1);

                    vec.push(callee.as_ref());

                    for argument in &arguments.0 {
                        vec.push(argument);
                    }

                    vec
                }
            },

            ExpressionKind::Access { left, .. } => vec![ &left ],

            ExpressionKind::StructInitialization { fields, .. } => fields.0.iter().map(| (_, e) | e).collect(),
        }
    }

    pub fn is_r_value(&mut self) -> bool {
        match self.kind {
            ExpressionKind::Identifier(_) | ExpressionKind::Access { .. } => false,
            _ => true,
        }
    }

    #[inline]
    pub fn is_l_value(&mut self) -> bool {
        !self.is_r_value()
    }
}

impl<'a> PartialEq for Expression<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl<'a> std::fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Error(err) => write!(f, "{err}"),
            ExpressionKind::NullLiteral => write!(f, "null"),
            ExpressionKind::DecLiteral(lit) | ExpressionKind::FloatLiteral(lit) | ExpressionKind::StringLiteral(lit) | ExpressionKind::Char(lit) => write!(f, "{lit}"),
            ExpressionKind::SizeOf(ty) => write!(f, "sizeof({ty})"),
            ExpressionKind::New(expr) => write!(f, "new {}", expr.node),
            ExpressionKind::Negate(_, expr) | ExpressionKind::BoolNegate(_, expr) => write!(f, "-{}", expr.node),
            ExpressionKind::Reference(_, expr) => write!(f, "&{}", expr.node),
            ExpressionKind::Dereference(_, expr) => write!(f, "*{}", expr.node),
            ExpressionKind::Binary(l, op, r) | ExpressionKind::BoolBinary(l, op, r) => write!(f, "{} {} {}", l.node, op.node, r.node),
            ExpressionKind::Identifier(name) => write!(f, "{name}"),
            ExpressionKind::Cast(expr, _, ty) => write!(f, "{} as {}", expr.node, ty.node),
            ExpressionKind::Assignment { left, value, .. } => write!(f, "{} = {}", left.node, value.node),
            ExpressionKind::Call { callee, arguments, .. } => write!(f, "{}({})", callee.node, arguments),
            ExpressionKind::Access { left, identifier } => write!(f, "{}.{}", left.node, identifier.node),
            ExpressionKind::StructInitialization { identifier, fields } => write!(f, "{} {{ {} }}", identifier.node, fields.0.iter().map(| (n, e) | format!("{}: {}", n.node, e.node)).collect::<Vec<String>>().join(",\n"))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program<'a> (pub Vec<TopLevel<'a>>);

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Block<'a> (pub Vec<Statement<'a>>);

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter<'a> (pub Spanned<&'a str>, pub Spanned<Type<'a>>);

impl<'a> Parameter<'a> {
    pub fn new(identifier: Spanned<&'a str>, ty: Spanned<Type<'a>>) -> Self {
        Self(identifier, ty)
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ParameterList<'a> {
    pub varargs: bool,
    pub parameters: Vec<Parameter<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ArgumentList<'a> (pub Vec<Spanned<Expression<'a>>>);

impl<'a> std::fmt::Display for ArgumentList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let strings: Vec<String> = self
            .0
            .iter()
            .map(| Spanned { node, .. } | node.to_string())
            .collect();
        write!(f, "{}", strings.join(", "))
    }
}

#[derive(Debug, PartialEq)]
pub struct InitializerList<'a> (pub Vec<(Spanned<&'static str>, Spanned<Expression<'a>>)>);

#[derive(Debug, PartialEq, Eq)]
pub enum TopLevel<'a> {
    FunctionDeclaration {
        name: Spanned<&'a str>,
        arguments: ParameterList<'a>,
        body: Block<'a>,
        return_type: Spanned<Type<'a>>,
        is_external: bool,
    },

    Import {
        name: Spanned<&'static str>,
    },

    TypeDeclaration {
        ty: TypeDeclaration<'a>,
    },

    Error {
        error: Spanned<ParseError<'a>>,
    }
}

impl<'a> std::fmt::Display for TopLevel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FunctionDeclaration { 
                name, 
                arguments, 
                body: _, 
                return_type, 
                is_external } => {
                    let mut external = String::new();

                    if *is_external {
                        external.push_str("extern");
                    }

                    let name = format!("{}", name.node);
                    let return_type = format!("{}", return_type.node);
                    let mut args = Vec::new();

                    for argument in &arguments.parameters {
                        args.push(format!("{}: {}", argument.0.node, argument.1.node));
                    }

                    let mut signature = String::new();

                    if *is_external {
                        signature.push_str(&format!("extern {} {}({});\n", return_type, name, args.join(", ")));
                    } else {
                        signature.push_str(&format!("{} {} ({}) {{\n", return_type, name, args.join(", ")));
                    }

                    write!(f, "{}", signature)
                },

            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDeclaration<'a> {
    StructDefinition {
        name: Spanned<&'static str>,
        fields: Vec<(Spanned<&'static str>, Spanned<Type<'a>>)>,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    VariableDeclaration(Box<VariableDeclaration<'a>>),
    IfStatement(Box<IfStatement<'a>>),
    WhileStatement(Box<WhileStatement<'a>>),
    ReturnStatement(Option<Spanned<Expression<'a>>>),
    DeleteStatement(Box<Spanned<Expression<'a>>>),
    ExpressionStatement(Spanned<Expression<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclaration<'a> {
    pub name: Spanned<&'a str>,
    pub value: Spanned<Expression<'a>>,
    pub eq: Spanned<TokenType<'a>>,
    pub ty: std::cell::RefCell<Option<Spanned<Type<'a>>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement<'a> {
    pub condition: Spanned<Expression<'a>>,
    pub then_block: Block<'a>,
    pub else_branch: Option<Box<Else<'a>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Else<'a> {
    IfStatement(Box<Statement<'a>>),
    Block(Block<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement<'a> {
    pub condition: Spanned<Expression<'a>>,
    pub body: Block<'a>,
}
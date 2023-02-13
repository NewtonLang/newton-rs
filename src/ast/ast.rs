use crate::types::types::*;
use crate::parser::span::*;
use crate::lexer::token::*;
use crate::parser::error::*;

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Error(ParseError),
    NullLiteral,
    DecLiteral(&'static str),
    FloatLiteral(&'static str),
    StringLiteral(&'static str),
    Char(&'static str),
    Reference(Spanned<Token>, Box<Spanned<Expression>>),
    Dereference(Spanned<Token>, Box<Spanned<Expression>>),
    Negate(Spanned<Token>, Box<Spanned<Expression>>),
    BoolNegate(Spanned<Token>, Box<Spanned<Expression>>),
    Binary(Box<Spanned<Expression>>, Spanned<Token>, Box<Spanned<Expression>>),
    BoolBinary(Box<Spanned<Expression>>, Spanned<Token>, Box<Spanned<Expression>>),
    Cast(Box<Spanned<Expression>>, Spanned<Token>, Spanned<Type>),
    Identifier(&'static str),
    New(Box<Spanned<Expression>>),
    SizeOf(Type),

    Assignment {
        left: Box<Spanned<Expression>>,
        eq: Spanned<Token>,
        value: Box<Spanned<Expression>>,
    },

    Call {
        module: &'static str,
        callee: Box<Spanned<Expression>>,
        arguments: ArgumentList,
    },

    Access {
        left: Box<Spanned<Expression>>,
        identifier: Spanned<&'static str>,
    },

    StructInitialization {
        identifier: Spanned<UserIdentifier>,
        fields: InitializerList,
    },
}

#[derive(Debug)]
pub struct Expression {
    ty: std::cell::RefCell<Option<Type>>,
    kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self {
            ty: std::cell::RefCell::new(None),
            kind,
        }
    }

    pub fn is_error(&mut self) -> bool {
        if let ExpressionKind::Error(..) = self.kind {
            return true;
        }

        false
    }

    #[inline]
    pub fn ty(&mut self) -> std::cell::Ref<Option<Type>> {
        self.ty.borrow()
    }

    #[inline]
    pub fn clone_ty(&mut self) -> Option<Type> {
        self.ty.borrow().clone()
    }

    #[inline]
    pub fn kind(&mut self) -> &ExpressionKind {
        &self.kind
    }

    pub fn set_ty(&mut self, ty: Type) {
        self.ty.replace(Some(ty));
    }

    pub fn sub_expressions(&mut self) -> Vec<&Spanned<Expression>> {
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

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl std::fmt::Display for Expression {
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
pub struct Program(Vec<TopLevel>);

#[derive(Debug, PartialEq, Eq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter(pub Spanned<&'static str>, pub Spanned<Type>);

impl Parameter {
    pub fn new(identifier: Spanned<&'static str>, ty: Spanned<Type>) -> Self {
        Parameter(identifier, ty)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArgumentList(pub Vec<Spanned<Expression>>);

impl std::fmt::Display for ArgumentList {
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
pub struct InitializerList(pub Vec<(Spanned<&'static str>, Spanned<Expression>)>);

#[derive(Debug, PartialEq, Eq)]
pub enum TopLevel {
    FunctionDeclaration {
        name: Spanned<&'static str>,
        arguments: ArgumentList,
        body: Block,
        return_type: Spanned<Type>,
        is_external: bool,
    },

    Import {
        name: Spanned<&'static str>,
    },

    TypeDeclaration {
        ty: TypeDeclaration,
    },

    Error {
        error: Spanned<ParseError>,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeDeclaration {
    StructDefinition {
        name: Spanned<&'static str>,
        fields: Vec<(Spanned<&'static str>, Spanned<Type>)>,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    VariableDeclaration(Box<VariableDeclaration>),
    IfStatement(Box<IfStatement>),
    WhileStatement(Box<WhileStatement>),
    ReturnStatement(Option<Spanned<Expression>>),
    DeleteStatement(Box<Spanned<Expression>>),
    ExpressionStatement(Spanned<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclaration {
    pub name: Spanned<&'static str>,
    pub value: Spanned<Expression>,
    pub eq: Spanned<Token>,
    pub ty: std::cell::RefCell<Option<Spanned<Type>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Spanned<Expression>,
    pub then_block: Block,
    pub else_branch: Option<Box<Else>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Else {
    IfStatement(Box<Statement>),
    Block(Block),
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement {
    pub condition: Spanned<Expression>,
    pub body: Block,
}
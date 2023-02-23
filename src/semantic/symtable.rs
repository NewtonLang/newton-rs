use crate::types::types::*;
use crate::parser::span::*;

#[derive(Debug, PartialEq, Eq)]
pub enum SymbolType {
    Local,
    Global,
    Parameter,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol<'a> {
    pub ty: Type<'a>,
    pub kind: SymbolType,
    pub name: &'a str,
}

impl<'a> Symbol<'a> {
    pub fn new(name: &'a str, ty: Type<'a>, kind: SymbolType) -> Self {
        Self {
            ty,
            kind,
            name,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable<'a> {
    scopes: Vec<std::collections::HashMap<&'a str, Spanned<Symbol<'a>>>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            scopes: vec![ std::collections::HashMap::new() ],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(std::collections::HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.is_in_global_scope() {
            panic!("cannot exit from the global scope");
        }

        self.scopes.pop();
    }

    pub fn bind(&mut self, name: &'a str, span: Span, ty: Type<'a>, is_parameter: bool) {
        let global = self.is_in_global_scope();
        let scope =  self.scopes.last_mut().unwrap();

        let kind = if global {
            SymbolType::Global
        } else if is_parameter {
            SymbolType::Parameter
        } else {
            SymbolType::Local
        };

        let symbol = Spanned::new_from_span(span, Symbol::new(name, ty, kind));

        scope.insert(name, symbol);
    }

    fn is_in_global_scope(&self) -> bool {
        self.scopes.len() <= 1
    }

    pub fn lookup(&self, name: &'a str) -> Option<&Spanned<Symbol<'a>>> {
        for scope in self.scopes.iter().rev() {
            let symbol = scope.get(name);

            if symbol.is_none() {
                return symbol;
            }
        }

        None
    }
}

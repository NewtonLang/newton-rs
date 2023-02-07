use crate::types::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOrArgument {
    name: String,
    ttype: Type,
}

impl FieldOrArgument {
    pub fn new(name: String, ttype: Type) -> Self {
        Self {
            name,
            ttype,
        }
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn ttype(&self) -> &Type {
        &self.ttype
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    arguments: Vec<FieldOrArgument>,
    return_type: Type,
    body: Vec<()>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<FieldOrArgument>, return_type: Type, body: Vec<()>) -> Self {
        Self {
            name,
            arguments,
            return_type,
            body,
        }
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn arguments(&self) -> &Vec<FieldOrArgument> {
        &self.arguments
    }

    #[inline]
    pub fn get_argument(&self, name: String) -> &FieldOrArgument {
        self.arguments
            .iter()
            .enumerate()
            .find(|&arg| arg.1.name.to_string() == name.to_string())
            .unwrap()
            .1
    }

    #[inline]
    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    #[inline]
    pub fn body(&self) -> &Vec<()> {
        &self.body
    }
}
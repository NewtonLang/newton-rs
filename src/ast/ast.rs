use std::borrow::*;

use crate::{ types::types::*, lexer::token::* };

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOrArgument {
    name: String,
    ttype: Type,
}

impl FieldOrArgument {
    pub fn new(name: &'static str, ttype: Type) -> Self {
        Self {
            name: name.to_owned(),
            ttype,
        }
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[inline]
    pub fn ttype(&self) -> &Type {
        &self.ttype
    }
}

impl std::fmt::Display for FieldOrArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ttype)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: &'static str,
    arguments: std::collections::BTreeMap<&'static str, FieldOrArgument>,
    return_type: Type,
    body: Vec<()>,
}

impl Function {
    pub fn new(name: &'static str, arguments: std::collections::BTreeMap<&'static str, FieldOrArgument>, return_type: Type, body: Vec<()>) -> Self {
        Self {
            name,
            arguments,
            return_type,
            body,
        }
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    #[inline]
    pub fn arguments(&self) -> &std::collections::BTreeMap<&'static str, FieldOrArgument> {
        &self.arguments
    }

    #[inline]
    pub fn get_argument(&self, name: &'static str) -> &FieldOrArgument {
        self.arguments.get(name).unwrap()
    }

    #[inline]
    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    #[inline]
    pub fn body(&self) -> &Vec<()> {
        &self.body
    }

    #[inline]
    pub fn check_if_signature_is(&self, signature: &std::collections::BTreeMap<&'static str, FieldOrArgument>) -> bool {
        if self.arguments == *signature {
            return true;
        } 

        false
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from(format!("fn {}(", self.name));
        let mut args: Vec<String> = vec![];

        for (_, v) in &self.arguments {
            args.push(format!("{}: {}", v.name, v.ttype));
        }

        res.push_str(&args.join(", "));
        res.push_str(&format!(") => {} {{}}", self.return_type));

        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    name: String,
    mangled_c_name: String,
    is_generic: bool,
    generic_params: std::collections::BTreeSet<Token>,
    fields: std::collections::BTreeMap<&'static str, FieldOrArgument>,
    methods: std::collections::BTreeMap<&'static str, Function>,
}

impl Struct {
    // big ass signature for the constructor
    pub fn new(
        name: &'static str, 
        is_generic: bool, 
        generic_params: std::collections::BTreeSet<Token>, 
        fields: std::collections::BTreeMap<&'static str, FieldOrArgument>, 
        methods: std::collections::BTreeMap<&'static str, Function>
    ) -> Self {

        
        let mut mangled_c_name = String::from(name);

        if is_generic {
            generic_params.iter().for_each(|param| {
                mangled_c_name.push_str(&format!("_{}", param));
            });
        }

        Self {
            name: name.to_owned(),
            mangled_c_name,
            is_generic,
            generic_params,
            fields,
            methods
        }
    }

    #[inline]
    pub fn name(&mut self) -> &String {
        &self.name
    }

    #[inline]
    pub fn mangled_c_name(&mut self) -> &String {
        &self.mangled_c_name
    }

    #[inline]
    pub fn is_generic(&mut self) -> bool {
        self.is_generic
    }

    #[inline]
    pub fn generic_params(&mut self) -> &std::collections::BTreeSet<Token> {
        &self.generic_params
    }

    // this method might be useless, i'll keep it in the mean time until a real usage scenario pops up
    #[inline]
    pub fn get_generic_param(&mut self, param: &Token) -> &Token {
        self.generic_params.get(param).unwrap()
    }

    #[inline]
    pub fn fields(&mut self) -> &std::collections::BTreeMap<&'static str, FieldOrArgument> {
        &self.fields
    }

    #[inline]
    pub fn get_field(&mut self, field: &'static str) -> &FieldOrArgument {
        self.fields.get(field).unwrap()
    }

    #[inline]
    pub fn methods(&mut self) -> &std::collections::BTreeMap<&'static str, Function> {
        &self.methods
    }

    #[inline]
    pub fn get_method(&mut self, method: &'static str) -> &Function {
        self.methods.get(method).unwrap()
    } 
}

impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::from(format!("struct {} {{\n", self.name));

        for (_, v) in &self.fields {
            res.push_str(&format!("\t{}: {};\n", v.name, v.ttype));
        }

        for (_, v) in &self.methods {
            res.push_str(&format!("\t{}\n", v));
        }

        res.push('}');

        write!(f, "{}", res)
    }
}
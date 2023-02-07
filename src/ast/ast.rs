use crate::types::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOrArgument {
    name: &'static str,
    ttype: Type,
}

impl FieldOrArgument {
    pub fn new(name: &'static str, ttype: Type) -> Self {
        Self {
            name,
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
    pub fn name(&self) -> &'static str {
        self.name
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
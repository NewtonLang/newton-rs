use super::super::api::*;

/*
 * Newton's C backend. This is one of the backends originally included in the project.
 *
 * This can also be used as an example for new backend implementations.
 *
 * Newton (C) 2023
 */

#[derive(Debug)]
pub struct C {
    // Fields needed to provide `Backend` all the information about the new backend.
    pub name: String,
    pub description: String,
    pub author: String,
    pub target: String,

    pub source: String,
}

impl C {
    pub fn new() -> Self {
        Self {
            name: "Newton C backend".to_owned(),
            description:
                "The official C backend for Newton. Created by the Newton project creators"
                    .to_owned(),
            author: "Newton Team".to_owned(),
            target: "C".to_owned(),

            source: String::new(),
        }
    }
}

impl Backend for C {
    fn backend_name(&self) -> &String {
        &self.name
    }

    fn backend_description(&self) -> &String {
        &self.description
    }

    fn backend_author(&self) -> &String {
        &self.author
    }

    fn backend_target(&self) -> &String {
        &self.target
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn emit(&mut self, code: &str) -> () {
        self.source.push_str(&code.to_owned());
    }

    fn generate_header(&mut self) -> () {
        self.emit("// This code has been generated by Newton's official C backend.");
    }
}

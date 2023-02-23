use crate::types::types::*;
use crate::{FunctionDefinition, FunctionMap, UserTypeDefinition, UserTypeMap};

#[derive(Debug, Default)]
struct Module<'a> {
    user_types: UserTypeMap<'a>,
    functions: FunctionMap<'a>,
}

type ModuleName<'a> = &'a str;

#[derive(Debug, Default)]
pub struct ModuleMap<'a> {
    modules: std::collections::HashMap<ModuleName<'a>, Module<'a>>,
}

impl<'a> ModuleMap<'a> {
    pub fn create(&mut self, module: ModuleName<'a>) {
        self.modules.insert(module, Module::default());
    }

    pub fn define_function(
        &mut self,
        module: ModuleName<'a>,
        name: &'a str,
        definition: FunctionDefinition<'a>,
    ) {
        self.modules
            .entry(module)
            .or_insert_with(Module::default)
            .functions
            .insert(name, definition);
    }

    pub fn define_type(
        &mut self,
        module: ModuleName<'a>,
        name: &'a str,
        definition: UserTypeDefinition<'a>,
    ) {
        self.modules
            .entry(module)
            .or_insert_with(Module::default)
            .user_types
            .insert(name, definition);
    }

    fn and_then<'b, T, F>(&'b self, module: ModuleName, f: F) -> Option<T>
    where
        F: FnOnce(&'b Module<'a>) -> Option<T>,
    {
        self.modules.get(module).and_then(f)
    }

    pub fn function_defined(&self, module: ModuleName, name: &str) -> bool {
        self.and_then(module, |m| Some(m.functions.contains_key(name)))
            .unwrap_or(false)
    }

    pub fn type_defined(&self, module: ModuleName, identifier: &mut UserIdentifier) -> bool {
        self.and_then(module, |m| {
            Some(m.user_types.contains_key(identifier.name()))
        })
        .unwrap_or(false)
    }

    pub fn get_function(&self, module: ModuleName, name: &str) -> Option<&FunctionDefinition<'a>> {
        self.and_then(module, |m| m.functions.get(name))
    }

    pub fn get_user_type(&self, module: ModuleName, name: &str) -> Option<&UserTypeDefinition<'a>> {
        self.and_then(module, |m| m.user_types.get(name))
    }

    pub fn iter_functions<'b>(
        &'b self,
    ) -> impl Iterator<Item = (ModuleName<'a>, &'b FunctionDefinition<'a>)> {
        self.modules
            .iter()
            .flat_map(|(module_name, m)| m.functions.iter().map(move |(_, t)| (*module_name, t)))
    }

    pub fn iter_types<'b>(
        &'b self,
    ) -> impl Iterator<Item = (ModuleName<'a>, &'b UserTypeDefinition<'a>)> {
        self.modules
            .iter()
            .flat_map(|(module_name, m)| m.user_types.iter().map(move |(_, t)| (*module_name, t)))
    }

    pub fn move_iter_types(self) -> impl Iterator<Item = (ModuleName<'a>, UserTypeDefinition<'a>)> {
        self.modules.into_iter().flat_map(move |(module_name, m)| {
            m.user_types.into_iter().map(move |(_, t)| (module_name, t))
        })
    }
}

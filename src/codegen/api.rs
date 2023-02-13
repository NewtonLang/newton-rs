/*
 * An API to register new backends. In theory, it should allow anyone to add a new target *rather* easily.
 * 
 * This is also the file with the most comments in the whole codebase so far. The `Internal Compiler Design for Newton` 
 * document provides information and examples about the usage of this API.
 * 
 * Newton (C) 2023
 */

/*
 * `BackendInfo` trait. Any new backend needs to derive this trait, and provide the necessary fields for the trait to work properly.
 */
pub trait BackendInfo {
    // Return the name for the backend.
    fn backend_name(&mut self) -> &String;

    // Return the description for the backend. This is up to the author(s) of the backend.
    fn backend_description(&mut self) -> &String;

    // The name(s) of the author(s).
    fn backend_author(&mut self) -> &String;

    // The target of the backend. Could be C, could be some IR, could be JavaScript (if you're a maniac).
    fn backend_target(&mut self) -> &String;
}

/*
 * `BackendMethods` trait. Any new backend needs to implement this trait and add all the methods in here, which just serve as a base
 * to provide the methods needed for any new backend to arbitrarily generate code for the new backend.
 */
pub trait BackendMethods {
    // The emit method. Supposedly, it should feed generated source code to a `source` field on the backend's struct.
    fn emit(&mut self, code: &'static str) -> ();

    // Arbitrary header. Could be info about the backend, could be anything else the author wants.
    fn generate_header(&mut self) -> ();
}

/*
 * This struct is where all the magic happens. You simply `::register()` the new backend, and in *theory* it should all work just fine.
 * Subject to change; if you are the maintainer of a backend, make sure to always check this file with every Newton update to see if
 * there's any changes to the backend API.
 */

pub struct Backend<T: BackendMethods> {
    pub backends: std::collections::HashMap<String, T>,
}

impl<T: BackendMethods> Backend<T> {
    pub fn new() -> Self {
        Self {
            backends: std::collections::HashMap::new(),
        }
    }

    pub fn register_backend(&mut self, name: &'static str, backend: T) -> () {
        self.backends.insert(name.to_owned(), backend);
    }

    pub fn get_backend(&mut self, name: &'static str) -> &T {
        match self.backends.get(&name.to_owned()) {
            Some(v) => v,
            None => panic!("could not find backend `{}`", name),
        }
    }
}
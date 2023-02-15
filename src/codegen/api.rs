/*
 * An API to register new backends. In theory, it should allow anyone to add a new target *rather* easily.
 * 
 * This is also the file with the most comments in the whole codebase so far. The `Internal Compiler Design for Newton` 
 * document provides information and examples about the usage of this API.
 * 
 * Newton (C) 2023
 */

/*
 * `Backend` trait. Every new backend must implement this trait and its associated methods.
 */

pub trait Backend {
    // Return the name for the backend.
    fn backend_name(&self) -> &String;

    // Return the description for the backend. This is up to the author(s) of the backend.
    fn backend_description(&self) -> &String;

    // The name(s) of the author(s).
    fn backend_author(&self) -> &String;

    // The target of the backend. Could be C, could be some IR, could be JavaScript (if you're a maniac).
    fn backend_target(&self) -> &String;

    // Return the source stream of the backend.
    fn source(&self) -> &String;

    // The emit method. Supposedly, it should feed generated source code to a `source` field on the backend's struct.
    fn emit(&mut self, code: &str) -> ();

    // Arbitrary header. Could be info about the backend, could be anything else the author wants.
    fn generate_header(&mut self) -> ();
}

// `Display` is already implemented for `BackendInfo`, providing a default pretty-printed message for the backend.
impl std::fmt::Display for dyn Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {} ({})\n{}", self.backend_name(), self.backend_author(), self.backend_target(), self.backend_description())
    }
}

/*
 * This struct is where all the magic happens. You simply `::register()` the new backend, and in *theory* it should all work just fine.
 * Subject to change; if you are the maintainer of a backend, make sure to always check this file with every Newton update to see if
 * there's any changes to the backend API.
 * 
 * Any new backend must implement the aptly named `Backend` trait that provides the base functionality for the newly added target.
 */

pub struct BackendAPI {
    pub backends: std::sync::Mutex<std::collections::HashMap<String, std::rc::Rc<std::cell::RefCell<dyn Backend>>>>,
}

impl BackendAPI {
    pub fn new() -> Self {
        Self { 
            backends: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    // Register the new backend, and push it to a HashMap.
    pub fn register(&mut self, name: &'static str, backend: std::rc::Rc<std::cell::RefCell<dyn Backend>>) -> () {
        self.backends.lock().unwrap().insert(name.to_owned(), backend);
    }

    // Retrieve a specific backend by name.
    // You need to get a reference to the returned value to be able to use the methods that belong to the backend.
    pub fn get(&mut self, name: &'static str) -> std::rc::Rc<std::cell::RefCell<dyn Backend>> {
        self.backends.lock().unwrap().get(name).unwrap().clone()
    }
}
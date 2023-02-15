/*
 * An API to register new backends. In theory, it should allow anyone to add a new target *rather* easily.
 * 
 * This is also the file with the most comments in the whole codebase so far. The `Internal Compiler Design for Newton` 
 * document provides information and examples about the usage of this API.
 * 
 * Newton (C) 2023
 */

/*
 * Just an empty trait that the backend needs to implement.
 */

pub trait Backend {}

/*
 * `BackendInfo` trait. Any new backend needs to derive this trait, and provide the necessary fields for the trait to work properly.
 */
pub trait BackendInfo {
    // Return the name for the backend.
    fn backend_name(&self) -> &String;

    // Return the description for the backend. This is up to the author(s) of the backend.
    fn backend_description(&self) -> &String;

    // The name(s) of the author(s).
    fn backend_author(&self) -> &String;

    // The target of the backend. Could be C, could be some IR, could be JavaScript (if you're a maniac).
    fn backend_target(&self) -> &String;
}

// `Display` is already implemented for `BackendInfo`, providing a default pretty-printed message for the backend.
impl std::fmt::Display for dyn BackendInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} by {} ({})\n{}", self.backend_name(), self.backend_author(), self.backend_target(), self.backend_description())
    }
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

/// The new backend must satisfy the `BackendInfo` and `BackendMethods` bounds. These are explicitly required to properly register the new backend.
/// A new backend must also implement the `Backend` trait, otherwise the HashMap will not accept the target.
pub struct BackendAPI {
    pub backends: std::sync::Mutex<std::collections::HashMap<String, std::rc::Rc<dyn Backend>>>,
}

impl BackendAPI {
    pub fn new() -> Self {
        Self { 
            backends: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    // Register the new backend, and push it to a HashMap.
    pub fn register(&mut self, name: &'static str, backend: std::rc::Rc<dyn Backend>) -> () {
        self.backends.lock().unwrap().insert(name.to_owned(), backend);
    }

    // Retrieve a specific backend by name.
    pub fn get(&mut self, name: &'static str) -> std::rc::Rc<dyn Backend> {
        self.backends.lock().unwrap().get(name).unwrap().clone()
    }
}
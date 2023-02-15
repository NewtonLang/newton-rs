use newton_rs::codegen::api::*;
use newton_rs::codegen::backends::*;

fn main() {
    let c_backend = c::C::new();

    let mut backends: BackendAPI = BackendAPI::new();
    backends.register("C", std::rc::Rc::new(c_backend));
}

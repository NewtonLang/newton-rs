use newton_rs::codegen::api::*;
use newton_rs::codegen::backends::*;

fn main() {
    let c_backend = c::C::new();

    let mut c_target: BackendAPI = BackendAPI::new();
    c_target.register("C", std::rc::Rc::new(c_backend));
}

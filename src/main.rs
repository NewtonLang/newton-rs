use newton_rs::codegen::*;

fn main() {
    let c_backend = backends::c::C::new();

    let mut c_target: api::Backend<backends::c::C> = api::Backend::new();
    c_target.register_backend("C", c_backend);

    c_target.get_backend("C");
}

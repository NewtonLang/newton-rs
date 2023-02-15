use newton_rs::ast::ast::*;
use newton_rs::codegen::api::*;
use newton_rs::parser::span::*;
use newton_rs::types::types::*;
use newton_rs::codegen::backends::*;

fn main() {

    let new_c_backend = c::C::new();

    let mut backends: BackendAPI = BackendAPI::new();
    backends.register("C", std::rc::Rc::new(std::cell::RefCell::new(new_c_backend)));

    let mut args = ArgumentList(Vec::new());

    args.0.push(Spanned::new(0, 0, Expression::new_with_ty(Type::Simple(Simple::Integer(Integer::new_unsigned_int(32))), ExpressionKind::Identifier("size"))));

    let external_function = TopLevel::FunctionDeclaration { 
        name: Spanned::new(0, 0, "malloc"), 
        arguments: args, 
        body: Block(vec![]), 
        return_type: Spanned::new(0, 0, Type::Complex(Complex::Pointer(Pointer::new(Simple::Void, 1)))), 
        is_external: true 
    };

    println!("{}", external_function);
}

use newton_rs::ast::ast::*;
use newton_rs::types::types::*;
use newton_rs::codegen::codegen::*;

fn main() {
    let mut signature = std::collections::BTreeMap::new();
    signature.insert("argc", FieldOrArgument::new("argc", Type::Simple(Simple::Integer(Integer::new_signed_int(32, 0)))));
    signature.insert("argv", FieldOrArgument::new("argv", Type::Complex(Complex::Array(Array::new(Simple::String, None)))));

    // Testing Newton's rather primitive error handling. Switch `correct_signature` for `incorrect_signature` and *hopefully* you'll see a panic in the console.
    let main_function = Function::new("main", signature.clone(), Type::Simple(Simple::Integer(Integer::new_signed_int(32, 0))), vec![]);

    let mut codegen_writer = CodegenWriter::new();
    codegen_writer.generate_main_function(&main_function);

    println!("{}", codegen_writer.output());
}

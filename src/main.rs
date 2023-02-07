use std::borrow::Borrow;

use newton_rs::ast::ast::*;
use newton_rs::types::types::*;
use newton_rs::semantic::error::*;

fn main() {
    let i64 = Type::Simple(Simple::Integer(Integer::new_signed_int(64, i64::MAX)));
    let u64 = Type::Simple(Simple::Integer(Integer::new_unsigned_int(64, u64::MAX)));
    let f32 = Type::Simple(Simple::Float(Float::new_f32(f32::MAX)));
    let f64 = Type::Simple(Simple::Float(Float::new_f64(f64::MAX)));
    let my_type = Type::Simple(Simple::UserDefinedType(UserIdentifier::new("std", "str")));
    let ptr = Type::Complex(Complex::Pointer(Pointer::new(Simple::Void, 2)));
    let sized_arr = Type::Complex(Complex::Array(Array::new(Simple::Character, Some(13))));
    let unsized_arr = Type::Complex(Complex::Array(Array::new(Simple::Character, None)));
    let str = Type::Simple(Simple::String);
    let char = Type::Simple(Simple::Character);
    let void = Type::Simple(Simple::Void);
    let bool = Type::Simple(Simple::Bool);
    let varargs = Type::Simple(Simple::VarArgs);

    println!("{} = {}", i64, i64.is_integer());
    println!("{} = {}", u64, u64.is_integer());
    println!("{} = {}", f32, f32.is_float());
    println!("{} = {}", f64, f64.is_float());
    println!("{}", my_type);
    println!("{} = {}", ptr, ptr.is_pointer());
    println!("{} = {}", sized_arr, sized_arr.is_pointer());
    println!("{} = {}", unsized_arr, unsized_arr.is_pointer());
    println!("{} = {}", str, str.is_pointer());
    println!("{} = {}", char, char.is_character());
    println!("{}", void);
    println!("{}", bool);
    println!("{}", varargs);

    let mut incorrect_signature = std::collections::BTreeMap::new();
    incorrect_signature.insert("argc", FieldOrArgument::new("argc", Type::Simple(Simple::Void)));
    incorrect_signature.insert("argv", FieldOrArgument::new("argv", Type::Simple(Simple::Void)));

    let mut correct_signature = std::collections::BTreeMap::new();
    correct_signature.insert("argc", FieldOrArgument::new("argc", Type::Simple(Simple::Integer(Integer::new_signed_int(32, 0)))));
    correct_signature.insert("argv", FieldOrArgument::new("argv", Type::Complex(Complex::Array(Array::new(Simple::String, None)))));

    // Testing Newton's rather primitive error handling. Switch `correct_signature` for `incorrect_signature` and *hopefully* you'll see a panic in the console.
    let main_function = Function::new("main", incorrect_signature.clone(), Type::Simple(Simple::Integer(Integer::new_signed_int(32, 0))), vec![]);

    //if main_function.get_argument("argc").ttype() != &Type::Simple(Simple::Integer(Integer::new_signed_int(32, 0))) && main_function.get_argument("argv").ttype() != &Type::Complex(Complex::Array(Array::new(Simple::String, None))) {
    if main_function.check_if_signature_is(&incorrect_signature.clone()) {
        panic!("{}", Error::MismatchedMainFunctionArgumentsError(MismatchedMainFunctionArgumentsError::new(format!("{}", main_function))));
    }
}

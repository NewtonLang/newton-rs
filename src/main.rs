use newton_rs::types::types::*;

fn main() {
    let i64 = Type::Simple(Simple::Integer(Integer::new_signed_int(64, i64::MAX)));
    let u64 = Type::Simple(Simple::Integer(Integer::new_unsigned_int(64, u64::MAX)));
    let f32 = Type::Simple(Simple::Float(Float::new_f32(f32::MAX)));
    let f64 = Type::Simple(Simple::Float(Float::new_f64(f64::MAX)));
    let my_type = Type::Simple(Simple::UserDefinedType(UserIdentifier::new("std".to_string(), "str".to_string())));
    let ptr = Type::Complex(Complex::Pointer(Pointer::new(Simple::Void, 1)));
    let sized_arr = Type::Complex(Complex::Array(Array::new(Simple::Character, 13)));
    let unsized_arr = Type::Complex(Complex::Array(Array::new(Simple::Character, 0)));
    let str = Type::Simple(Simple::String);
    let char = Type::Simple(Simple::Character);
    let void = Type::Simple(Simple::Void);
    let bool = Type::Simple(Simple::Bool);
    let varargs = Type::Simple(Simple::VarArgs);

    println!("{}", i64);
    println!("{}", u64);
    println!("{}", f32);
    println!("{}", f64);
    println!("{}", my_type);
    println!("{}", ptr);
    println!("{}", sized_arr);
    println!("{}", unsized_arr);
    println!("{}", str);
    println!("{}", char);
    println!("{}", void);
    println!("{}", bool);
    println!("{}", varargs);
}

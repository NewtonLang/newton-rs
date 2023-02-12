use newton_rs::ast::ast::*;
use newton_rs::types::types::*;

fn main() {
    let mut fields = std::collections::BTreeMap::new();
    fields.insert("x", FieldOrArgument::new("x", Type::Simple(Simple::Float(Float::new_f64()))));
    fields.insert("y", FieldOrArgument::new("y", Type::Simple(Simple::Float(Float::new_f64()))));
    fields.insert("z", FieldOrArgument::new("z", Type::Simple(Simple::Float(Float::new_f64()))));

    let mut methods = std::collections::BTreeMap::new();
    
    let mut common_args = std::collections::BTreeMap::new();
    common_args.insert("self", FieldOrArgument::new("self", Type::Complex(Complex::Ref(Ref::new(Simple::UserDefinedType(UserIdentifier::new("Vec3", "Vec3")), 1)))));

    methods.insert("get_x", Function::new("get_x", common_args.clone(), Type::Simple(Simple::Float(Float::new_f64())), vec![]));
    methods.insert("get_y", Function::new("get_y", common_args.clone(), Type::Simple(Simple::Float(Float::new_f64())), vec![]));
    methods.insert("get_z", Function::new("get_z", common_args.clone(), Type::Simple(Simple::Float(Float::new_f64())), vec![]));

    let strct = Struct::new("Vec3", false, std::collections::BTreeSet::new(), fields, methods);

    println!("{}", strct);
}

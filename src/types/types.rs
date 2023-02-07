use either::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserIdentifier {
    file: String,
    name: String
}

impl UserIdentifier {
    pub fn new(file: String, name: String) -> Self {
        Self {
            file,
            name
        }
    }

    pub fn file(&mut self) -> String {
        self.file.to_owned()
    }

    pub fn name(&mut self) -> String {
        self.name.to_owned()
    }
}

impl std::fmt::Display for UserIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.file, self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Simple(Simple),
    Complex(Complex),
}

impl Type {
    pub fn is_pointer(&self) -> bool {
        match self {
            Type::Complex(Complex::Array(_)) | Type::Complex(Complex::Pointer(_)) | Type::Simple(Simple::String) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Type::Simple(Simple::Integer(_)) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Type::Simple(Simple::Float(_)) => true,
            _ => false,
        }
    }

    pub fn is_numerical(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub fn is_character(&self) -> bool {
        match self {
            Type::Simple(Simple::Character) => true,
            _ => false,
        }
    }

    pub fn simple(&self) -> &Simple {
        match self {
            Type::Simple(ty) => ty,
            Type::Complex(Complex::Array(arr)) => &arr.base_type,
            Type::Complex(Complex::Pointer(ptr)) => &ptr.base_type,
        }
    }

    pub fn arithmetic(&mut self) -> bool {
        if self.is_pointer() {
            true
        } else if let Type::Simple(ty) = self {
            ty.arithmetic() || self.is_character()
        } else {
            false
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Simple(ty) => write!(f, "{}", ty.to_string()),
            Type::Complex(ty) => write!(f, "{}", ty.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Simple {
    String,
    Integer(Integer),
    Float(Float),
    Character,
    Void,
    Bool,
    UserDefinedType(UserIdentifier),
    VarArgs,
}

impl Simple {
    pub fn arithmetic(&mut self) -> bool {
        match self {
            Simple::Integer(_) | Simple::Float(_) | Simple::Character => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Character => write!(f, "char"),
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
            Self::VarArgs => write!(f, "..."),

            Self::Integer(ty) => write!(f, "{}", ty),
            Self::Float(ty) => write!(f, "{}", ty),
            Self::UserDefinedType(ty) => write!(f, "{}", ty),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    size: u8,
    signed: bool,
    value: Either<i64, u64>,
}

impl Integer {
    pub fn new_signed_int(size: u8, value: i64) -> Self {
        Self {
            size,
            signed: true,
            value: Left(value)
        }
    }

    pub fn new_unsigned_int(size: u8, value: u64) -> Self {
        Self {
            size,
            signed: false,
            value: Right(value)
        }
    }

    #[inline]
    pub fn size(&mut self) -> u8 {
        self.size
    }

    #[inline]
    pub fn signed(&mut self) -> bool {
        self.signed
    }

    #[inline]
    pub fn value(&mut self) -> Either<i64, u64> {
        self.value
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.signed {
            write!(f, "i{}", self.size)
        } else {
            write!(f, "u{}", self.size)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float {
    size: u8,
    value: Either<f32, f64>,
}

impl Float {
    pub fn new_f32(value: f32) -> Self {
        Self {
            size: 32,
            value: Left(value),
        }
    }

    pub fn new_f64(value: f64) -> Self {
        Self {
            size: 64,
            value: Right(value),
        }
    }

    #[inline]
    pub fn size(&mut self) -> u8 {
        self.size
    }

    #[inline]
    pub fn value(&mut self) -> Either<f32, f64> {
        self.value
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            32 => write!(f, "f32"),
            64 => write!(f, "f64"),

            _ => panic!("floats cannot have any size other than 32 or 64 so this is pointless lol"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Complex {
    Pointer(Pointer),
    Array(Array),
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pointer(ptr) => write!(f, "{}", ptr),
            Self::Array(arr) => write!(f, "{}", arr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
    base_type: Simple,
    size: u8,
}

impl Pointer {
    pub fn new(base_type: Simple, size: u8) -> Self {
        if size > 2 {
            panic!("ERROR : pointer cannot be more than `**` long.")
        }

        Self {
            base_type,
            size,
        }
    }
}

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", "*".repeat(self.size.into()), self.base_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    base_type: Simple,
    size: Option<u64>,
}

impl Array {
    pub fn new(base_type: Simple, size: Option<u64>) -> Self {
        Self {
            base_type,
            size
        }
    }

    #[inline]
    pub fn base_type(&mut self) -> &Simple {
        &self.base_type
    }

    #[inline]
    pub fn size(&mut self) -> Option<u64> {
        self.size
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            Some(sz) => write!(f, "[{}]{}", sz, self.base_type),
            None => write!(f, "[?]{}", self.base_type)
        }
    }
}
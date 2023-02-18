#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserIdentifier<'a> {
    file: &'a str,
    name: &'a str,
}

impl<'a> UserIdentifier<'a> {
    pub fn new(file: &'a str, name: &'a str) -> Self {
        Self {
            file,
            name,
        }
    }

    pub fn file(&mut self) -> &'a str {
        self.file
    }

    pub fn name(&mut self) -> &'a str {
        self.name
    }
}

impl<'a> std::fmt::Display for UserIdentifier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.file, self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
    Simple(Simple<'a>),
    Complex(Complex<'a>),
}

impl<'a> Type<'a> {
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
            Type::Complex(Complex::Ref(_ref)) => &_ref.base_type,
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

impl<'a> std::fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Simple(ty) => write!(f, "{}", ty),
            Type::Complex(ty) => write!(f, "{}", ty),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Simple<'a> {
    String,
    Integer(Integer),
    Float(Float),
    Character,
    Void,
    Bool,
    UserDefinedType(UserIdentifier<'a>),
    VarArgs,
}

impl<'a> Simple<'a> {
    pub fn arithmetic(&mut self) -> bool {
        match self {
            Simple::Integer(_) | Simple::Float(_) | Simple::Character => true,
            _ => false,
        }
    }
}

impl<'a> std::fmt::Display for Simple<'a> {
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
}

impl Integer {
    pub fn new_signed_int(size: u8) -> Self {
        Self {
            size,
            signed: true,
        }
    }

    pub fn new_unsigned_int(size: u8) -> Self {
        Self {
            size,
            signed: false,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Float {
    size: u8
}

impl Float {
    pub fn new_f32() -> Self {
        Self {
            size: 32,
        }
    }

    pub fn new_f64() -> Self {
        Self {
            size: 64,
        }
    }

    #[inline]
    pub fn size(&mut self) -> u8 {
        self.size
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
pub enum Complex<'a> {
    Pointer(Pointer<'a>),
    Ref(Ref<'a>),
    Array(Array<'a>),
}

impl<'a> std::fmt::Display for Complex<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Pointer(ptr) => write!(f, "{}", ptr),
            Self::Ref(_ref) => write!(f, "{}", _ref),
            Self::Array(arr) => write!(f, "{}", arr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pointer<'a> {
    base_type: Simple<'a>,
    size: u8,
}

impl<'a> Pointer<'a> {
    pub fn new(base_type: Simple<'a>, size: u8) -> Self {
        if size > 2 {
            panic!("ERROR : pointer cannot be more than `**` long.")
        }

        Self {
            base_type,
            size,
        }
    }
}

impl<'a> std::fmt::Display for Pointer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", "*".repeat(self.size.into()), self.base_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ref<'a> {
    base_type: Simple<'a>,
    size: u8,
}

impl<'a> Ref<'a> {
    pub fn new(base_type: Simple<'a>, size: u8) -> Self {
        if size > 2 {
            panic!("ERROR : ref cannot be more than `&&` long.");
        }

        Self {
            base_type,
            size,
        }
    }
}

impl<'a> std::fmt::Display for Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", "&".repeat(self.size.into()), self.base_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array<'a> {
    base_type: Simple<'a>,
    size: Option<u64>,
}

impl<'a> Array<'a> {
    pub fn new(base_type: Simple<'a>, size: Option<u64>) -> Self {
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

impl<'a> std::fmt::Display for Array<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.size {
            Some(sz) => write!(f, "[{}]{}", sz, self.base_type),
            None => write!(f, "[?]{}", self.base_type)
        }
    }
}
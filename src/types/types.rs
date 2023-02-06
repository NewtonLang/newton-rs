#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserIdentifier {
    file: String,
    name: String
}

impl UserIdentifier {
    pub fn new(&mut self, file: String, name: String) -> Self {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Simple(Simple),
    Complex(Complex),
}

impl Type {
    pub fn is_pointer(&mut self) -> bool {
        match self {
            Type::Complex(Complex::Array(_)) | Type::Complex(Complex::Pointer(_)) | Type::Simple(Simple::String) => true,
            _ => false,
        }
    }

    pub fn is_integer(&mut self) -> bool {
        match self {
            Type::Simple(Simple::Integer(_)) => true,
            _ => false,
        }
    }

    pub fn is_float(&mut self) -> bool {
        match self {
            Type::Simple(Simple::Float(_)) => true,
            _ => false,
        }
    }

    pub fn is_numerical(&mut self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub fn is_character(&mut self) -> bool {
        match self {
            Type::Simple(Simple::Character) => true,
            _ => false,
        }
    }

    pub fn simple(&mut self) -> &Simple {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Simple {
    String,
    Integer(Integer),
    Float(Float),
    Character,
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
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Integer {}

impl Integer {}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Float {}

impl Float {}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Complex {
    Pointer(Pointer),
    Array(Array),
}

impl Complex {}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pointer {
    base_type: Simple,
    size: u8,
}

impl Pointer {
    pub fn new(&self, base_type: Simple, size: u8) -> Self {
        if size > 2 {
            println!("ERROR : pointer cannot be more than `**` long.")
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Array {
    base_type: Simple,
    size: u64,
}

impl Array {
    pub fn new(&self, base_type: Simple, size: u64) -> Self {
        Self {
            base_type,
            size
        }
    }

    #[inline]
    pub fn base_type(&mut self) -> Simple {
        self.base_type
    }

    #[inline]
    pub fn size(&mut self) -> u64 {
        self.size
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.size == 0 {
            write!(f, "[?]{}", self.base_type)
        } else {
            write!(f, "[{}]{}", self.size, self.base_type)
        }
    }
}
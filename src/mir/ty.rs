use std::fmt;

#[derive(Clone, PartialEq)]
pub enum MirType {
    Nil,
    Int, I8, I16, I32, I64,
    UInt, U8, U16, U32, U64,
    Float, F32, F64,
}

impl fmt::Debug for MirType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Int => write!(f, "int"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::UInt => write!(f, "uint"),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::Float => write!(f, "float"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
        }
    }
}
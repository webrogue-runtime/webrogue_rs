pub struct FeatureRequirements {
    pub commands: Vec<String>,
    pub enums: Vec<String>,
}
#[derive(PartialEq, Debug, Clone)]
pub enum GLType {
    Void,
    I8,
    U8,
    UInt,
    Int,
    Float,
    ISizeT,
    Ptr(Box<GLType>, bool),
}

impl GLType {
    pub fn to_c_type(&self) -> String {
        match self {
            GLType::Float => "float".to_owned(),
            GLType::I8 => "char".to_owned(),
            GLType::ISizeT => "signed long int".to_owned(),
            GLType::Int => "int".to_owned(),
            GLType::U8 => "unsigned char".to_owned(),
            GLType::UInt => "unsigned int".to_owned(),
            GLType::Void => "void".to_owned(),
            GLType::Ptr(inner, false) => (*inner).to_c_type() + " *",
            GLType::Ptr(inner, true) => (*inner).to_c_type() + " const*",
        }
    }

    pub fn to_rust_type(&self) -> String {
        match self {
            GLType::Float => "f32".to_owned(),
            GLType::I8 => "i8".to_owned(),
            GLType::ISizeT => "isize".to_owned(),
            GLType::Int => "std::os::raw::c_int".to_owned(),
            GLType::U8 => "u8".to_owned(),
            GLType::UInt => "std::os::raw::c_uint".to_owned(),
            GLType::Void => "()".to_owned(),
            GLType::Ptr(inner, false) => format!("*mut {}", (*inner).to_rust_type()),
            GLType::Ptr(inner, true) => format!("*const {}", (*inner).to_rust_type()),
        }
    }

    pub fn to_wasm_param_type(&self) -> String {
        match self {
            GLType::Float => "f32",
            GLType::I8 => "i32",
            GLType::ISizeT => "i32",
            GLType::Int => "i32",
            GLType::U8 => "u32",
            GLType::UInt => "u32",
            GLType::Void => "",
            GLType::Ptr(_, _) => "u32",
        }
        .to_owned()
    }

    pub fn to_wasm_mem_type(&self) -> String {
        match self {
            GLType::Float => "f32",
            GLType::I8 => "i8",
            GLType::ISizeT => "i32",
            GLType::Int => "i32",
            GLType::U8 => "u8",
            GLType::UInt => "u32",
            GLType::Void => "u8",
            GLType::Ptr(_, _) => "u32",
        }
        .to_owned()
    }

    pub fn wasm_type_size(&self) -> usize {
        match self {
            GLType::Float => 4,
            GLType::I8 => 1,
            GLType::ISizeT => 4,
            GLType::Int => 4,
            GLType::U8 => 1,
            GLType::UInt => 4,
            GLType::Void => 1,
            GLType::Ptr(_, _) => 4,
        }
    }
}

#[derive(Clone)]
pub struct Param {
    pub name: String,
    pub ty: GLType,
    pub len_name: Option<String>,
}

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub ret: GLType,
    pub params: Vec<Param>,
}

#[derive(Clone)]
pub struct EnumCase {
    pub name: String,
    pub value: String,
    pub ty: String,
}

#[derive(Clone)]
pub struct ParseResults {
    pub commands: Vec<Command>,
    pub enums: Vec<EnumCase>,
}

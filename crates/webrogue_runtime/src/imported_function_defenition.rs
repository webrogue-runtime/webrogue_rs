use crate::{memory::IMemory, raw_value::RawValue, value_type::ValueType};

#[derive(Clone, Debug)]
pub struct ImportedFunctionDefenition {
    pub module: String,
    pub name: String,
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl ImportedFunctionDefenition {
    pub fn new(module: &str, name: &str, params: Vec<ValueType>, results: Vec<ValueType>) -> Self {
        Self {
            module: module.to_owned(),
            name: name.to_owned(),
            params,
            results,
        }
    }
}

pub struct ImportedFunctionContext {
    pub params: [RawValue; 16],
    pub results: [RawValue; 16],
    pub memory: Box<dyn IMemory>,
}

impl ImportedFunctionContext {
    pub fn new(memory: Box<dyn IMemory>) -> Self {
        Self {
            params: [RawValue::zero(); 16],
            results: [RawValue::zero(); 16],
            memory,
        }
    }
}

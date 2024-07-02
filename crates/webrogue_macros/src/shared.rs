#[derive(Clone)]
pub enum ValueType {
    U32,
    U64,
}

pub struct Import {
    pub module: String,
    pub func_name: String,
    pub args: Vec<ValueType>,
    pub ret_str: Option<ValueType>,
    pub implementation_func_name: String,
}

pub fn get_imports() -> Vec<Import> {
    let mut result = vec![];

    let file_content = include_bytes!("../../webrogue_runtime/src/imported_functions.in");
    let file_content = String::from_utf8_lossy(file_content);

    for line in file_content.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut line = line.to_owned();
        line = line[20..].to_string();
        let dot_pos = line.find(".").unwrap();
        let module = line[..dot_pos].to_string();
        line = line[dot_pos + 1..].to_string();
        // let colon_pos = line.find(":").unwrap();
        let bracket_pos = line.find("(").unwrap();
        let func_name = line[..bracket_pos].to_string();
        line = line[bracket_pos + 1..].to_string();
        let bracket_pos = line.find(")").unwrap();
        let args_str = line[..bracket_pos].to_string();
        line = line[bracket_pos + 6..].to_string();
        let bracket_pos = line.find(")").unwrap();
        let ret_str = line[..bracket_pos].to_string();

        let mut args: Vec<String> = vec![];

        for arg_str in args_str.split(", ") {
            args.push(arg_str.to_string());
        }
        let implementation_func_name = format!("{}::{}", module.clone(), func_name.clone());

        result.push(Import {
            module: module,
            func_name: func_name,
            args: args
                .iter()
                .map(|arg| match arg.as_str() {
                    "u32" => ValueType::U32,
                    "u64" => ValueType::U64,
                    _ => panic!("Unknown type: {}", arg),
                })
                .collect(),
            ret_str: match ret_str.as_str() {
                "u32" => Some(ValueType::U32),
                "u64" => Some(ValueType::U64),
                "" => None,
                _ => panic!("Unknown type: {}", ret_str),
            },
            implementation_func_name: implementation_func_name,
        })
    }

    result
}

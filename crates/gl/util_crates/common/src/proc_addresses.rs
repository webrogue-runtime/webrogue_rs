use crate::types::*;

fn map_param_name(original: String) -> String {
    match original.as_str() {
        "type" => "_type".to_owned(),
        "ref" => "_ref".to_owned(),
        _ => original,
    }
}

pub fn get_as_str(parse_results: &ParseResults) -> String {
    let mut result = "pub struct ProcAddresses {\n".to_owned();
    let mut init_func_ptrs = r#"
impl ProcAddresses {
    pub fn new() -> Self {
        unsafe { std::mem::MaybeUninit::<ProcAddresses>::zeroed().assume_init() }
    }

    pub fn fill(&mut self, _context: &mut webrogue_gfx::Context) {
"#
    .to_owned();
    for command in parse_results.commands.clone() {
        let mut ffi_arg_types = vec![];

        for param in command.params.clone() {
            let mapped_name = map_param_name(param.name.clone());

            ffi_arg_types.push(format!(
                "    {}: {},\n",
                mapped_name,
                param.ty.to_rust_type()
            ));
        }
        result += &format!(
            "#[allow(non_snake_case)]\npub {}: unsafe extern \"stdcall\" fn({}) -> {},\n",
            command.name,
            ffi_arg_types.join(""),
            match command.ret {
                GLType::Void => "()".to_owned(),
                _ => command.ret.to_rust_type(),
            },
        );
        init_func_ptrs += &format!(
            "(self.{}) = unsafe {{ std::mem::transmute(get_proc_address(_context, \"{}\")) }};",
            command.name, command.name
        );
    }
    result += "}\n";
    result += &init_func_ptrs;
    result += "    }\n}\n";
    result
}

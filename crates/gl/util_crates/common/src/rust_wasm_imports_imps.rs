use crate::types::*;

fn simple_type_rust_to_wasm(ty: GLType, name: String) -> Option<String> {
    match ty {
        GLType::Float => Some(name),
        GLType::UInt => Some(format!("{}.into()", name)),
        GLType::Int => Some(format!("{}.into()", name)),
        GLType::U8 => Some(format!("{}.into()", name)),
        GLType::Void => Some(name),
        _ => None,
    }
}

fn map_param_name(original: String) -> String {
    match original.as_str() {
        "type" => "_type".to_owned(),
        "ref" => "_ref".to_owned(),
        _ => original,
    }
}

pub fn get_as_str(parse_results: &ParseResults) -> String {
    let mut result = "".to_owned();
    for command in parse_results.commands.clone() {
        if crate::common::EXCLUDED.contains(&command.name.as_str()) {
            continue;
        }
        if crate::common::MANUAL_IMPL.contains(&command.name.as_str()) {
            continue;
        }
        let mut import_args = vec![];
        let mut ffi_args = vec![];
        let mut converts = vec![];
        let mut writes = vec![];

        let mut is_memory_mut = None;

        for param in command.params.clone() {
            let mapped_name = map_param_name(param.name.clone());
            import_args.push(format!(
                "    {}: {},\n",
                mapped_name,
                param.ty.to_wasm_param_type()
            ));

            let converted_param = match param.ty.clone() {
                GLType::Float => format!("    let converted_{} = {};", mapped_name, mapped_name),
                GLType::UInt => format!("    let converted_{} = {};", mapped_name, mapped_name),
                GLType::Int => format!("    let converted_{} = {};", mapped_name, mapped_name),
                GLType::U8 => format!("    let converted_{} = {} as u8;", mapped_name, mapped_name),
                GLType::ISizeT => {
                    format!(
                        "    let converted_{} = {} as isize;",
                        mapped_name, mapped_name
                    )
                }
                GLType::Void => panic!(),
                GLType::Ptr(inner, is_const) => {
                    let inner_ty = (*inner).clone();
                    if is_const {
                        if is_memory_mut == None {
                            is_memory_mut = Some(false)
                        }
                    } else {
                        is_memory_mut = Some(true)
                    }
                    let rust_type = match inner_ty.clone() {
                        GLType::Void => "u8".to_owned(),
                        _ => inner_ty.to_rust_type(),
                    };
                    let mut len_param = param
                        .len_name
                        .or(Some("UNKNOWN".to_owned()))
                        .clone()
                        .unwrap();
                    if len_param.starts_with("COMPSIZE") {
                        len_param = format!(
                            "crate::compsize::{}_{}_compsize({})",
                            command.name,
                            mapped_name,
                            len_param[9..len_param.len() - 1]
                                .split(",")
                                .map(|s| { map_param_name(s.to_owned()) })
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                    }
                    if !is_const {
                        writes.push(format!(
                            "    for (i, value) in vec_{}.iter().enumerate() {{
        memory
            .write::<{}>(
                webrogue_runtime::wiggle::GuestPtr::<{}>::new({} + (i as u32) * {}),
                *value as {},
            )
            .unwrap()
    }}",
                            mapped_name,
                            inner_ty.to_wasm_mem_type(),
                            inner_ty.to_wasm_mem_type(),
                            mapped_name,
                            inner_ty.wasm_type_size(),
                            inner_ty.to_wasm_mem_type()
                        ))
                    }
                    format!(
                        "    let len_{} = ({}) as usize;
    let mut vec_{}: Vec<{}> = vec![];
    vec_{}.reserve(len_{});
    for i in 0..(len_{} as u32) {{
        vec_{}.push(
            memory
                .read::<{}>(webrogue_runtime::wiggle::GuestPtr::<{}>::new(
                    {} + i * {},
                ))
                .unwrap() as {},
        );
    }}
    let converted_{} = vec_{}.as_mut_ptr() as {};
",
                        mapped_name,
                        len_param,
                        mapped_name,
                        rust_type,
                        mapped_name,
                        mapped_name,
                        mapped_name,
                        mapped_name,
                        inner_ty.to_wasm_mem_type(),
                        inner_ty.to_wasm_mem_type(),
                        mapped_name,
                        inner_ty.wasm_type_size(),
                        rust_type,
                        mapped_name,
                        mapped_name,
                        param.ty.to_rust_type(),
                    )
                }
                GLType::I8 => panic!(),
            };

            converts.push(converted_param);
            ffi_args.push(format!("converted_{}", mapped_name));
        }
        let memory_init = match is_memory_mut {
            None => "",
            Some(false) => "    let memory = _memory_factory.make_memory();\n",
            Some(true) => "    let mut memory = _memory_factory.make_memory();\n",
        };
        result += &format!(
            "
pub fn {}(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
{}) -> {} {{
{}{}
    let result = unsafe {{ crate::ffi::{}({}) }};{}{}
}}
",
            command.name,
            import_args.join(""),
            match command.ret {
                GLType::Void => "()".to_owned(),
                _ => command.ret.to_wasm_param_type(),
            },
            memory_init,
            converts.join("\n"),
            command.name,
            ffi_args.join(", "),
            writes.join("\n"),
            match simple_type_rust_to_wasm(command.ret.clone(), "    result\n".to_owned()) {
                None => {
                    dbg!(command.ret.clone());
                    format!("compile_error!(\"eee1: {}\")", command.name)
                }
                Some(v) => v,
            }
        );
    }
    result
}

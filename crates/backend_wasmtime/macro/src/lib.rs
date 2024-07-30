use proc_macro::TokenStream;

fn val_to_wasmtime(val: &webrogue_macro_common::ValueType) -> String {
    if val.is_float() { "F" } else { "I" }.to_owned() + if val.is_64bit() { "64" } else { "32" }
}

#[proc_macro]
pub fn make_funcs(item: TokenStream) -> TokenStream {
    // let i = get_imports(syn::parse_macro_input!(item as Config));
    let mut result = "
fn make_imports() -> webrogue_backend_wasmtime::Imports {
    webrogue_backend_wasmtime::Imports {
        f: Box::new(| 
            store: *mut webrogue_backend_wasmtime::wasmtime::Store<webrogue_backend_wasmtime::Context>,
            engine: &webrogue_backend_wasmtime::wasmtime::Engine
        | {
            let mut funcs: Vec<(&str, webrogue_backend_wasmtime::wasmtime::Func)> = vec![];
        "
    .to_owned();

    let imports = webrogue_macro_common::parse_macro_input!(item as webrogue_macro_common::Imports);
    for (i, imported_module) in imports.modules.iter().enumerate() {
        for import in imported_module.funcs.clone() {
            result += &format!(
                "
funcs.push((
    \"{}.{}\",
    webrogue_backend_wasmtime::wasmtime::Func::new(
        unsafe {{ &mut *store }},
        webrogue_backend_wasmtime::wasmtime::FuncType::new(
            &engine,
            vec![{}],
            vec![{}],
        ),
        move |mut caller, params, results| {{
            let context = caller.data_mut();
            let result = {}::{}(
                &mut context.memory_factory,
                unsafe {{ context.context_vec.get({}) }},
                {}
            );
            {}
            Ok(())
        }},
    ),
));
            ",
                import.module,
                import.func_name,
                import
                    .args
                    .iter()
                    .map(|arg| {
                        "webrogue_backend_wasmtime::wasmtime::ValType::".to_owned()
                            + &val_to_wasmtime(&arg)
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                match import.ret_str {
                    Some(v) =>
                        "webrogue_backend_wasmtime::wasmtime::ValType::".to_owned()
                            + &val_to_wasmtime(&v),
                    None => "".to_owned(),
                },
                import.rust_module,
                import.func_name,
                i,
                import
                    .args
                    .iter()
                    .enumerate()
                    .map(|(i, arg)| {
                        format!(
                            "params[{}].unwrap_{}(){}",
                            i,
                            val_to_wasmtime(arg).to_lowercase(),
                            if arg.is_unsigned() {
                                " as ".to_owned() + arg.rust_type_str()
                            } else {
                                "".to_owned()
                            },
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",\n"),
                import
                    .ret_str
                    .and_then(|ret| {
                        format!(
                            "results[0] = webrogue_backend_wasmtime::wasmtime::Val::{}(result{});",
                            val_to_wasmtime(&ret),
                            if ret.is_unsigned() {
                                " as i".to_owned() + if ret.is_64bit() { "64" } else { "32" }
                            } else {
                                "".to_owned()
                            },
                        )
                        .into()
                    })
                    .unwrap_or("".to_owned()),
            );
        }
    }
    result += "
            funcs
        })
    }
}
";
    result += &format!(
        "
fn make_context_vec(
    {}
) -> webrogue_runtime::ContextVec {{
    let mut result = webrogue_runtime::ContextVec::new();
    {}
    result
}}
    ",
        imports
            .modules
            .iter()
            .map(|imported_module| {
                format!(
                    "{}_ctx: *mut {}::Context,\n",
                    imported_module.module_name, imported_module.rust_module
                )
            })
            .collect::<Vec<_>>()
            .join(""),
        imports
            .modules
            .iter()
            .enumerate()
            .map(|(i, imported_module)| {
                format!("result.set({}, {}_ctx);\n", i, imported_module.module_name)
            })
            .collect::<Vec<_>>()
            .join("")
    );
    result.parse().unwrap()
}
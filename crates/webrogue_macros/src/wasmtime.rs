extern crate proc_macro;
use proc_macro::TokenStream;

pub fn make_link_functions(_item: TokenStream) -> TokenStream {
    let mut result = "
pub fn get_funcs(
    store: *mut wasmtime::Store<webrogue_runtime::Context>,
    engine: &wasmtime::Engine
) -> Vec<(&str, wasmtime::Func)> {
    let mut funcs: Vec<(&str, wasmtime::Func)> = vec![];
        "
    .to_owned();

    for import in crate::shared::get_imports() {
        result += &format!(
            "
funcs.push((
    \"{}.{}\",
    wasmtime::Func::new(
        unsafe {{ &mut *store }},
        wasmtime::FuncType::new(
            &engine,
            vec![{}],
            vec![{}],
        ),
        move |mut caller, params, results| {{
            let context = caller.data_mut();
            let result = webrogue_runtime::imported_functions::{}(
                context,
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
                    match arg {
                        crate::shared::ValueType::U32 => "wasmtime::ValType::I32",
                    }
                })
                .collect::<Vec<_>>()
                .join(", "),
            match import.ret_str {
                Some(crate::shared::ValueType::U32) => "wasmtime::ValType::I32",
                None => "",
            },
            import.implementation_func_name,
            import
                .args
                .iter()
                .enumerate()
                .map(|(i, arg)| {
                    format!(
                        "params[{}].{}",
                        i,
                        match arg {
                            crate::shared::ValueType::U32 => {
                                "unwrap_i32() as u32"
                            }
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n"),
            match import.ret_str {
                Some(crate::shared::ValueType::U32) =>
                    "results[0] = wasmtime::Val::I32(result as i32);",
                None => "",
            },
        );
    }
    result += "
    funcs
}
    ";
    result.parse().unwrap()
}

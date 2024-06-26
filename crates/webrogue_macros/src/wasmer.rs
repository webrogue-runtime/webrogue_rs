extern crate proc_macro;
use proc_macro::TokenStream;

pub fn make_link_functions(_item: TokenStream) -> TokenStream {
    let mut result = "".to_owned();

    for import in crate::shared::get_imports() {
        let static_func_name = format!("imported_fn_{}_{}", import.module, import.func_name);

        let in_args = import
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                let ty = match arg {
                    crate::shared::ValueType::U32 => "i32",
                    crate::shared::ValueType::U64 => "i64",
                };
                format!(", arg{}: {}", i, ty)
            })
            .collect::<Vec<_>>()
            .join("");

        let out_args = import
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                let conv = match arg {
                    crate::shared::ValueType::U32 => " as u32",
                    crate::shared::ValueType::U64 => " as u64",
                };
                format!("arg{}{},\n", i, conv)
            })
            .collect::<Vec<_>>()
            .join("");

        let out_ty = match import.ret_str.clone() {
            Some(ty) => {
                format!(
                    " -> {}",
                    match ty {
                        crate::shared::ValueType::U32 => "i32",
                        crate::shared::ValueType::U64 => "i64",
                    }
                )
            }
            None => "".to_string(),
        };

        let out_conv = match import.ret_str {
            Some(crate::shared::ValueType::U32) => "as i32",
            Some(crate::shared::ValueType::U64) => "as i64",
            _ => "",
        };

        result += &format!(
            "
fn {}(mut env: wasmer::FunctionEnvMut<Env>{}){} {{
    let mut context = env.data_mut().context.lock().unwrap();
    webrogue_runtime::imported_functions::{}::{}(
        &mut context,
        {}
    ) {}
}}

import_object.define(
    \"{}\",
    \"{}\",
    wasmer::Function::new_typed_with_env(
        &mut store,
        &env,
        {},
    ),
);
    ",
            static_func_name,
            in_args,
            out_ty,
            import.module,
            import.func_name,
            out_args,
            out_conv,
            import.module,
            import.func_name,
            static_func_name,
        );
    }

    result.parse().unwrap()
}

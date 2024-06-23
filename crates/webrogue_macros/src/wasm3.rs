extern crate proc_macro;
use proc_macro::TokenStream;

pub fn make_link_functions(_item: TokenStream) -> TokenStream {
    let mut result = "
pub fn link_functions(
    module: &mut wasm3::Module, 
    context: *mut webrogue_runtime::Context
) {
"
    .to_owned();

    for import in crate::shared::get_imports() {
        let args = import
            .args
            .iter()
            .map(|arg| match arg {
                crate::shared::ValueType::U32 => "u32",
            })
            .collect::<Vec<_>>()
            .join(", ");
        result += &format!(
            "
    let _ = module.link_closure(
        \"{}\",
        \"{}\",
        move |_ctx, args: ({})| -> wasm3::error::TrappedResult<({})> {{
            std::result::Result::Ok(
                webrogue_runtime::imported_functions::{}(
                    unsafe {{ &mut *context }},
                    {}
                ),
            )
        }},
    );
            ",
            import.module,
            import.func_name,
            if import.args.len() == 1 {
                args + ","
            } else {
                args
            },
            match import.ret_str {
                Some(crate::shared::ValueType::U32) => "u32",
                None => "",
            },
            import.implementation_func_name,
            import
                .args
                .iter()
                .enumerate()
                .map(|(i, _)| { format!("args.{},", i) })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    result += "
}
    ";
    result.parse().unwrap()
}

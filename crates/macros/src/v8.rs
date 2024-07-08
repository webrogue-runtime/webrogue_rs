extern crate proc_macro;
use proc_macro::TokenStream;

pub fn make_link_functions(_item: TokenStream) -> TokenStream {
    let mut result = "
pub fn link_functions(scope: &mut v8::HandleScope<v8::Context>, imports: v8::Local<v8::Object>) {
"
    .to_owned();

    for import in crate::shared::get_imports() {
        let args = import
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                format!(
                    "                args.get({}).{}",
                    i,
                    match arg {
                        crate::shared::ValueType::U32 => "uint32_value(scope).unwrap()",
                        crate::shared::ValueType::U64 => "to_big_int(scope).unwrap().u64_value().0",
                    }
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");
        let ret = match import.ret_str {
            Some(ret) => match ret {
                crate::shared::ValueType::U32 => "rv.set_uint32(ret);",
                crate::shared::ValueType::U64 => {
                    "rv.set(v8::BigInt::new_from_u64(scope, ret).into());"
                }
            },
            None => "",
        };
        result += &format!(
            "
    let func = v8::Function::new(
        scope,
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut rv: v8::ReturnValue| {{
            let data = scope.get_data(0);
            let webrogue_context = data as *mut webrogue_runtime::Context;

            let ret = webrogue_runtime::imported_functions::{}::{}(
                unsafe {{ &mut *webrogue_context }},
                {}
            );
            {}
        }},
    )
    .unwrap();
    register_import(scope, imports, \"{}\", \"{}\", func);
            ",
            import.module, import.func_name, args, ret, import.module, import.func_name,
        );
    }
    result += "
}
    ";
    result.parse().unwrap()
}

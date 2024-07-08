#[cfg(feature = "v8_crates_io")]
pub(crate) use webrogue_backend_v8_src_crates_io::v8;
#[cfg(feature = "v8_submodule")]
pub(crate) use webrogue_backend_v8_src_submodule::v8;

fn register_import(
    scope: &mut v8::HandleScope<v8::Context>,
    imports: v8::Local<v8::Object>,
    module_name: &str,
    func_name: &str,
    func: v8::Local<v8::Function>,
) {
    {
        let module_name = v8::String::new(scope, module_name).unwrap();
        let module_object = imports.get(scope, module_name.into());
        let module_object = match module_object {
            Some(module_object) => {
                if module_object.is_object() {
                    module_object.to_object(scope).unwrap()
                } else {
                    v8::Object::new(scope)
                }
            }
            None => v8::Object::new(scope),
        };
        imports.set(scope, module_name.into(), module_object.into());
        let func_name = v8::String::new(scope, func_name).unwrap();
        module_object.set(scope, func_name.into(), func.into());
    };
}

webrogue_macros::make_v8_link_functions!();

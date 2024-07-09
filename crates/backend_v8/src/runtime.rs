use std::ffi::c_void;

#[cfg(feature = "v8_crates_io")]
pub(crate) use webrogue_backend_v8_src_crates_io::v8;
#[cfg(feature = "v8_submodule")]
pub(crate) use webrogue_backend_v8_src_submodule::v8;

use anyhow::Ok;

use crate::memory;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

impl webrogue_runtime::Runtime for Runtime {
    fn run(
        &self,
        wasi: webrogue_runtime::wasi_common::WasiCtx,
        bytecode: Vec<u8>,
    ) -> anyhow::Result<()> {
        {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();

            {
                let mut webrogue_context = Box::new(webrogue_runtime::Context::new(
                    Box::new(memory::StubMemoryFactory {}),
                    wasi,
                ));
                let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
                let handle_scope = &mut v8::HandleScope::new(isolate);

                let context = v8::Context::new(handle_scope);

                let scope = &mut v8::ContextScope::new(handle_scope, context);
                scope.set_data(
                    0,
                    (webrogue_context.as_mut() as *mut webrogue_runtime::Context) as *mut c_void,
                );
                let mut global = context.global(scope);

                let bytecode_array_buffer = v8::ArrayBuffer::new(scope, bytecode.len());
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytecode.as_ptr(),
                        bytecode_array_buffer.data().unwrap().as_ptr() as *mut u8,
                        bytecode.len(),
                    )
                };
                let bytecode_array =
                    v8::Uint8Array::new(scope, bytecode_array_buffer, 0, bytecode.len()).unwrap();
                let key = v8::String::new(scope, "bytecode").unwrap();

                global
                    .set(scope, key.into(), bytecode_array.into())
                    .unwrap();
                {
                    let func = v8::Function::new(
                        scope,
                        |scope: &mut v8::HandleScope,
                         args: v8::FunctionCallbackArguments,
                         mut _rv: v8::ReturnValue| {
                            let message = args.get(0).to_rust_string_lossy(scope);
                            print!("{}\n", message);
                        },
                    )
                    .unwrap();
                    let name = v8::String::new(scope, "print").unwrap();
                    global.set(scope, name.into(), func.into()).unwrap();
                }
                {
                    let imports = v8::Object::new(scope);
                    crate::link_functions::link_functions(scope, imports);
                    let name = v8::String::new(scope, "imports").unwrap();
                    global.set(scope, name.into(), imports.into()).unwrap();
                }
                scope.set_data(
                    1,
                    (&mut global as *mut v8::Local<v8::Object>) as *mut c_void,
                );
                let c_source = r#"
try {
    let module = new WebAssembly.Module(bytecode);
    instance = new WebAssembly.Instance(module, imports);
    global_wasm_memory = instance.exports.memory;
} catch (error) {
    print(error.toString())
}
                "#;
                let source = v8::String::new(scope, c_source).unwrap();
                let script = v8::Script::compile(scope, source, None).unwrap();
                let result = script.run(scope).unwrap();

                let memory_object = result.to_object(scope).unwrap();
                let wasm_memory_object: v8::Local<v8::WasmMemoryObject> =
                    memory_object.try_into().unwrap();
                let memory_factory = memory::MemoryFactory {
                    wasm_memory_object: wasm_memory_object,
                };

                webrogue_context.memory_factory = Box::new(memory::ProxiedMemoryFactory {
                    raw_ptr: (&memory_factory as *const memory::MemoryFactory) as usize,
                });

                let c_source = r#"
try {
    instance.exports._start();
} catch (error) {
    print(error.toString())
}
                "#;
                let source = v8::String::new(scope, c_source).unwrap();
                let script = v8::Script::compile(scope, source, None).unwrap();
                let result = script.run(scope).unwrap();
                result.to_uint32(scope).unwrap();
                drop(webrogue_context);
                drop(memory_factory);
            }

            unsafe {
                v8::V8::dispose();
            }
            v8::V8::dispose_platform();
        }
        Ok(())
    }
}

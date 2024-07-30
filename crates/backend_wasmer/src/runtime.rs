use std::sync::{Arc, Mutex};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

impl webrogue_runtime::Runtime<crate::Imports> for Runtime {
    fn run(
        &self,
        imports: crate::Imports,
        context_vec: webrogue_runtime::ContextVec,
        bytecode: Vec<u8>,
    ) -> anyhow::Result<()> {
        let mut store = wasmer::Store::default();
        let context = crate::context::Context {
            memory_factory: Box::new(crate::memory::StubMemoryFactory {}),
            context_vec,
        };
        // webrogue_runtime::Context::new(, wasi);
        let context_arc = Arc::new(Mutex::new(context));
        let module = wasmer::Module::new(&store, &bytecode)?;
        let env = wasmer::FunctionEnv::new(
            &mut store,
            crate::context::Env {
                context: context_arc.clone(),
            },
        );

        let mut import_object = wasmer::imports! {};

        (imports.f)(&mut import_object, &mut store, env);

        let instance = wasmer::Instance::new(&mut store, &module, &import_object)?;
        let memory = instance.exports.get_memory("memory")?;
        context_arc.lock().unwrap().memory_factory = Box::new(crate::memory::MemoryFactory {
            memory: memory.clone(),
            store_ptr: &mut store,
        });

        let start_fn: wasmer::TypedFunction<(), ()> =
            instance.exports.get_typed_function(&mut store, "_start")?;
        start_fn.call(&mut store)?;
        anyhow::Ok(())
    }
}

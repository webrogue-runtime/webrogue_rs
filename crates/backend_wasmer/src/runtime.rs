use std::sync::{Arc, Mutex};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

struct Env {
    context: Arc<Mutex<webrogue_runtime::Context>>,
}

unsafe impl Sync for Env {}
unsafe impl Send for Env {}

impl webrogue_runtime::Runtime for Runtime {
    fn run(
        &self,
        wasi: webrogue_runtime::wasi_common::WasiCtx,
        bytecode: Vec<u8>,
    ) -> anyhow::Result<()> {
        let mut store = wasmer::Store::default();
        let context =
            webrogue_runtime::Context::new(Box::new(crate::memory::StubMemoryFactory {}), wasi);
        let context_arc = Arc::new(Mutex::new(context));
        let module = wasmer::Module::new(&store, &bytecode)?;
        let env = wasmer::FunctionEnv::new(
            &mut store,
            Env {
                context: context_arc.clone(),
            },
        );

        let mut import_object = wasmer::imports! {};
        webrogue_macros::make_wasmer_link_functions!();

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

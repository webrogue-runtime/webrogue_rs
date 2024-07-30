use wasmtime::AsContextMut;

use crate::{
    imports::implement_imports,
    memory::{MemoryFactory, StubMemoryFactory},
};

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
        let mut config = wasmtime::Config::new();
        config.debug_info(true);
        config.cranelift_opt_level(wasmtime::OptLevel::None);
        let engine = wasmtime::Engine::new(&config)?;
        let module = wasmtime::Module::new(&engine, bytecode)?;

        let context = crate::context::Context {
            memory_factory: Box::new(StubMemoryFactory::new()),
            context_vec,
        };

        let mut store_box = Box::new(wasmtime::Store::new(&engine, context));
        let store_ptr = &mut *store_box;

        let imports = implement_imports(imports, &module, store_ptr, &engine)?;

        let instance = wasmtime::Instance::new(&mut *store_ptr, &module, &imports)?;

        let memory = instance.get_memory(&mut *store_ptr, "memory");
        let memory = match memory {
            None => anyhow::bail!("memory not found"),
            Some(memory) => memory,
        };

        (*store_ptr).data_mut().memory_factory = Box::new(MemoryFactory::new(memory, store_ptr));

        let run = instance.get_typed_func::<(), ()>(store_ptr.as_context_mut(), "_start")?;

        run.call(&mut *store_ptr, ())?;
        Ok(())
    }
}

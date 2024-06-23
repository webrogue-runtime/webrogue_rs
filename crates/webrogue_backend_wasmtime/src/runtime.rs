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

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = std::fs::File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    std::io::Read::read(&mut f, &mut buffer).expect("buffer overflow");

    buffer
}

impl webrogue_runtime::Runtime for Runtime {
    fn run(&self) -> anyhow::Result<()> {
        let mut config = wasmtime::Config::new();
        config.debug_info(true);
        config.cranelift_opt_level(wasmtime::OptLevel::None);
        let engine = wasmtime::Engine::new(&config)?;
        let module = wasmtime::Module::new(
            &engine,
            get_file_as_byte_vec("./example_mods/simple/main.wasm"),
        )?;

        let mut store_box = Box::new(wasmtime::Store::new(
            &engine,
            webrogue_runtime::Context::new(Box::new(StubMemoryFactory::new())),
        ));
        let store_ptr = &mut *store_box;

        let imports = implement_imports(&module, store_ptr, &engine)?;

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

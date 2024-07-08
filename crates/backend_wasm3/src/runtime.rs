use anyhow::Ok;

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
        let env = wasm3::Environment::new().expect("Unable to create environment");
        let runtime = env
            .create_runtime(1024 * 60)
            .map_err(|_| anyhow::Error::msg("m3_NewRuntime returned an error"))?;

        let mut runtime_box = Box::new(runtime);
        let runtime_ptr: *mut wasm3::Runtime = &mut *runtime_box;

        // .expect("Unable to create runtime");
        let module = wasm3::Module::parse(&env, bytecode)
            .map_err(|_| anyhow::Error::msg("m3_ParseModule returned an error"))?;
        let mut module = runtime_box
            .load_module(module)
            .map_err(|_| anyhow::Error::msg("m3_LoadModule returned an error"))?;
        let mut context = webrogue_runtime::Context::new(
            Box::new(crate::memory::MemoryFactory::new(runtime_ptr)),
            wasi,
        );
        crate::link_functions::link_functions(&mut module, &mut context);

        let func = module
            .find_function::<(), ()>("_start")
            .map_err(|_| anyhow::Error::msg("m3_FindFunction returned an error"))?;
        func.call()
            .map_err(|_| anyhow::Error::msg("m3_CallV returned an error"))?;
        drop(runtime_box);
        Ok(())
    }
}

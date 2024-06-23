use anyhow::Ok;

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
        let module_data = get_file_as_byte_vec("./example_mods/simple/main.wasm");

        let env = wasm3::Environment::new().expect("Unable to create environment");
        let runtime = env
            .create_runtime(1024 * 60)
            .map_err(|_| anyhow::Error::msg("m3_NewRuntime returned an error"))?;

        let mut runtime_box = Box::new(runtime);
        let runtime_ptr: *mut wasm3::Runtime = &mut *runtime_box;

        // .expect("Unable to create runtime");
        let module = wasm3::Module::parse(&env, module_data)
            .map_err(|_| anyhow::Error::msg("m3_ParseModule returned an error"))?;
        let mut module = runtime_box
            .load_module(module)
            .map_err(|_| anyhow::Error::msg("m3_LoadModule returned an error"))?;
        let mut context = webrogue_runtime::Context::new(Box::new(
            crate::memory::MemoryFactory::new(runtime_ptr),
        ));
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

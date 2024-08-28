use anyhow::Ok;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

extern "C" {
    fn wr_rs_em_js_initWasmModule(pointer: *const u8, size: u32);
    fn wr_rs_em_js_makeWorker(jsonPtr: *const u8);
    fn wr_rs_em_js_terminateWorker();
    fn wr_rs_em_js_execFunc(funcNamePtr: *const u8);
    fn wr_rs_em_js_waitForStatus() -> i32;
    fn wr_rs_em_js_continueFuncExecution();
    // fn wr_rs_em_js_modErrorSize() -> u32;
    // fn wr_rs_em_js_getModError(error: *mut u8);
    fn wr_rs_em_js_getImportedFuncId() -> u32;
    fn wr_rs_em_js_makeSharedMemory(inital_pages: u32, max_pages: u32);
}

fn exec_async_func(
    func_name: &str,
    context: &mut crate::context::Context,
    funcs: Vec<Box<dyn Fn(&mut crate::Context)>>,
) -> bool {
    unsafe {
        let mut func_name = func_name.as_bytes().to_vec();
        func_name.push(0);
        wr_rs_em_js_execFunc(func_name.as_ptr());
        loop {
            match wr_rs_em_js_waitForStatus() {
                1 => {
                    let func_id = wr_rs_em_js_getImportedFuncId();
                    funcs[func_id as usize](context);
                    // if (!procExit)
                    wr_rs_em_js_continueFuncExecution();
                }
                2 => break,
                val => {
                    panic!("wr_rs_em_js_waitForStatus returned unknown value: {}", val)
                }
            }
        }
        // let error_size = wr_rs_em_js_modErrorSize();
        // if error_size != 0 {
        //     let mut error_text = vec![0u8; error_size as usize];
        //     wr_rs_em_js_getModError(error_text.as_mut_ptr());
        //     eprintln!("error: {}", String::from_utf8(error_text).unwrap());
        //     return false;
        // }
        return true;
    }
}

impl webrogue_runtime::Runtime<crate::Imports> for Runtime {
    fn run(
        &self,
        imports: crate::Imports,
        context_vec: webrogue_runtime::ContextVec,
        bytecode: Vec<u8>,
        memory_size_range: Option<(u64, u64)>,
    ) -> anyhow::Result<()> {
        unsafe {
            if let Some(memory_size_range) = memory_size_range {
                wr_rs_em_js_makeSharedMemory(
                    memory_size_range.0 as u32,
                    memory_size_range.1 as u32,
                );
            }
            let mut worker_config = imports.to_json().as_bytes().to_vec();
            worker_config.push(0);
            let mut context = crate::context::Context {
                context_vec: context_vec,
                memory_factory: Box::new(crate::memory::MemoryFactory {}),
            };
            // let context = Arc::new(Mutex::new(context));
            wr_rs_em_js_makeWorker(worker_config.as_ptr());
            wr_rs_em_js_initWasmModule(bytecode.as_ptr(), bytecode.len() as u32);

            exec_async_func("_start", &mut context, imports.funcs);
            wr_rs_em_js_terminateWorker();
        }
        Ok(())
    }
}

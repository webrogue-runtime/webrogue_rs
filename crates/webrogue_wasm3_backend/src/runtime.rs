use std::{
    ffi::{c_void, CString},
    ptr::{null, null_mut},
};
use webrogue_runtime::{imported_function_defenition::ImportedFunctionContext, runtime::IRuntime};

use wasm3::{Environment, Module};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

fn wr_log(data: String) {
    println!("{}", data);
}

extern "C" {
    // pub fn m3_NewEnvironment() -> *mut c_void;

    // pub fn m3_NewRuntime(
    //     i_environment: *mut c_void,
    //     i_stackSizeInBytes: u32,
    //     i_userdata: *mut c_void,
    // ) -> *mut c_void;

    // pub fn m3_ParseModule(
    //     i_environment: *mut c_void,
    //     o_module: *mut *mut c_void,
    //     i_bytes: *const u8,
    //     i_numBytes: u32,
    // ) -> *const u8;

    // pub fn m3_LoadModule(io_runtime: *mut c_void, io_module: *mut *mut c_void) -> *const u8;
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = std::fs::File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    std::io::Read::read(&mut f, &mut buffer).expect("buffer overflow");

    buffer
}

impl IRuntime for Runtime {
    fn run(&self) -> anyhow::Result<()> {
        let module_data = get_file_as_byte_vec("./example_mods/simple/main.wasm");
        // let environment = unsafe { m3_NewEnvironment() };
        // if environment.is_null() {
        //     panic!();
        // }
        // let runtime =
        //     unsafe { m3_NewRuntime(environment, 64 * 1024 * 1024, core::ptr::null_mut()) };
        // if runtime.is_null() {
        //     panic!();
        // }
        // let mut module: *mut c_void = null_mut();
        // let mut err = unsafe {
        //     m3_ParseModule(
        //         environment,
        //         std::ptr::addr_of_mut!(module),
        //         module_data.as_ptr(),
        //         module_data.len().try_into().expect(""),
        //     )
        // };
        // if !err.is_null() {
        //     panic!();
        // }
        // err = unsafe { m3_LoadModule(runtime, std::ptr::addr_of_mut!(module)) };
        // if !err.is_null() {
        //     panic!();
        // }

        let env = Environment::new().expect("Unable to create environment");
        let rt = env
            .create_runtime(1024 * 60)
            .expect("Unable to create runtime");
        let module = Module::parse(&env, module_data).expect("Unable to parse module");

        let mut module = rt.load_module(module).expect("Unable to load module");
        let _ =
            module.link_function::<i32, ()>("webrogue", "imported_func_1", imported_func_1_wrapped);

        let _ = module.link_function::<i32, i32>(
            "webrogue",
            "imported_func_2",
            imported_func_2_wrapped,
        );

        let _ = module.link_function::<(i32, i32), ()>(
            "webrogue",
            "imported_func_3",
            imported_func_3_wrapped,
        );

        let _ = module.link_function::<(i32, i32), ()>(
            "webroguea",
            "imported_func_4",
            imported_func_3_wrapped,
        );

        let func = module
            .find_function::<(), ()>("exported_func_1")
            .expect("Unable to find function");
        func.call().expect("Err");
        Ok(())
    }
}

wasm3::make_func_wrapper!(imported_func_1_wrapped: imported_func_1(a: i32) -> ());
fn imported_func_1(a: i32) -> () {
    println!("imported_func_1");
}

wasm3::make_func_wrapper!(imported_func_2_wrapped: imported_func_2(a: i32) -> i32);
fn imported_func_2(a: i32) -> i32 {
    println!("imported_func_2");
    2
}

wasm3::make_func_wrapper!(imported_func_3_wrapped: imported_func_3(a: i32, b: i32) -> ());
fn imported_func_3(a: i32, b: i32) -> () {
    println!("imported_func_3")
}

use crate::{
    imported_function_defenition::{ImportedFunctionContext, ImportedFunctionDefenition},
    value_type::ValueType,
    wasm_memory_types::{WasmMemoryDataType, WasmMemoryPtr, WasmMemoryU32, WasmMemoryU8},
};

pub fn imported_func_1(context: &mut ImportedFunctionContext) {
    println!(
        "imported_func_1({})",
        WasmMemoryU32::from_raw(context.params[0]).value
    )
}

pub fn imported_func_2(context: &mut ImportedFunctionContext) {
    let arg = WasmMemoryU32::from_raw(context.params[0]);
    println!("imported_func_2({})", arg.value);
    context.results[0] = WasmMemoryU32 {
        value: arg.value + 1,
    }
    .to_raw();
}

pub fn imported_func_3(context: &mut ImportedFunctionContext) {
    println!("memory_size({})", context.memory.size());

    let ptr = WasmMemoryPtr::<WasmMemoryU8>::from_raw(context.params[0]);
    let len = WasmMemoryU32::from_raw(context.params[1]);
    let data = context.memory.get_range(ptr.value, len.value);
    match data {
        Ok(data) => {
            println!("imported_func_3({})", String::from_utf8_lossy(&data));
            match ptr.set(&mut context.memory, WasmMemoryU8 { value: 48 }) {
                Err(_) => {
                    println!("imported_func_3 err");
                }
                Ok(_) => {}
            };
        }
        Err(_) => {
            println!("imported_func_3 err");
        }
    }
}

pub fn wasi_snapshot_preview1_args_sizes_get(context: &mut ImportedFunctionContext) {
    let argc_ptr = WasmMemoryPtr::<WasmMemoryU32>::from_raw(context.params[0]);
    let argv_buf_size_ptr = WasmMemoryPtr::<WasmMemoryU32>::from_raw(context.params[1]);
    argc_ptr.set(&mut context.memory, WasmMemoryU32 { value: 1 });
    argv_buf_size_ptr.set(&mut context.memory, WasmMemoryU32 { value: 6 });
    context.results[0] = WasmMemoryU32 { value: 0 }.to_raw();
}

pub fn wasi_snapshot_preview1_args_get(context: &mut ImportedFunctionContext) {
    let argv = WasmMemoryPtr::<WasmMemoryPtr<WasmMemoryU8>>::from_raw(context.params[0]);
    let argv_buf = WasmMemoryPtr::<WasmMemoryU8>::from_raw(context.params[1]);

    argv.set_ref(&mut context.memory, &argv_buf);
    argv_buf.set_vec(
        &mut context.memory,
        vec![WasmMemoryU8 { value: 49 }, WasmMemoryU8 { value: 0 }],
    );
    context.results[0] = WasmMemoryU32 { value: 0 }.to_raw();
}

pub fn wasi_snapshot_preview1_proc_exit(context: &mut ImportedFunctionContext) {}

pub fn get_function_table() -> Vec<(ImportedFunctionDefenition, fn(&mut ImportedFunctionContext))> {
    vec![
        (
            ImportedFunctionDefenition::new(
                "webrogue",
                "imported_func_1",
                vec![ValueType::I32],
                vec![],
            ),
            imported_func_1,
        ),
        (
            ImportedFunctionDefenition::new(
                "webrogue",
                "imported_func_2",
                vec![ValueType::I32],
                vec![ValueType::I32],
            ),
            imported_func_2,
        ),
        (
            ImportedFunctionDefenition::new(
                "webrogue",
                "imported_func_3",
                vec![ValueType::I32, ValueType::I32],
                vec![],
            ),
            imported_func_3,
        ),
        (
            ImportedFunctionDefenition::new(
                "wasi_snapshot_preview1",
                "args_sizes_get",
                vec![ValueType::I32, ValueType::I32],
                vec![ValueType::I32],
            ),
            wasi_snapshot_preview1_args_sizes_get,
        ),
        (
            ImportedFunctionDefenition::new(
                "wasi_snapshot_preview1",
                "args_get",
                vec![ValueType::I32, ValueType::I32],
                vec![ValueType::I32],
            ),
            wasi_snapshot_preview1_args_get,
        ),
        (
            ImportedFunctionDefenition::new(
                "wasi_snapshot_preview1",
                "proc_exit",
                vec![ValueType::I32],
                vec![],
            ),
            wasi_snapshot_preview1_proc_exit,
        ),
    ]
}

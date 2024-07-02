use webrogue_runtime::WasiFactory;

fn main() -> anyhow::Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();

    let wasi_factory = webrogue_wasi_sync::WasiFactory {};
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            let backend = webrogue_backend_wasmer::Backend::new();
            lifecycle.run(
                backend,
                wasi,
                include_bytes!("../../../../../../../example_mods/simple/main.wasm").to_vec(),
            )
        }),
    );

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn webrogue_android_main() {
    main().unwrap();
}

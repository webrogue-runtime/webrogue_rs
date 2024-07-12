use anyhow::Result;
use clap::Parser;
use webrogue_runtime::WasiFactory;

#[cfg(feature = "backend_wasmtime")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasmtime::Backend::new()
}

#[cfg(feature = "_backend_wasmer")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasmer::Backend::new()
}

#[cfg(feature = "backend_wasm3")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasm3::Backend::new()
}

#[cfg(feature = "backend_v8")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_v8::Backend::new()
}

#[cfg(feature = "wasi_sync")]
fn make_wasi_factory() -> impl webrogue_runtime::WasiFactory {
    webrogue_wasi_sync::WasiFactory {}
}

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();

    let wasi_factory = make_wasi_factory();
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, std::env::current_dir()?, "/");

    let args = Cli::parse();
    let reader = webrogue_runtime::wrapp::Reader::from_file_path(args.path)?;

    #[cfg(all(feature = "std_stream_sdl", feature = "std_stream_os"))]
    compile_error!("webrogue_rs currently can't use more than one std_stream implementation");

    #[cfg(feature = "std_stream_sdl")]
    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            let backend = make_backend();
            lifecycle.run(backend, wasi, reader.clone()).unwrap();
        }),
    );

    #[cfg(not(feature = "std_stream_sdl"))]
    {
        #[cfg(feature = "std_stream_os")]
        webrogue_std_stream_os::bind_streams(&mut wasi);

        let backend = make_backend();
        lifecycle.run(backend, wasi, reader)?
    }

    Ok(())
}

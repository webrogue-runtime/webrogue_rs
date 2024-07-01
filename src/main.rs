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

#[cfg(feature = "wasi_sync")]
fn make_wasi_factory() -> impl webrogue_runtime::WasiFactory {
    webrogue_wasi_sync::WasiFactory {}
}

fn get_file_as_byte_vec(filename: std::path::PathBuf) -> Vec<u8> {
    let mut f = std::fs::File::open(filename.clone()).expect("no file found");
    let metadata = std::fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    std::io::Read::read(&mut f, &mut buffer).expect("buffer overflow");

    buffer
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

    #[cfg(all(feature = "std_stream_sdl", feature = "std_stream_os"))]
    compile_error!("webrogue_rs currently can't use more than one std_stream implementation");

    #[cfg(feature = "std_stream_sdl")]
    webrogue_std_stream_sdl::run_in_terminal(
        wasi,
        std::sync::Arc::new(move |wasi| {
            let backend = make_backend();
            let args = Cli::parse();
            lifecycle.run(backend, wasi, get_file_as_byte_vec(args.path))
        }),
    );

    #[cfg(not(feature = "std_stream_sdl"))]
    {
        #[cfg(feature = "std_stream_os")]
        webrogue_std_stream_os::bind_streams(&mut wasi);

        let backend = make_backend();
        let args = Cli::parse();
        lifecycle.run(backend, wasi, get_file_as_byte_vec(args.path))?
    }

    Ok(())
}

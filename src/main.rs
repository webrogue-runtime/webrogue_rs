use anyhow::Result;
use webrogue_runtime::WasiFactory;

#[cfg(feature = "wasmtime")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasmtime::Backend::new()
}

#[cfg(feature = "wasm3")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasm3::Backend::new()
}

#[cfg(feature = "sync")]
fn make_wasi_factory() -> impl webrogue_runtime::WasiFactory {
    webrogue_wasi_sync::WasiFactory {}
}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = std::fs::File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    std::io::Read::read(&mut f, &mut buffer).expect("buffer overflow");

    buffer
}

fn main() -> Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();
    let backend = make_backend();

    let wasi_factory = make_wasi_factory();
    let mut wasi = wasi_factory.make();

    wasi_factory.add_actual_dir(&mut wasi, ".", "/");

    lifecycle.run(
        backend,
        wasi,
        get_file_as_byte_vec("./example_mods/simple/main.wasm"),
    )?;
    Ok(())
}

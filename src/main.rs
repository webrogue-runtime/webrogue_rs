use anyhow::Result;

#[cfg(feature = "wasmtime")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasmtime::Backend::new()
}

#[cfg(feature = "wasm3")]
fn make_backend() -> impl webrogue_runtime::Backend {
    webrogue_backend_wasm3::Backend::new()
}

fn main() -> Result<()> {
    let lifecycle = webrogue_runtime::Lifecycle::new();
    let backend = make_backend();

    lifecycle.run(backend)?;
    Ok(())
}

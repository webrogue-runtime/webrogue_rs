use anyhow::Result;
use webrogue_runtime::lifecycle::Lifecycle;

fn main() -> Result<()> {
    let lifecycle = Lifecycle::new();
    let backend = webrogue_wasm3_backend::backend::Backend::new();
    lifecycle.run(backend)?;
    Ok(())
}

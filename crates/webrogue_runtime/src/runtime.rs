use anyhow::Result;

pub trait Runtime {
    fn run(&self, wasi: wasi_common::WasiCtx, bytecode: Vec<u8>) -> Result<()>;
}

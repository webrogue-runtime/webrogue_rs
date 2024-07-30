use anyhow::Result;

pub trait Runtime<Imports> {
    fn run(
        &self,
        imports: Imports,
        context_vec: crate::context::ContextVec,
        bytecode: Vec<u8>,
    ) -> Result<()>;
}

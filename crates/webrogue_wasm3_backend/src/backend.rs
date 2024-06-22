pub struct Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {}
    }
}

impl webrogue_runtime::backend::IBackend for Backend {
    fn make_runtime(&self) -> Box<dyn webrogue_runtime::runtime::IRuntime> {
        Box::new(crate::runtime::Runtime::new())
    }
}

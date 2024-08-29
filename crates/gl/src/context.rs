pub struct Context {
    pub gfx_context: *mut webrogue_gfx::Context,
    pub proc_addresses: crate::proc_addresses::ProcAddresses,
}

impl Context {
    pub fn new(gfx_context: *mut webrogue_gfx::Context) -> Self {
        Context {
            gfx_context,
            proc_addresses: crate::proc_addresses::ProcAddresses::new(),
        }
    }
}

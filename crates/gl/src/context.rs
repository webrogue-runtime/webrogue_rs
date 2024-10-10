pub struct Context {
    pub gfx_context: *mut webrogue_gfx::Context,
    pub proc_addresses: crate::proc_addresses::ProcAddresses,
    pub opaque_sync_objects: std::collections::BTreeMap<u32, *mut ()>,
    pub opaque_sync_object_counter: u32,
}

impl Context {
    pub fn new(gfx_context: *mut webrogue_gfx::Context) -> Self {
        Context {
            gfx_context,
            proc_addresses: crate::proc_addresses::ProcAddresses::new(),
            opaque_sync_objects: std::collections::BTreeMap::new(),
            opaque_sync_object_counter: 1,
        }
    }

    pub fn register_opaque_sync_object(&mut self, object: *mut ()) -> u32 {
        let result = self.opaque_sync_object_counter;
        self.opaque_sync_objects.insert(self.opaque_sync_object_counter, object);
        self.opaque_sync_object_counter += 1;
        return result;
    }

    pub fn resolve_opaque_sync_object(&mut self, handle: u32) -> *mut () {
        self.opaque_sync_objects.get(&handle).unwrap_or(&std::ptr::null_mut()).clone()
    }

    pub fn delete_opaque_sync_object(&mut self, handle: u32) {
        self.opaque_sync_objects.remove(&handle);
    }
}

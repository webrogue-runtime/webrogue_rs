pub struct MemoryFactory {
    runtime: *mut wasm3::Runtime,
}

impl MemoryFactory {
    pub fn new(runtime: *mut wasm3::Runtime) -> Self {
        Self { runtime }
    }
}

impl webrogue_runtime::MemoryFactory for MemoryFactory {
    fn make_memory(&self) -> webrogue_runtime::GuestMemory {
        if true {
            let runtime = unsafe { self.runtime.as_mut().unwrap() };
            let memory = runtime.memory_mut();
            let data = unsafe { memory.as_mut().unwrap() };
            webrogue_runtime::GuestMemory::Unshared(data)
        } else {
            webrogue_runtime::GuestMemory::Dynamic(Box::new(DynamicMemory {
                runtime: self.runtime,
            }))
        }
    }
}
struct DynamicMemory {
    runtime: *mut wasm3::Runtime,
}

impl webrogue_runtime::DynamicGuestMemory for DynamicMemory {
    fn size(&self) -> usize {
        unsafe { self.runtime.as_mut().unwrap().memory_mut().len() }
    }

    fn write(&mut self, offset: u32, data: &[u8]) {
        let runtime = unsafe { self.runtime.as_mut().unwrap() };
        let memory = runtime.memory_mut();
        unsafe {
            (*memory)
                .as_mut_ptr()
                .add(offset as usize)
                .copy_from_nonoverlapping(data.as_ptr(), data.len())
        };
    }

    fn read(&self, offset: u32, data: &mut [u8]) {
        let runtime = unsafe { self.runtime.as_mut().unwrap() };
        let memory = runtime.memory_mut();
        unsafe {
            (*memory)
                .as_mut_ptr()
                .add(offset as usize)
                .copy_to_nonoverlapping(data.as_mut_ptr(), data.len())
        };
    }
}

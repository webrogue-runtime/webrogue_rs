use anyhow::{Error, Ok, Result};
use webrogue_runtime::{imported_function_defenition::ImportedFunctionContext, memory::IMemory};

pub struct StubMemory {}

impl StubMemory {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMemory for StubMemory {
    fn size(&self) -> u32 {
        panic!("StubMemory used")
    }

    fn get_range(&self, _ptr: u32, _size: u32) -> Result<Vec<u8>> {
        panic!("StubMemory used")
    }

    fn set_range(&self, _ptr: u32, _data: Vec<u8>) -> Result<()> {
        panic!("StubMemory used")
    }
}

pub struct Memory {
    wasmtime_memory: wasmtime::Memory,
    store: *mut wasmtime::Store<ImportedFunctionContext>,
}

impl Memory {
    pub fn new(
        wasmtime_memory: wasmtime::Memory,
        store: *mut wasmtime::Store<ImportedFunctionContext>,
    ) -> Self {
        Self {
            wasmtime_memory,
            store,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
struct MemoryAccessError {}

impl MemoryAccessError {
    pub fn new() -> Self {
        Self {}
    }
}

impl core::fmt::Display for MemoryAccessError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "out of bounds memory access")
    }
}

impl std::error::Error for MemoryAccessError {}

impl Memory {
    fn store_ref(&self) -> &mut wasmtime::Store<ImportedFunctionContext> {
        unsafe { &mut *self.store }
    }

    fn get_slice(&self, ptr: u32, size: usize) -> Result<&mut [u8]> {
        Ok(self
            .wasmtime_memory
            .data_mut(self.store_ref())
            .get_mut(ptr.try_into().unwrap()..)
            .and_then(|s| s.get_mut(..size))
            .ok_or(Error::new(MemoryAccessError::new()))?)
    }
}

impl IMemory for Memory {
    fn size(&self) -> u32 {
        self.wasmtime_memory
            .data_size(self.store_ref())
            .try_into()
            .unwrap()
    }

    fn get_range(&self, ptr: u32, size: u32) -> Result<Vec<u8>> {
        let slice = self.get_slice(ptr, size.try_into().unwrap())?;
        Result::Ok(slice.to_vec())
    }

    fn set_range(&self, ptr: u32, data: Vec<u8>) -> Result<()> {
        let slice = self.get_slice(ptr, data.len())?;
        slice.copy_from_slice(&data);
        Ok(())
    }
}

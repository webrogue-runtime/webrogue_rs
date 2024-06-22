use std::io::Write;

use crate::{memory::IMemory, raw_value::RawValue};

pub trait WasmMemoryDataType {
    fn size() -> usize;
    fn read(data: *const u8) -> Self;
    fn write(&self, data: *mut u8);
    fn from_raw(raw_ralue: RawValue) -> Self;
    fn to_raw(&self) -> RawValue;
}

pub struct WasmMemoryI32 {
    pub value: i32,
}

impl WasmMemoryDataType for WasmMemoryI32 {
    fn size() -> usize {
        4
    }

    fn read(data: *const u8) -> Self {
        let mut bytes: [u8; 4] = [0; 4];
        unsafe { std::ptr::copy_nonoverlapping(data, bytes.as_mut_ptr(), 4) }
        WasmMemoryI32 {
            value: i32::from_ne_bytes(bytes),
        }
    }

    fn write(&self, data: *mut u8) {
        let bytes = self.value.to_le_bytes();
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, 4) }
    }

    fn from_raw(raw_ralue: RawValue) -> Self {
        WasmMemoryI32 {
            value: unsafe { raw_ralue.i32 },
        }
    }

    fn to_raw(&self) -> RawValue {
        RawValue { i32: self.value }
    }
}

pub struct WasmMemoryU32 {
    pub value: u32,
}

impl WasmMemoryDataType for WasmMemoryU32 {
    fn size() -> usize {
        4
    }

    fn read(data: *const u8) -> Self {
        let mut bytes: [u8; 4] = [0; 4];
        unsafe { std::ptr::copy_nonoverlapping(data, bytes.as_mut_ptr(), 4) }
        WasmMemoryU32 {
            value: u32::from_ne_bytes(bytes),
        }
    }

    fn write(&self, data: *mut u8) {
        let bytes = self.value.to_le_bytes();
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, 4) }
    }

    fn from_raw(raw_ralue: RawValue) -> Self {
        WasmMemoryU32 {
            value: unsafe { raw_ralue.u32 },
        }
    }

    fn to_raw(&self) -> RawValue {
        RawValue { u32: self.value }
    }
}

pub struct WasmMemoryPtr<T>
where
    T: WasmMemoryDataType,
{
    pub value: u32,
    a: std::marker::PhantomData<T>,
}

impl<T> WasmMemoryPtr<T>
where
    T: WasmMemoryDataType,
{
    pub fn set(&self, memory: &mut Box<dyn IMemory>, data: T) -> anyhow::Result<()> {
        self.set_ref(memory, &data)
    }

    pub fn set_ref(&self, memory: &mut Box<dyn IMemory>, data: &T) -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = vec![0; T::size()];
        data.write(bytes.as_mut_ptr());
        memory.as_mut().set_range(self.value, bytes)?;
        Ok(())
    }

    pub fn set_vec(&self, memory: &mut Box<dyn IMemory>, data: Vec<T>) -> anyhow::Result<()> {
        self.set_vec_ref(memory, &data)
    }

    pub fn set_vec_ref(&self, memory: &mut Box<dyn IMemory>, data: &Vec<T>) -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = vec![0; T::size() * data.len()];
        for (index, value) in data.iter().enumerate() {
            value.write(unsafe { bytes.as_mut_ptr().add(index * T::size()) });
        }
        memory.as_mut().set_range(self.value, bytes)?;
        Ok(())
    }
}

impl<T: WasmMemoryDataType> WasmMemoryDataType for WasmMemoryPtr<T> {
    fn size() -> usize {
        4
    }

    fn read(data: *const u8) -> Self {
        let mut bytes: [u8; 4] = [0; 4];
        unsafe { std::ptr::copy_nonoverlapping(data, bytes.as_mut_ptr(), 4) }
        WasmMemoryPtr::<T> {
            value: u32::from_ne_bytes(bytes),
            a: std::marker::PhantomData,
        }
    }

    fn write(&self, data: *mut u8) {
        let bytes = self.value.to_le_bytes();
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), data, 4) }
    }

    fn from_raw(raw_ralue: RawValue) -> Self {
        WasmMemoryPtr::<T> {
            value: unsafe { raw_ralue.u32 },
            a: std::marker::PhantomData,
        }
    }

    fn to_raw(&self) -> RawValue {
        RawValue { u32: self.value }
    }
}

pub struct WasmMemoryU8 {
    pub value: u8,
}

impl WasmMemoryDataType for WasmMemoryU8 {
    fn size() -> usize {
        1
    }

    fn read(data: *const u8) -> Self {
        WasmMemoryU8 {
            value: unsafe { *data },
        }
    }

    fn write(&self, data: *mut u8) {
        unsafe { *data = self.value }
    }

    fn from_raw(raw_ralue: RawValue) -> Self {
        todo!();
    }

    fn to_raw(&self) -> RawValue {
        todo!();
    }
}

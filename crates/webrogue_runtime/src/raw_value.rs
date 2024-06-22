#[derive(Copy)]
pub union RawValue {
    pub i32: i32,
    pub i64: i64,
    pub u32: u32,
    pub u64: u64,
}

impl Clone for RawValue {
    fn clone(&self) -> Self {
        RawValue {
            i64: unsafe { self.i64 },
        }
    }
}

impl RawValue {
    pub fn zero() -> Self {
        RawValue { i64: 0 }
    }
}

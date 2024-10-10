pub fn guest_strlen(memory: &webrogue_runtime::GuestMemory, ptr: u32) -> usize {
    let mut new_ptr = ptr;
    loop {
        let read_result = memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(new_ptr));
        match read_result {
            Ok(byte) => {
                if byte == 0 {
                    break;
                }
            }
            Err(_) => break,
        }
        new_ptr += 1
    }
    return (new_ptr - ptr) as usize;
}

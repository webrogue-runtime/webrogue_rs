use crate::context::Context;

#[allow(non_snake_case)]
pub fn glShaderSource(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
    count: i32,
    string: u32,
    length: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let mut sources: Vec<Vec<i8>> = vec![];
    sources.reserve(count as usize);
    for i in 0..count {
        let mut size = None;
        if length != 0 {
            let size_candidate = memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + (i as u32) * 4,
                ))
                .unwrap() as i32;
            if size_candidate >= 0 {
                size = Some(size_candidate);
            }
        }
        let string_offset = memory
            .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                string + (i as u32) * 4,
            ))
            .unwrap() as u32;
        sources.push(match size {
            None => {
                let mut source: Vec<i8> = vec![];
                let mut byte_offset = 0;
                loop {
                    let byte = memory
                        .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                            string_offset + byte_offset,
                        ))
                        .unwrap();
                    if byte == 0 {
                        break;
                    } else {
                        byte_offset += 1;
                        source.push(byte);
                    }
                }
                source
            }
            Some(size) => {
                let slice = memory
                    .as_cow(webrogue_runtime::wiggle::GuestPtr::<[u8]>::new((
                        string_offset as u32,
                        size as u32,
                    )))
                    .unwrap();
                let v = slice.to_vec();
                unsafe {
                    std::slice::from_raw_parts(v.as_ptr() as *const i8, v.len() as usize).to_vec()
                }
            }
        });
    }

    let mut converted_string: Vec<*mut i8> = vec![];
    let mut converted_length: Vec<std::os::raw::c_int> = vec![];

    sources.iter_mut().for_each(|source| {
        converted_string.push(source.as_mut_ptr());
        converted_length.push(source.len() as std::os::raw::c_int);
    });

    let result = unsafe {
        crate::ffi::glShaderSource(
            shader as std::os::raw::c_uint,
            count as std::os::raw::c_int,
            converted_string.as_mut_ptr(),
            converted_length.as_mut_ptr(),
        )
    };
    drop(sources);
    result
}

#[allow(non_snake_case)]
pub fn glGetAttribLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    name: u32,
) -> i32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let mut vec_name: Vec<i8> = {
        let mut source: Vec<i8> = vec![];
        let mut byte_offset = 0;
        loop {
            let byte = memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + byte_offset,
                ))
                .unwrap();
            source.push(byte);
            if byte == 0 {
                break;
            } else {
                byte_offset += 1;
            }
        }
        source
    };
    let converted_name = vec_name.as_mut_ptr() as *mut i8;

    let result = unsafe { crate::ffi::glGetAttribLocation(converted_program, converted_name) };
    result.into()
}

#[allow(non_snake_case)]
pub fn glVertexAttribPointer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    size: i32,
    _type: u32,
    normalized: u32,
    stride: i32,
    pointer: u32,
) -> () {
    let converted_index = index;
    let converted_size = size;
    let converted__type = _type;
    let converted_normalized = normalized as u8;
    let converted_stride = stride;

    let result = unsafe {
        crate::ffi::glVertexAttribPointer(
            converted_index,
            converted_size,
            converted__type,
            converted_normalized,
            converted_stride,
            pointer as *mut (),
        )
    };
    result
}

#[allow(non_snake_case)]
pub fn glBindAttribLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    index: u32,
    name: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let mut vec_name: Vec<i8> = {
        let mut source: Vec<i8> = vec![];
        let mut byte_offset = 0;
        loop {
            let byte = memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + byte_offset,
                ))
                .unwrap();
            source.push(byte);
            if byte == 0 {
                break;
            } else {
                byte_offset += 1;
            }
        }
        source
    };
    let converted_name = vec_name.as_mut_ptr() as *mut i8;
    let result = unsafe {
        crate::ffi::glBindAttribLocation(converted_program, converted_index, converted_name)
    };
    result
}
#[allow(non_snake_case)]
pub fn glGetUniformLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    name: u32,
) -> i32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let mut vec_name: Vec<i8> = {
        let mut source: Vec<i8> = vec![];
        let mut byte_offset = 0;
        loop {
            let byte = memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + byte_offset,
                ))
                .unwrap();
            source.push(byte);
            if byte == 0 {
                break;
            } else {
                byte_offset += 1;
            }
        }
        source
    };
    let converted_name = vec_name.as_mut_ptr() as *mut i8;
    let result = unsafe { crate::ffi::glGetUniformLocation(converted_program, converted_name) };
    result.into()
}

#[allow(non_snake_case)]
pub fn glTexImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    border: i32,
    format: u32,
    _type: u32,
    pixels: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let converted_border = border;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels =
        (crate::compsize::glTexImage2D_pixels_compsize(format, _type, width, height)) as usize;
    let mut vec_pixels: Vec<u8> = vec![];
    let converted_pixels = if pixels != 0 {
        vec_pixels.reserve(len_pixels);
        for i in 0..(len_pixels as u32) {
            vec_pixels.push(
                memory
                    .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                        pixels + i * 1,
                    ))
                    .unwrap() as u8,
            );
        }
        vec_pixels.as_mut_ptr() as *mut ()
    } else {
        std::ptr::null_mut()
    };
    let result = unsafe {
        crate::ffi::glTexImage2D(
            converted_target,
            converted_level,
            converted_internalformat,
            converted_width,
            converted_height,
            converted_border,
            converted_format,
            converted__type,
            converted_pixels,
        )
    };
    let e = unsafe { crate::ffi::glGetError() };
    println!("{}", e);
    result
}

fn get_string(name: u32) -> Option<Vec<u8>> {
    if name == crate::ffi::GL_EXTENSIONS {
        return Some(Vec::new());
    }
    let gl_str = unsafe { crate::ffi::glGetString(name) } as *const i8;
    if gl_str.is_null() {
        return None;
    }
    let owned_str = unsafe { std::ffi::CStr::from_ptr(gl_str) };
    Some(owned_str.to_bytes_with_nul().to_vec())
}

#[allow(non_snake_case)]
pub fn glGetStringData(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    name: u32,
    data_ptr: u32,
) {
    if let Some(s) = get_string(name) {
        let mut memory = _memory_factory.make_memory();
        let ptr = webrogue_runtime::wiggle::GuestPtr::<[u8]>::new((data_ptr, s.len() as u32));
        let _ = memory.copy_from_slice(s.as_slice(), ptr);
    }
}
#[allow(non_snake_case)]
pub fn glGetStringLen(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    name: u32,
) -> i32 {
    match get_string(name) {
        None => -1,
        Some(s) => s.len() as i32,
    }
}

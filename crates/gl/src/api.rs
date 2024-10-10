#![allow(non_snake_case)]

pub use crate::context::Context;
pub use crate::mainual_impl::*;

// DO NOT EDIT! This file is generated automatically



#[rustfmt::skip]
pub fn glActiveTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    texture: u32,
) -> () {
    let converted_texture = texture;
    let result = unsafe { (
            _context.proc_addresses.glActiveTexture
        )(converted_texture) };    result
 
}


#[rustfmt::skip]
pub fn glAttachShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    shader: u32,
) -> () {
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe { (
            _context.proc_addresses.glAttachShader
        )(converted_program, converted_shader) };    result
 
}


#[rustfmt::skip]
pub fn glBeginQuery(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    id: u32,
) -> () {
    let converted_target = target;
    let converted_id = id;
    let result = unsafe { (
            _context.proc_addresses.glBeginQuery
        )(converted_target, converted_id) };    result
 
}


#[rustfmt::skip]
pub fn glBeginTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    primitiveMode: u32,
) -> () {
    let converted_primitiveMode = primitiveMode;
    let result = unsafe { (
            _context.proc_addresses.glBeginTransformFeedback
        )(converted_primitiveMode) };    result
 
}


#[rustfmt::skip]
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
    let len_name = (crate::utils::guest_strlen(&memory, name) + 1) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *const i8;

    let result = unsafe { (
            _context.proc_addresses.glBindAttribLocation
        )(converted_program, converted_index, converted_name) };    result
 
}


#[rustfmt::skip]
pub fn glBindBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    buffer: u32,
) -> () {
    let converted_target = target;
    let converted_buffer = buffer;
    let result = unsafe { (
            _context.proc_addresses.glBindBuffer
        )(converted_target, converted_buffer) };    result
 
}


#[rustfmt::skip]
pub fn glBindBufferBase(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    index: u32,
    buffer: u32,
) -> () {
    let converted_target = target;
    let converted_index = index;
    let converted_buffer = buffer;
    let result = unsafe { (
            _context.proc_addresses.glBindBufferBase
        )(converted_target, converted_index, converted_buffer) };    result
 
}


#[rustfmt::skip]
pub fn glBindBufferRange(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    index: u32,
    buffer: u32,
    offset: i32,
    size: i32,
) -> () {
    let converted_target = target;
    let converted_index = index;
    let converted_buffer = buffer;
    let converted_offset = offset as isize;
    let converted_size = size as isize;
    let result = unsafe { (
            _context.proc_addresses.glBindBufferRange
        )(converted_target, converted_index, converted_buffer, converted_offset, converted_size) };    result
 
}


#[rustfmt::skip]
pub fn glBindFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    framebuffer: u32,
) -> () {
    let converted_target = target;
    let converted_framebuffer = framebuffer;
    let result = unsafe { (
            _context.proc_addresses.glBindFramebuffer
        )(converted_target, converted_framebuffer) };    result
 
}


#[rustfmt::skip]
pub fn glBindRenderbuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    renderbuffer: u32,
) -> () {
    let converted_target = target;
    let converted_renderbuffer = renderbuffer;
    let result = unsafe { (
            _context.proc_addresses.glBindRenderbuffer
        )(converted_target, converted_renderbuffer) };    result
 
}


#[rustfmt::skip]
pub fn glBindSampler(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    unit: u32,
    sampler: u32,
) -> () {
    let converted_unit = unit;
    let converted_sampler = sampler;
    let result = unsafe { (
            _context.proc_addresses.glBindSampler
        )(converted_unit, converted_sampler) };    result
 
}


#[rustfmt::skip]
pub fn glBindTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    texture: u32,
) -> () {
    let converted_target = target;
    let converted_texture = texture;
    let result = unsafe { (
            _context.proc_addresses.glBindTexture
        )(converted_target, converted_texture) };    result
 
}


#[rustfmt::skip]
pub fn glBindTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    id: u32,
) -> () {
    let converted_target = target;
    let converted_id = id;
    let result = unsafe { (
            _context.proc_addresses.glBindTransformFeedback
        )(converted_target, converted_id) };    result
 
}


#[rustfmt::skip]
pub fn glBindVertexArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    array: u32,
) -> () {
    let converted_array = array;
    let result = unsafe { (
            _context.proc_addresses.glBindVertexArray
        )(converted_array) };    result
 
}


#[rustfmt::skip]
pub fn glBindVertexArrayOES(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    array: u32,
) -> () {
    let converted_array = array;
    let result = unsafe { (
            _context.proc_addresses.glBindVertexArrayOES
        )(converted_array) };    result
 
}


#[rustfmt::skip]
pub fn glBlendColor(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) -> () {
    let converted_red = red;
    let converted_green = green;
    let converted_blue = blue;
    let converted_alpha = alpha;
    let result = unsafe { (
            _context.proc_addresses.glBlendColor
        )(converted_red, converted_green, converted_blue, converted_alpha) };    result
 
}


#[rustfmt::skip]
pub fn glBlendEquation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { (
            _context.proc_addresses.glBlendEquation
        )(converted_mode) };    result
 
}


#[rustfmt::skip]
pub fn glBlendEquationSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    modeRGB: u32,
    modeAlpha: u32,
) -> () {
    let converted_modeRGB = modeRGB;
    let converted_modeAlpha = modeAlpha;
    let result = unsafe { (
            _context.proc_addresses.glBlendEquationSeparate
        )(converted_modeRGB, converted_modeAlpha) };    result
 
}


#[rustfmt::skip]
pub fn glBlendFunc(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sfactor: u32,
    dfactor: u32,
) -> () {
    let converted_sfactor = sfactor;
    let converted_dfactor = dfactor;
    let result = unsafe { (
            _context.proc_addresses.glBlendFunc
        )(converted_sfactor, converted_dfactor) };    result
 
}


#[rustfmt::skip]
pub fn glBlendFuncSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sfactorRGB: u32,
    dfactorRGB: u32,
    sfactorAlpha: u32,
    dfactorAlpha: u32,
) -> () {
    let converted_sfactorRGB = sfactorRGB;
    let converted_dfactorRGB = dfactorRGB;
    let converted_sfactorAlpha = sfactorAlpha;
    let converted_dfactorAlpha = dfactorAlpha;
    let result = unsafe { (
            _context.proc_addresses.glBlendFuncSeparate
        )(converted_sfactorRGB, converted_dfactorRGB, converted_sfactorAlpha, converted_dfactorAlpha) };    result
 
}


#[rustfmt::skip]
pub fn glBlitFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    srcX0: i32,
    srcY0: i32,
    srcX1: i32,
    srcY1: i32,
    dstX0: i32,
    dstY0: i32,
    dstX1: i32,
    dstY1: i32,
    mask: u32,
    filter: u32,
) -> () {
    let converted_srcX0 = srcX0;
    let converted_srcY0 = srcY0;
    let converted_srcX1 = srcX1;
    let converted_srcY1 = srcY1;
    let converted_dstX0 = dstX0;
    let converted_dstY0 = dstY0;
    let converted_dstX1 = dstX1;
    let converted_dstY1 = dstY1;
    let converted_mask = mask;
    let converted_filter = filter;
    let result = unsafe { (
            _context.proc_addresses.glBlitFramebuffer
        )(converted_srcX0, converted_srcY0, converted_srcX1, converted_srcY1, converted_dstX0, converted_dstY0, converted_dstX1, converted_dstY1, converted_mask, converted_filter) };    result
 
}


#[rustfmt::skip]
pub fn glBufferData(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    size: i32,
    data: u32,
    usage: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_size = size as isize;
    let len_data = (size) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *const ();

    let converted_usage = usage;
    let result = unsafe { (
            _context.proc_addresses.glBufferData
        )(converted_target, converted_size, converted_data, converted_usage) };    result
 
}


#[rustfmt::skip]
pub fn glCheckFramebufferStatus(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> u32 {
    let converted_target = target;
    let result = unsafe { (
            _context.proc_addresses.glCheckFramebufferStatus
        )(converted_target) };    result
.into() 
}


#[rustfmt::skip]
pub fn glClear(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mask: u32,
) -> () {
    let converted_mask = mask;
    let result = unsafe { (
            _context.proc_addresses.glClear
        )(converted_mask) };    result
 
}


#[rustfmt::skip]
pub fn glClearBufferfi(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
    drawbuffer: i32,
    depth: f32,
    stencil: i32,
) -> () {
    let converted_buffer = buffer;
    let converted_drawbuffer = drawbuffer;
    let converted_depth = depth;
    let converted_stencil = stencil;
    let result = unsafe { (
            _context.proc_addresses.glClearBufferfi
        )(converted_buffer, converted_drawbuffer, converted_depth, converted_stencil) };    result
 
}


#[rustfmt::skip]
pub fn glClearBufferfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
    drawbuffer: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_buffer = buffer;
    let converted_drawbuffer = drawbuffer;
    let len_value = (crate::compsize::glClearBufferfv_value_compsize(_context, crate::ffi::GLEnumGroupBuffer::from_raw(buffer))) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glClearBufferfv
        )(converted_buffer, converted_drawbuffer, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glClearBufferiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
    drawbuffer: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_buffer = buffer;
    let converted_drawbuffer = drawbuffer;
    let len_value = (crate::compsize::glClearBufferiv_value_compsize(_context, crate::ffi::GLEnumGroupBuffer::from_raw(buffer))) as usize;
    let mut vec_value: Vec<std::os::raw::c_int> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glClearBufferiv
        )(converted_buffer, converted_drawbuffer, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glClearBufferuiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
    drawbuffer: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_buffer = buffer;
    let converted_drawbuffer = drawbuffer;
    let len_value = (crate::compsize::glClearBufferuiv_value_compsize(_context, crate::ffi::GLEnumGroupBuffer::from_raw(buffer))) as usize;
    let mut vec_value: Vec<std::os::raw::c_uint> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glClearBufferuiv
        )(converted_buffer, converted_drawbuffer, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glClearColor(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) -> () {
    let converted_red = red;
    let converted_green = green;
    let converted_blue = blue;
    let converted_alpha = alpha;
    let result = unsafe { (
            _context.proc_addresses.glClearColor
        )(converted_red, converted_green, converted_blue, converted_alpha) };    result
 
}


#[rustfmt::skip]
pub fn glClearDepthf(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    d: f32,
) -> () {
    let converted_d = d;
    let result = unsafe { (
            _context.proc_addresses.glClearDepthf
        )(converted_d) };    result
 
}


#[rustfmt::skip]
pub fn glClearStencil(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    s: i32,
) -> () {
    let converted_s = s;
    let result = unsafe { (
            _context.proc_addresses.glClearStencil
        )(converted_s) };    result
 
}


#[rustfmt::skip]
pub fn glClientWaitSync(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sync: u32,
    flags: u32,
    timeout: u64,
) -> u32 {
    let converted_sync = _context.resolve_opaque_sync_object(sync);
    let converted_flags = flags;
    let converted_timeout = timeout;
    let result = unsafe { (
            _context.proc_addresses.glClientWaitSync
        )(converted_sync, converted_flags, converted_timeout) };    result
.into() 
}


#[rustfmt::skip]
pub fn glColorMask(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
) -> () {
    let converted_red = red as u8;
    let converted_green = green as u8;
    let converted_blue = blue as u8;
    let converted_alpha = alpha as u8;
    let result = unsafe { (
            _context.proc_addresses.glColorMask
        )(converted_red, converted_green, converted_blue, converted_alpha) };    result
 
}


#[rustfmt::skip]
pub fn glCompileShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> () {
    let converted_shader = shader;
    let result = unsafe { (
            _context.proc_addresses.glCompileShader
        )(converted_shader) };    result
 
}


#[rustfmt::skip]
pub fn glCompressedTexImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    border: i32,
    imageSize: i32,
    data: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let converted_border = border;
    let converted_imageSize = imageSize;
    let len_data = (imageSize) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glCompressedTexImage2D
        )(converted_target, converted_level, converted_internalformat, converted_width, converted_height, converted_border, converted_imageSize, converted_data) };    result
 
}


#[rustfmt::skip]
pub fn glCompressedTexImage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
    border: i32,
    imageSize: i32,
    data: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let converted_depth = depth;
    let converted_border = border;
    let converted_imageSize = imageSize;
    let len_data = (imageSize) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glCompressedTexImage3D
        )(converted_target, converted_level, converted_internalformat, converted_width, converted_height, converted_depth, converted_border, converted_imageSize, converted_data) };    result
 
}


#[rustfmt::skip]
pub fn glCompressedTexSubImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    imageSize: i32,
    data: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted_imageSize = imageSize;
    let len_data = (imageSize) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glCompressedTexSubImage2D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_width, converted_height, converted_format, converted_imageSize, converted_data) };    result
 
}


#[rustfmt::skip]
pub fn glCompressedTexSubImage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    imageSize: i32,
    data: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_zoffset = zoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_depth = depth;
    let converted_format = format;
    let converted_imageSize = imageSize;
    let len_data = (imageSize) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glCompressedTexSubImage3D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_zoffset, converted_width, converted_height, converted_depth, converted_format, converted_imageSize, converted_data) };    result
 
}


#[rustfmt::skip]
pub fn glCopyBufferSubData(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    readTarget: u32,
    writeTarget: u32,
    readOffset: i32,
    writeOffset: i32,
    size: i32,
) -> () {
    let converted_readTarget = readTarget;
    let converted_writeTarget = writeTarget;
    let converted_readOffset = readOffset as isize;
    let converted_writeOffset = writeOffset as isize;
    let converted_size = size as isize;
    let result = unsafe { (
            _context.proc_addresses.glCopyBufferSubData
        )(converted_readTarget, converted_writeTarget, converted_readOffset, converted_writeOffset, converted_size) };    result
 
}


#[rustfmt::skip]
pub fn glCopyTexImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    internalformat: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border: i32,
) -> () {
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let converted_border = border;
    let result = unsafe { (
            _context.proc_addresses.glCopyTexImage2D
        )(converted_target, converted_level, converted_internalformat, converted_x, converted_y, converted_width, converted_height, converted_border) };    result
 
}


#[rustfmt::skip]
pub fn glCopyTexSubImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> () {
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glCopyTexSubImage2D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_x, converted_y, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glCopyTexSubImage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> () {
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_zoffset = zoffset;
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glCopyTexSubImage3D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_zoffset, converted_x, converted_y, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glCreateProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {

    let result = unsafe { (
            _context.proc_addresses.glCreateProgram
        )() };    result
.into() 
}


#[rustfmt::skip]
pub fn glCreateShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    _type: u32,
) -> u32 {
    let converted__type = _type;
    let result = unsafe { (
            _context.proc_addresses.glCreateShader
        )(converted__type) };    result
.into() 
}


#[rustfmt::skip]
pub fn glCullFace(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { (
            _context.proc_addresses.glCullFace
        )(converted_mode) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteBuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    buffers: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_buffers = (n) as usize;
    let mut vec_buffers: Vec<std::os::raw::c_uint> = vec![];
    vec_buffers.reserve(len_buffers);
    for i in 0..(len_buffers as u32) {
        vec_buffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    buffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_buffers = vec_buffers.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteBuffers
        )(converted_n, converted_buffers) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteFramebuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    framebuffers: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_framebuffers = (n) as usize;
    let mut vec_framebuffers: Vec<std::os::raw::c_uint> = vec![];
    vec_framebuffers.reserve(len_framebuffers);
    for i in 0..(len_framebuffers as u32) {
        vec_framebuffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    framebuffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_framebuffers = vec_framebuffers.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteFramebuffers
        )(converted_n, converted_framebuffers) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { (
            _context.proc_addresses.glDeleteProgram
        )(converted_program) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteQueries(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    ids: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_ids = (n) as usize;
    let mut vec_ids: Vec<std::os::raw::c_uint> = vec![];
    vec_ids.reserve(len_ids);
    for i in 0..(len_ids as u32) {
        vec_ids.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    ids + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_ids = vec_ids.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteQueries
        )(converted_n, converted_ids) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteRenderbuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    renderbuffers: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_renderbuffers = (n) as usize;
    let mut vec_renderbuffers: Vec<std::os::raw::c_uint> = vec![];
    vec_renderbuffers.reserve(len_renderbuffers);
    for i in 0..(len_renderbuffers as u32) {
        vec_renderbuffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    renderbuffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_renderbuffers = vec_renderbuffers.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteRenderbuffers
        )(converted_n, converted_renderbuffers) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteSamplers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    count: i32,
    samplers: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_count = count;
    let len_samplers = (count) as usize;
    let mut vec_samplers: Vec<std::os::raw::c_uint> = vec![];
    vec_samplers.reserve(len_samplers);
    for i in 0..(len_samplers as u32) {
        vec_samplers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    samplers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_samplers = vec_samplers.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteSamplers
        )(converted_count, converted_samplers) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> () {
    let converted_shader = shader;
    let result = unsafe { (
            _context.proc_addresses.glDeleteShader
        )(converted_shader) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteSync(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sync: u32,
) -> () {
    let converted_sync = _context.resolve_opaque_sync_object(sync);
    let result = unsafe { (
            _context.proc_addresses.glDeleteSync
        )(converted_sync) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteTextures(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    textures: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_textures = (n) as usize;
    let mut vec_textures: Vec<std::os::raw::c_uint> = vec![];
    vec_textures.reserve(len_textures);
    for i in 0..(len_textures as u32) {
        vec_textures.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    textures + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_textures = vec_textures.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteTextures
        )(converted_n, converted_textures) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteTransformFeedbacks(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    ids: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_ids = (n) as usize;
    let mut vec_ids: Vec<std::os::raw::c_uint> = vec![];
    vec_ids.reserve(len_ids);
    for i in 0..(len_ids as u32) {
        vec_ids.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    ids + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_ids = vec_ids.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteTransformFeedbacks
        )(converted_n, converted_ids) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteVertexArrays(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    arrays: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_arrays = (n) as usize;
    let mut vec_arrays: Vec<std::os::raw::c_uint> = vec![];
    vec_arrays.reserve(len_arrays);
    for i in 0..(len_arrays as u32) {
        vec_arrays.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    arrays + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_arrays = vec_arrays.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteVertexArrays
        )(converted_n, converted_arrays) };    result
 
}


#[rustfmt::skip]
pub fn glDeleteVertexArraysOES(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    arrays: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_arrays = (n) as usize;
    let mut vec_arrays: Vec<std::os::raw::c_uint> = vec![];
    vec_arrays.reserve(len_arrays);
    for i in 0..(len_arrays as u32) {
        vec_arrays.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    arrays + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_arrays = vec_arrays.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDeleteVertexArraysOES
        )(converted_n, converted_arrays) };    result
 
}


#[rustfmt::skip]
pub fn glDepthFunc(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    func: u32,
) -> () {
    let converted_func = func;
    let result = unsafe { (
            _context.proc_addresses.glDepthFunc
        )(converted_func) };    result
 
}


#[rustfmt::skip]
pub fn glDepthMask(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    flag: u32,
) -> () {
    let converted_flag = flag as u8;
    let result = unsafe { (
            _context.proc_addresses.glDepthMask
        )(converted_flag) };    result
 
}


#[rustfmt::skip]
pub fn glDepthRangef(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: f32,
    f: f32,
) -> () {
    let converted_n = n;
    let converted_f = f;
    let result = unsafe { (
            _context.proc_addresses.glDepthRangef
        )(converted_n, converted_f) };    result
 
}


#[rustfmt::skip]
pub fn glDetachShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    shader: u32,
) -> () {
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe { (
            _context.proc_addresses.glDetachShader
        )(converted_program, converted_shader) };    result
 
}


#[rustfmt::skip]
pub fn glDisable(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> () {
    let converted_cap = cap;
    let result = unsafe { (
            _context.proc_addresses.glDisable
        )(converted_cap) };    result
 
}


#[rustfmt::skip]
pub fn glDisableVertexAttribArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
) -> () {
    let converted_index = index;
    let result = unsafe { (
            _context.proc_addresses.glDisableVertexAttribArray
        )(converted_index) };    result
 
}


#[rustfmt::skip]
pub fn glDrawArrays(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
    first: i32,
    count: i32,
) -> () {
    let converted_mode = mode;
    let converted_first = first;
    let converted_count = count;
    let result = unsafe { (
            _context.proc_addresses.glDrawArrays
        )(converted_mode, converted_first, converted_count) };    result
 
}


#[rustfmt::skip]
pub fn glDrawArraysInstanced(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
    first: i32,
    count: i32,
    instancecount: i32,
) -> () {
    let converted_mode = mode;
    let converted_first = first;
    let converted_count = count;
    let converted_instancecount = instancecount;
    let result = unsafe { (
            _context.proc_addresses.glDrawArraysInstanced
        )(converted_mode, converted_first, converted_count, converted_instancecount) };    result
 
}


#[rustfmt::skip]
pub fn glDrawBuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    bufs: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_bufs = (n) as usize;
    let mut vec_bufs: Vec<std::os::raw::c_uint> = vec![];
    vec_bufs.reserve(len_bufs);
    for i in 0..(len_bufs as u32) {
        vec_bufs.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    bufs + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_bufs = vec_bufs.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glDrawBuffers
        )(converted_n, converted_bufs) };    result
 
}


#[rustfmt::skip]
pub fn glEnable(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> () {
    let converted_cap = cap;
    let result = unsafe { (
            _context.proc_addresses.glEnable
        )(converted_cap) };    result
 
}


#[rustfmt::skip]
pub fn glEnableVertexAttribArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
) -> () {
    let converted_index = index;
    let result = unsafe { (
            _context.proc_addresses.glEnableVertexAttribArray
        )(converted_index) };    result
 
}


#[rustfmt::skip]
pub fn glEndQuery(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> () {
    let converted_target = target;
    let result = unsafe { (
            _context.proc_addresses.glEndQuery
        )(converted_target) };    result
 
}


#[rustfmt::skip]
pub fn glEndTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glEndTransformFeedback
        )() };    result
 
}


#[rustfmt::skip]
pub fn glFenceSync(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    condition: u32,
    flags: u32,
) -> u32 {
    let converted_condition = condition;
    let converted_flags = flags;
    let result = unsafe { (
            _context.proc_addresses.glFenceSync
        )(converted_condition, converted_flags) };_context.register_opaque_sync_object(    result
) 
}


#[rustfmt::skip]
pub fn glFinish(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glFinish
        )() };    result
 
}


#[rustfmt::skip]
pub fn glFlush(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glFlush
        )() };    result
 
}


#[rustfmt::skip]
pub fn glFlushMappedBufferRange(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    offset: i32,
    length: i32,
) -> () {
    let converted_target = target;
    let converted_offset = offset as isize;
    let converted_length = length as isize;
    let result = unsafe { (
            _context.proc_addresses.glFlushMappedBufferRange
        )(converted_target, converted_offset, converted_length) };    result
 
}


#[rustfmt::skip]
pub fn glFramebufferRenderbuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
) -> () {
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_renderbuffertarget = renderbuffertarget;
    let converted_renderbuffer = renderbuffer;
    let result = unsafe { (
            _context.proc_addresses.glFramebufferRenderbuffer
        )(converted_target, converted_attachment, converted_renderbuffertarget, converted_renderbuffer) };    result
 
}


#[rustfmt::skip]
pub fn glFramebufferTexture2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
) -> () {
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_textarget = textarget;
    let converted_texture = texture;
    let converted_level = level;
    let result = unsafe { (
            _context.proc_addresses.glFramebufferTexture2D
        )(converted_target, converted_attachment, converted_textarget, converted_texture, converted_level) };    result
 
}


#[rustfmt::skip]
pub fn glFramebufferTextureLayer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    attachment: u32,
    texture: u32,
    level: i32,
    layer: i32,
) -> () {
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_texture = texture;
    let converted_level = level;
    let converted_layer = layer;
    let result = unsafe { (
            _context.proc_addresses.glFramebufferTextureLayer
        )(converted_target, converted_attachment, converted_texture, converted_level, converted_layer) };    result
 
}


#[rustfmt::skip]
pub fn glFrontFace(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { (
            _context.proc_addresses.glFrontFace
        )(converted_mode) };    result
 
}


#[rustfmt::skip]
pub fn glGenBuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    buffers: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_buffers = (n) as usize;
    let mut vec_buffers: Vec<std::os::raw::c_uint> = vec![];
    vec_buffers.reserve(len_buffers);
    for i in 0..(len_buffers as u32) {
        vec_buffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    buffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_buffers = vec_buffers.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenBuffers
        )(converted_n, converted_buffers) };    for (i, value) in vec_buffers.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(buffers + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenFramebuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    framebuffers: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_framebuffers = (n) as usize;
    let mut vec_framebuffers: Vec<std::os::raw::c_uint> = vec![];
    vec_framebuffers.reserve(len_framebuffers);
    for i in 0..(len_framebuffers as u32) {
        vec_framebuffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    framebuffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_framebuffers = vec_framebuffers.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenFramebuffers
        )(converted_n, converted_framebuffers) };    for (i, value) in vec_framebuffers.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(framebuffers + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenQueries(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    ids: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_ids = (n) as usize;
    let mut vec_ids: Vec<std::os::raw::c_uint> = vec![];
    vec_ids.reserve(len_ids);
    for i in 0..(len_ids as u32) {
        vec_ids.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    ids + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_ids = vec_ids.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenQueries
        )(converted_n, converted_ids) };    for (i, value) in vec_ids.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(ids + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenRenderbuffers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    renderbuffers: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_renderbuffers = (n) as usize;
    let mut vec_renderbuffers: Vec<std::os::raw::c_uint> = vec![];
    vec_renderbuffers.reserve(len_renderbuffers);
    for i in 0..(len_renderbuffers as u32) {
        vec_renderbuffers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    renderbuffers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_renderbuffers = vec_renderbuffers.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenRenderbuffers
        )(converted_n, converted_renderbuffers) };    for (i, value) in vec_renderbuffers.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(renderbuffers + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenSamplers(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    count: i32,
    samplers: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_count = count;
    let len_samplers = (count) as usize;
    let mut vec_samplers: Vec<std::os::raw::c_uint> = vec![];
    vec_samplers.reserve(len_samplers);
    for i in 0..(len_samplers as u32) {
        vec_samplers.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    samplers + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_samplers = vec_samplers.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenSamplers
        )(converted_count, converted_samplers) };    for (i, value) in vec_samplers.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(samplers + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenTextures(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    textures: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_textures = (n) as usize;
    let mut vec_textures: Vec<std::os::raw::c_uint> = vec![];
    vec_textures.reserve(len_textures);
    for i in 0..(len_textures as u32) {
        vec_textures.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    textures + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_textures = vec_textures.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenTextures
        )(converted_n, converted_textures) };    for (i, value) in vec_textures.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(textures + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenTransformFeedbacks(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    ids: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_ids = (n) as usize;
    let mut vec_ids: Vec<std::os::raw::c_uint> = vec![];
    vec_ids.reserve(len_ids);
    for i in 0..(len_ids as u32) {
        vec_ids.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    ids + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_ids = vec_ids.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenTransformFeedbacks
        )(converted_n, converted_ids) };    for (i, value) in vec_ids.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(ids + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenVertexArrays(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    arrays: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_arrays = (n) as usize;
    let mut vec_arrays: Vec<std::os::raw::c_uint> = vec![];
    vec_arrays.reserve(len_arrays);
    for i in 0..(len_arrays as u32) {
        vec_arrays.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    arrays + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_arrays = vec_arrays.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenVertexArrays
        )(converted_n, converted_arrays) };    for (i, value) in vec_arrays.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(arrays + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenVertexArraysOES(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: i32,
    arrays: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_arrays = (n) as usize;
    let mut vec_arrays: Vec<std::os::raw::c_uint> = vec![];
    vec_arrays.reserve(len_arrays);
    for i in 0..(len_arrays as u32) {
        vec_arrays.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    arrays + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_arrays = vec_arrays.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGenVertexArraysOES
        )(converted_n, converted_arrays) };    for (i, value) in vec_arrays.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(arrays + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGenerateMipmap(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> () {
    let converted_target = target;
    let result = unsafe { (
            _context.proc_addresses.glGenerateMipmap
        )(converted_target) };    result
 
}


#[rustfmt::skip]
pub fn glGetActiveAttrib(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    index: u32,
    bufSize: i32,
    length: u32,
    size: u32,
    _type: u32,
    name: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_size = (1) as usize;
    let mut vec_size: Vec<std::os::raw::c_int> = vec![];
    vec_size.reserve(len_size);
    for i in 0..(len_size as u32) {
        vec_size.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    size + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_size = vec_size.as_mut_ptr() as *mut std::os::raw::c_int;

    let len__type = (1) as usize;
    let mut vec__type: Vec<std::os::raw::c_uint> = vec![];
    vec__type.reserve(len__type);
    for i in 0..(len__type as u32) {
        vec__type.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    _type + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted__type = vec__type.as_mut_ptr() as *mut std::os::raw::c_uint;

    let len_name = (bufSize) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetActiveAttrib
        )(converted_program, converted_index, converted_bufSize, converted_length, converted_size, converted__type, converted_name) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_size.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(size + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec__type.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }
    for (i, value) in vec_name.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(name + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetActiveUniform(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    index: u32,
    bufSize: i32,
    length: u32,
    size: u32,
    _type: u32,
    name: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_size = (1) as usize;
    let mut vec_size: Vec<std::os::raw::c_int> = vec![];
    vec_size.reserve(len_size);
    for i in 0..(len_size as u32) {
        vec_size.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    size + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_size = vec_size.as_mut_ptr() as *mut std::os::raw::c_int;

    let len__type = (1) as usize;
    let mut vec__type: Vec<std::os::raw::c_uint> = vec![];
    vec__type.reserve(len__type);
    for i in 0..(len__type as u32) {
        vec__type.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    _type + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted__type = vec__type.as_mut_ptr() as *mut std::os::raw::c_uint;

    let len_name = (bufSize) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetActiveUniform
        )(converted_program, converted_index, converted_bufSize, converted_length, converted_size, converted__type, converted_name) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_size.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(size + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec__type.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }
    for (i, value) in vec_name.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(name + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetActiveUniformBlockName(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    uniformBlockIndex: u32,
    bufSize: i32,
    length: u32,
    uniformBlockName: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_uniformBlockIndex = uniformBlockIndex;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_uniformBlockName = (bufSize) as usize;
    let mut vec_uniformBlockName: Vec<i8> = vec![];
    vec_uniformBlockName.reserve(len_uniformBlockName);
    for i in 0..(len_uniformBlockName as u32) {
        vec_uniformBlockName.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    uniformBlockName + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_uniformBlockName = vec_uniformBlockName.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetActiveUniformBlockName
        )(converted_program, converted_uniformBlockIndex, converted_bufSize, converted_length, converted_uniformBlockName) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_uniformBlockName.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(uniformBlockName + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetActiveUniformBlockiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    uniformBlockIndex: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_uniformBlockIndex = uniformBlockIndex;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetActiveUniformBlockiv_params_compsize(_context, program,uniformBlockIndex,crate::ffi::GLEnumGroupUniformBlockPName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetActiveUniformBlockiv
        )(converted_program, converted_uniformBlockIndex, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetActiveUniformsiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    uniformCount: i32,
    uniformIndices: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_uniformCount = uniformCount;
    let len_uniformIndices = (uniformCount) as usize;
    let mut vec_uniformIndices: Vec<std::os::raw::c_uint> = vec![];
    vec_uniformIndices.reserve(len_uniformIndices);
    for i in 0..(len_uniformIndices as u32) {
        vec_uniformIndices.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    uniformIndices + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_uniformIndices = vec_uniformIndices.as_mut_ptr() as *const std::os::raw::c_uint;

    let converted_pname = pname;
    let len_params = (crate::compsize::glGetActiveUniformsiv_params_compsize(_context, uniformCount,crate::ffi::GLEnumGroupUniformPName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetActiveUniformsiv
        )(converted_program, converted_uniformCount, converted_uniformIndices, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetAttachedShaders(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    maxCount: i32,
    count: u32,
    shaders: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_maxCount = maxCount;
    let len_count = (1) as usize;
    let mut vec_count: Vec<std::os::raw::c_int> = vec![];
    vec_count.reserve(len_count);
    for i in 0..(len_count as u32) {
        vec_count.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    count + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_count = vec_count.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_shaders = (maxCount) as usize;
    let mut vec_shaders: Vec<std::os::raw::c_uint> = vec![];
    vec_shaders.reserve(len_shaders);
    for i in 0..(len_shaders as u32) {
        vec_shaders.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    shaders + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_shaders = vec_shaders.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGetAttachedShaders
        )(converted_program, converted_maxCount, converted_count, converted_shaders) };    for (i, value) in vec_count.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(count + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_shaders.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(shaders + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetAttribLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    name: u32,
) -> i32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let len_name = (crate::utils::guest_strlen(&memory, name) + 1) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *const i8;

    let result = unsafe { (
            _context.proc_addresses.glGetAttribLocation
        )(converted_program, converted_name) };    result
.into() 
}


#[rustfmt::skip]
pub fn glGetBooleanv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetBooleanv_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(pname))) as usize;
    let mut vec_data: Vec<u8> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    data + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut u8;

    let result = unsafe { (
            _context.proc_addresses.glGetBooleanv
        )(converted_pname, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<u8>(
                webrogue_runtime::wiggle::GuestPtr::<u8>::new(data + (i as u32) * 1),
                *value as u8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetBufferParameteri64v(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetBufferParameteri64v_params_compsize(_context, crate::ffi::GLEnumGroupBufferPNameARB::from_raw(pname))) as usize;
    let mut vec_params: Vec<i64> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i64>(webrogue_runtime::wiggle::GuestPtr::<i64>::new(
                    params + i * 8,
                ))
                .unwrap() as i64,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut i64;

    let result = unsafe { (
            _context.proc_addresses.glGetBufferParameteri64v
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i64>(
                webrogue_runtime::wiggle::GuestPtr::<i64>::new(params + (i as u32) * 8),
                *value as i64,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetBufferParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetBufferParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupBufferPNameARB::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetBufferParameteriv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetBufferPointerv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (1) as usize;
    let mut vec_params: Vec<*mut ()> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    params + i * 4,
                ))
                .unwrap() as *mut (),
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut *mut ();

    let result = unsafe { (
            _context.proc_addresses.glGetBufferPointerv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(params + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetError(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {

    let result = unsafe { (
            _context.proc_addresses.glGetError
        )() };    result
.into() 
}


#[rustfmt::skip]
pub fn glGetFloatv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetFloatv_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(pname))) as usize;
    let mut vec_data: Vec<f32> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    data + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut f32;

    let result = unsafe { (
            _context.proc_addresses.glGetFloatv
        )(converted_pname, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<f32>(
                webrogue_runtime::wiggle::GuestPtr::<f32>::new(data + (i as u32) * 4),
                *value as f32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetFragDataLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    name: u32,
) -> i32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let len_name = (crate::utils::guest_strlen(&memory, name) + 1) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *const i8;

    let result = unsafe { (
            _context.proc_addresses.glGetFragDataLocation
        )(converted_program, converted_name) };    result
.into() 
}


#[rustfmt::skip]
pub fn glGetFramebufferAttachmentParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    attachment: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetFramebufferAttachmentParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupFramebufferAttachmentParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetFramebufferAttachmentParameteriv
        )(converted_target, converted_attachment, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetInteger64i_v(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    index: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_index = index;
    let len_data = (crate::compsize::glGetInteger64i_v_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(target))) as usize;
    let mut vec_data: Vec<i64> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<i64>(webrogue_runtime::wiggle::GuestPtr::<i64>::new(
                    data + i * 8,
                ))
                .unwrap() as i64,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut i64;

    let result = unsafe { (
            _context.proc_addresses.glGetInteger64i_v
        )(converted_target, converted_index, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<i64>(
                webrogue_runtime::wiggle::GuestPtr::<i64>::new(data + (i as u32) * 8),
                *value as i64,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetInteger64v(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetInteger64v_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(pname))) as usize;
    let mut vec_data: Vec<i64> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<i64>(webrogue_runtime::wiggle::GuestPtr::<i64>::new(
                    data + i * 8,
                ))
                .unwrap() as i64,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut i64;

    let result = unsafe { (
            _context.proc_addresses.glGetInteger64v
        )(converted_pname, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<i64>(
                webrogue_runtime::wiggle::GuestPtr::<i64>::new(data + (i as u32) * 8),
                *value as i64,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetIntegeri_v(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    index: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_index = index;
    let len_data = (crate::compsize::glGetIntegeri_v_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(target))) as usize;
    let mut vec_data: Vec<std::os::raw::c_int> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    data + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetIntegeri_v
        )(converted_target, converted_index, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(data + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetIntegerv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    data: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetIntegerv_data_compsize(_context, crate::ffi::GLEnumGroupGetPName::from_raw(pname))) as usize;
    let mut vec_data: Vec<std::os::raw::c_int> = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32) {
        vec_data.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    data + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetIntegerv
        )(converted_pname, converted_data) };    for (i, value) in vec_data.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(data + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetInternalformativ(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    internalformat: u32,
    pname: u32,
    count: i32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_internalformat = internalformat;
    let converted_pname = pname;
    let converted_count = count;
    let len_params = (count) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetInternalformativ
        )(converted_target, converted_internalformat, converted_pname, converted_count, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetProgramBinary(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    bufSize: i32,
    length: u32,
    binaryFormat: u32,
    binary: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_binaryFormat = (1) as usize;
    let mut vec_binaryFormat: Vec<std::os::raw::c_uint> = vec![];
    vec_binaryFormat.reserve(len_binaryFormat);
    for i in 0..(len_binaryFormat as u32) {
        vec_binaryFormat.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    binaryFormat + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_binaryFormat = vec_binaryFormat.as_mut_ptr() as *mut std::os::raw::c_uint;

    let len_binary = (bufSize) as usize;
    let mut vec_binary: Vec<u8> = vec![];
    vec_binary.reserve(len_binary);
    for i in 0..(len_binary as u32) {
        vec_binary.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    binary + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_binary = vec_binary.as_mut_ptr() as *mut ();

    let result = unsafe { (
            _context.proc_addresses.glGetProgramBinary
        )(converted_program, converted_bufSize, converted_length, converted_binaryFormat, converted_binary) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_binaryFormat.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(binaryFormat + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }
    for (i, value) in vec_binary.iter().enumerate() {
        memory
            .write::<u8>(
                webrogue_runtime::wiggle::GuestPtr::<u8>::new(binary + (i as u32) * 1),
                *value as u8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetProgramInfoLog(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    bufSize: i32,
    length: u32,
    infoLog: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_infoLog = (bufSize) as usize;
    let mut vec_infoLog: Vec<i8> = vec![];
    vec_infoLog.reserve(len_infoLog);
    for i in 0..(len_infoLog as u32) {
        vec_infoLog.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    infoLog + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_infoLog = vec_infoLog.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetProgramInfoLog
        )(converted_program, converted_bufSize, converted_length, converted_infoLog) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_infoLog.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetProgramiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetProgramiv_params_compsize(_context, crate::ffi::GLEnumGroupProgramPropertyARB::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetProgramiv
        )(converted_program, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetQueryObjectuiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    id: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_id = id;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetQueryObjectuiv_params_compsize(_context, crate::ffi::GLEnumGroupQueryObjectParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_uint> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGetQueryObjectuiv
        )(converted_id, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(params + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetQueryiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetQueryiv_params_compsize(_context, crate::ffi::GLEnumGroupQueryParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetQueryiv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetRenderbufferParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetRenderbufferParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupRenderbufferParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetRenderbufferParameteriv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetSamplerParameterfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_sampler = sampler;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetSamplerParameterfv_params_compsize(_context, crate::ffi::GLEnumGroupSamplerParameterF::from_raw(pname))) as usize;
    let mut vec_params: Vec<f32> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    params + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut f32;

    let result = unsafe { (
            _context.proc_addresses.glGetSamplerParameterfv
        )(converted_sampler, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<f32>(
                webrogue_runtime::wiggle::GuestPtr::<f32>::new(params + (i as u32) * 4),
                *value as f32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetSamplerParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_sampler = sampler;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetSamplerParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupSamplerParameterI::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetSamplerParameteriv
        )(converted_sampler, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetShaderInfoLog(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
    bufSize: i32,
    length: u32,
    infoLog: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_infoLog = (bufSize) as usize;
    let mut vec_infoLog: Vec<i8> = vec![];
    vec_infoLog.reserve(len_infoLog);
    for i in 0..(len_infoLog as u32) {
        vec_infoLog.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    infoLog + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_infoLog = vec_infoLog.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetShaderInfoLog
        )(converted_shader, converted_bufSize, converted_length, converted_infoLog) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_infoLog.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetShaderPrecisionFormat(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shadertype: u32,
    precisiontype: u32,
    range: u32,
    precision: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_shadertype = shadertype;
    let converted_precisiontype = precisiontype;
    let len_range = (2) as usize;
    let mut vec_range: Vec<std::os::raw::c_int> = vec![];
    vec_range.reserve(len_range);
    for i in 0..(len_range as u32) {
        vec_range.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    range + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_range = vec_range.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_precision = (1) as usize;
    let mut vec_precision: Vec<std::os::raw::c_int> = vec![];
    vec_precision.reserve(len_precision);
    for i in 0..(len_precision as u32) {
        vec_precision.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    precision + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_precision = vec_precision.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetShaderPrecisionFormat
        )(converted_shadertype, converted_precisiontype, converted_range, converted_precision) };    for (i, value) in vec_range.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(range + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_precision.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(precision + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetShaderSource(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
    bufSize: i32,
    length: u32,
    source: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_source = (bufSize) as usize;
    let mut vec_source: Vec<i8> = vec![];
    vec_source.reserve(len_source);
    for i in 0..(len_source as u32) {
        vec_source.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    source + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_source = vec_source.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetShaderSource
        )(converted_shader, converted_bufSize, converted_length, converted_source) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_source.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(source + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetShaderiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetShaderiv_params_compsize(_context, crate::ffi::GLEnumGroupShaderParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetShaderiv
        )(converted_shader, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetSynciv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sync: u32,
    pname: u32,
    count: i32,
    length: u32,
    values: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_sync = _context.resolve_opaque_sync_object(sync);
    let converted_pname = pname;
    let converted_count = count;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_values = (count) as usize;
    let mut vec_values: Vec<std::os::raw::c_int> = vec![];
    vec_values.reserve(len_values);
    for i in 0..(len_values as u32) {
        vec_values.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    values + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_values = vec_values.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetSynciv
        )(converted_sync, converted_pname, converted_count, converted_length, converted_values) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_values.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(values + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetTexParameterfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetTexParameterfv_params_compsize(_context, crate::ffi::GLEnumGroupGetTextureParameter::from_raw(pname))) as usize;
    let mut vec_params: Vec<f32> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    params + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut f32;

    let result = unsafe { (
            _context.proc_addresses.glGetTexParameterfv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<f32>(
                webrogue_runtime::wiggle::GuestPtr::<f32>::new(params + (i as u32) * 4),
                *value as f32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetTexParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetTexParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupGetTextureParameter::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetTexParameteriv
        )(converted_target, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetTransformFeedbackVarying(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    index: u32,
    bufSize: i32,
    length: u32,
    size: u32,
    _type: u32,
    name: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let converted_bufSize = bufSize;
    let len_length = (1) as usize;
    let mut vec_length: Vec<std::os::raw::c_int> = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32) {
        vec_length.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    length + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_length = vec_length.as_mut_ptr() as *mut std::os::raw::c_int;

    let len_size = (1) as usize;
    let mut vec_size: Vec<std::os::raw::c_int> = vec![];
    vec_size.reserve(len_size);
    for i in 0..(len_size as u32) {
        vec_size.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    size + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_size = vec_size.as_mut_ptr() as *mut std::os::raw::c_int;

    let len__type = (1) as usize;
    let mut vec__type: Vec<std::os::raw::c_uint> = vec![];
    vec__type.reserve(len__type);
    for i in 0..(len__type as u32) {
        vec__type.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    _type + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted__type = vec__type.as_mut_ptr() as *mut std::os::raw::c_uint;

    let len_name = (bufSize) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *mut i8;

    let result = unsafe { (
            _context.proc_addresses.glGetTransformFeedbackVarying
        )(converted_program, converted_index, converted_bufSize, converted_length, converted_size, converted__type, converted_name) };    for (i, value) in vec_length.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(length + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec_size.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(size + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    for (i, value) in vec__type.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }
    for (i, value) in vec_name.iter().enumerate() {
        memory
            .write::<i8>(
                webrogue_runtime::wiggle::GuestPtr::<i8>::new(name + (i as u32) * 1),
                *value as i8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetUniformBlockIndex(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    uniformBlockName: u32,
) -> u32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let len_uniformBlockName = (crate::utils::guest_strlen(&memory, uniformBlockName) + 1) as usize;
    let mut vec_uniformBlockName: Vec<i8> = vec![];
    vec_uniformBlockName.reserve(len_uniformBlockName);
    for i in 0..(len_uniformBlockName as u32) {
        vec_uniformBlockName.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    uniformBlockName + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_uniformBlockName = vec_uniformBlockName.as_mut_ptr() as *const i8;

    let result = unsafe { (
            _context.proc_addresses.glGetUniformBlockIndex
        )(converted_program, converted_uniformBlockName) };    result
.into() 
}


#[rustfmt::skip]
pub fn glGetUniformLocation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    name: u32,
) -> i32 {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let len_name = (crate::utils::guest_strlen(&memory, name) + 1) as usize;
    let mut vec_name: Vec<i8> = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32) {
        vec_name.push(
            memory
                .read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(
                    name + i * 1,
                ))
                .unwrap() as i8,
        );
    }
    let converted_name = vec_name.as_mut_ptr() as *const i8;

    let result = unsafe { (
            _context.proc_addresses.glGetUniformLocation
        )(converted_program, converted_name) };    result
.into() 
}


#[rustfmt::skip]
pub fn glGetUniformfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    location: i32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_location = location;
    let len_params = (crate::compsize::glGetUniformfv_params_compsize(_context, program,location)) as usize;
    let mut vec_params: Vec<f32> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    params + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut f32;

    let result = unsafe { (
            _context.proc_addresses.glGetUniformfv
        )(converted_program, converted_location, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<f32>(
                webrogue_runtime::wiggle::GuestPtr::<f32>::new(params + (i as u32) * 4),
                *value as f32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetUniformiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    location: i32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_location = location;
    let len_params = (crate::compsize::glGetUniformiv_params_compsize(_context, program,location)) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetUniformiv
        )(converted_program, converted_location, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetUniformuiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    location: i32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_location = location;
    let len_params = (crate::compsize::glGetUniformuiv_params_compsize(_context, program,location)) as usize;
    let mut vec_params: Vec<std::os::raw::c_uint> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGetUniformuiv
        )(converted_program, converted_location, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(params + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetVertexAttribIiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (1) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetVertexAttribIiv
        )(converted_index, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetVertexAttribIuiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (1) as usize;
    let mut vec_params: Vec<std::os::raw::c_uint> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glGetVertexAttribIuiv
        )(converted_index, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(params + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetVertexAttribPointerv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    pname: u32,
    pointer: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_pointer = (1) as usize;
    let mut vec_pointer: Vec<*mut ()> = vec![];
    vec_pointer.reserve(len_pointer);
    for i in 0..(len_pointer as u32) {
        vec_pointer.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    pointer + i * 4,
                ))
                .unwrap() as *mut (),
        );
    }
    let converted_pointer = vec_pointer.as_mut_ptr() as *mut *mut ();

    let result = unsafe { (
            _context.proc_addresses.glGetVertexAttribPointerv
        )(converted_index, converted_pname, converted_pointer) };    for (i, value) in vec_pointer.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(pointer + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetVertexAttribfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (4) as usize;
    let mut vec_params: Vec<f32> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    params + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut f32;

    let result = unsafe { (
            _context.proc_addresses.glGetVertexAttribfv
        )(converted_index, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<f32>(
                webrogue_runtime::wiggle::GuestPtr::<f32>::new(params + (i as u32) * 4),
                *value as f32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glGetVertexAttribiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    pname: u32,
    params: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (4) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *mut std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glGetVertexAttribiv
        )(converted_index, converted_pname, converted_params) };    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glHint(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    mode: u32,
) -> () {
    let converted_target = target;
    let converted_mode = mode;
    let result = unsafe { (
            _context.proc_addresses.glHint
        )(converted_target, converted_mode) };    result
 
}


#[rustfmt::skip]
pub fn glInvalidateFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    numAttachments: i32,
    attachments: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_numAttachments = numAttachments;
    let len_attachments = (numAttachments) as usize;
    let mut vec_attachments: Vec<std::os::raw::c_uint> = vec![];
    vec_attachments.reserve(len_attachments);
    for i in 0..(len_attachments as u32) {
        vec_attachments.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    attachments + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_attachments = vec_attachments.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glInvalidateFramebuffer
        )(converted_target, converted_numAttachments, converted_attachments) };    result
 
}


#[rustfmt::skip]
pub fn glInvalidateSubFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    numAttachments: i32,
    attachments: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_numAttachments = numAttachments;
    let len_attachments = (numAttachments) as usize;
    let mut vec_attachments: Vec<std::os::raw::c_uint> = vec![];
    vec_attachments.reserve(len_attachments);
    for i in 0..(len_attachments as u32) {
        vec_attachments.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    attachments + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_attachments = vec_attachments.as_mut_ptr() as *const std::os::raw::c_uint;

    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glInvalidateSubFramebuffer
        )(converted_target, converted_numAttachments, converted_attachments, converted_x, converted_y, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glIsBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
) -> u32 {
    let converted_buffer = buffer;
    let result = unsafe { (
            _context.proc_addresses.glIsBuffer
        )(converted_buffer) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsEnabled(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> u32 {
    let converted_cap = cap;
    let result = unsafe { (
            _context.proc_addresses.glIsEnabled
        )(converted_cap) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    framebuffer: u32,
) -> u32 {
    let converted_framebuffer = framebuffer;
    let result = unsafe { (
            _context.proc_addresses.glIsFramebuffer
        )(converted_framebuffer) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> u32 {
    let converted_program = program;
    let result = unsafe { (
            _context.proc_addresses.glIsProgram
        )(converted_program) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsQuery(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    id: u32,
) -> u32 {
    let converted_id = id;
    let result = unsafe { (
            _context.proc_addresses.glIsQuery
        )(converted_id) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsRenderbuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    renderbuffer: u32,
) -> u32 {
    let converted_renderbuffer = renderbuffer;
    let result = unsafe { (
            _context.proc_addresses.glIsRenderbuffer
        )(converted_renderbuffer) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsSampler(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
) -> u32 {
    let converted_sampler = sampler;
    let result = unsafe { (
            _context.proc_addresses.glIsSampler
        )(converted_sampler) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> u32 {
    let converted_shader = shader;
    let result = unsafe { (
            _context.proc_addresses.glIsShader
        )(converted_shader) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsSync(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sync: u32,
) -> u32 {
    let converted_sync = _context.resolve_opaque_sync_object(sync);
    let result = unsafe { (
            _context.proc_addresses.glIsSync
        )(converted_sync) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    texture: u32,
) -> u32 {
    let converted_texture = texture;
    let result = unsafe { (
            _context.proc_addresses.glIsTexture
        )(converted_texture) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    id: u32,
) -> u32 {
    let converted_id = id;
    let result = unsafe { (
            _context.proc_addresses.glIsTransformFeedback
        )(converted_id) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsVertexArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    array: u32,
) -> u32 {
    let converted_array = array;
    let result = unsafe { (
            _context.proc_addresses.glIsVertexArray
        )(converted_array) };    result
.into() 
}


#[rustfmt::skip]
pub fn glIsVertexArrayOES(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    array: u32,
) -> u32 {
    let converted_array = array;
    let result = unsafe { (
            _context.proc_addresses.glIsVertexArrayOES
        )(converted_array) };    result
.into() 
}


#[rustfmt::skip]
pub fn glLineWidth(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    width: f32,
) -> () {
    let converted_width = width;
    let result = unsafe { (
            _context.proc_addresses.glLineWidth
        )(converted_width) };    result
 
}


#[rustfmt::skip]
pub fn glLinkProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { (
            _context.proc_addresses.glLinkProgram
        )(converted_program) };    result
 
}


#[rustfmt::skip]
pub fn glPauseTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glPauseTransformFeedback
        )() };    result
 
}


#[rustfmt::skip]
pub fn glPixelStorei(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    param: i32,
) -> () {
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { (
            _context.proc_addresses.glPixelStorei
        )(converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glPolygonOffset(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    factor: f32,
    units: f32,
) -> () {
    let converted_factor = factor;
    let converted_units = units;
    let result = unsafe { (
            _context.proc_addresses.glPolygonOffset
        )(converted_factor, converted_units) };    result
 
}


#[rustfmt::skip]
pub fn glProgramBinary(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    binaryFormat: u32,
    binary: u32,
    length: i32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_binaryFormat = binaryFormat;
    let len_binary = (length) as usize;
    let mut vec_binary: Vec<u8> = vec![];
    vec_binary.reserve(len_binary);
    for i in 0..(len_binary as u32) {
        vec_binary.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    binary + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_binary = vec_binary.as_mut_ptr() as *const ();

    let converted_length = length;
    let result = unsafe { (
            _context.proc_addresses.glProgramBinary
        )(converted_program, converted_binaryFormat, converted_binary, converted_length) };    result
 
}


#[rustfmt::skip]
pub fn glProgramParameteri(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    pname: u32,
    value: i32,
) -> () {
    let converted_program = program;
    let converted_pname = pname;
    let converted_value = value;
    let result = unsafe { (
            _context.proc_addresses.glProgramParameteri
        )(converted_program, converted_pname, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glReadBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    src: u32,
) -> () {
    let converted_src = src;
    let result = unsafe { (
            _context.proc_addresses.glReadBuffer
        )(converted_src) };    result
 
}


#[rustfmt::skip]
pub fn glReadPixels(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    format: u32,
    _type: u32,
    pixels: u32,
) -> () {
    let mut memory = _memory_factory.make_memory();
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glReadPixels_pixels_compsize(_context, crate::ffi::GLEnumGroupPixelFormat::from_raw(format),crate::ffi::GLEnumGroupPixelType::from_raw(_type),width,height)) as usize;
    let mut vec_pixels: Vec<u8> = vec![];
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
    let converted_pixels = vec_pixels.as_mut_ptr() as *mut ();

    let result = unsafe { (
            _context.proc_addresses.glReadPixels
        )(converted_x, converted_y, converted_width, converted_height, converted_format, converted__type, converted_pixels) };    for (i, value) in vec_pixels.iter().enumerate() {
        memory
            .write::<u8>(
                webrogue_runtime::wiggle::GuestPtr::<u8>::new(pixels + (i as u32) * 1),
                *value as u8,
            )
            .unwrap()
    }    result
 
}


#[rustfmt::skip]
pub fn glReleaseShaderCompiler(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glReleaseShaderCompiler
        )() };    result
 
}


#[rustfmt::skip]
pub fn glRenderbufferStorage(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    internalformat: u32,
    width: i32,
    height: i32,
) -> () {
    let converted_target = target;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glRenderbufferStorage
        )(converted_target, converted_internalformat, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glRenderbufferStorageMultisample(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    samples: i32,
    internalformat: u32,
    width: i32,
    height: i32,
) -> () {
    let converted_target = target;
    let converted_samples = samples;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glRenderbufferStorageMultisample
        )(converted_target, converted_samples, converted_internalformat, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glResumeTransformFeedback(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {

    let result = unsafe { (
            _context.proc_addresses.glResumeTransformFeedback
        )() };    result
 
}


#[rustfmt::skip]
pub fn glSampleCoverage(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    value: f32,
    invert: u32,
) -> () {
    let converted_value = value;
    let converted_invert = invert as u8;
    let result = unsafe { (
            _context.proc_addresses.glSampleCoverage
        )(converted_value, converted_invert) };    result
 
}


#[rustfmt::skip]
pub fn glSamplerParameterf(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    param: f32,
) -> () {
    let converted_sampler = sampler;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { (
            _context.proc_addresses.glSamplerParameterf
        )(converted_sampler, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glSamplerParameterfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    param: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_sampler = sampler;
    let converted_pname = pname;
    let len_param = (crate::compsize::glSamplerParameterfv_param_compsize(_context, crate::ffi::GLEnumGroupSamplerParameterF::from_raw(pname))) as usize;
    let mut vec_param: Vec<f32> = vec![];
    vec_param.reserve(len_param);
    for i in 0..(len_param as u32) {
        vec_param.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    param + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_param = vec_param.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glSamplerParameterfv
        )(converted_sampler, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glSamplerParameteri(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    param: i32,
) -> () {
    let converted_sampler = sampler;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { (
            _context.proc_addresses.glSamplerParameteri
        )(converted_sampler, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glSamplerParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sampler: u32,
    pname: u32,
    param: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_sampler = sampler;
    let converted_pname = pname;
    let len_param = (crate::compsize::glSamplerParameteriv_param_compsize(_context, crate::ffi::GLEnumGroupSamplerParameterI::from_raw(pname))) as usize;
    let mut vec_param: Vec<std::os::raw::c_int> = vec![];
    vec_param.reserve(len_param);
    for i in 0..(len_param as u32) {
        vec_param.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    param + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_param = vec_param.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glSamplerParameteriv
        )(converted_sampler, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glScissor(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> () {
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glScissor
        )(converted_x, converted_y, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glShaderBinary(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    count: i32,
    shaders: u32,
    binaryFormat: u32,
    binary: u32,
    length: i32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_count = count;
    let len_shaders = (count) as usize;
    let mut vec_shaders: Vec<std::os::raw::c_uint> = vec![];
    vec_shaders.reserve(len_shaders);
    for i in 0..(len_shaders as u32) {
        vec_shaders.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    shaders + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_shaders = vec_shaders.as_mut_ptr() as *const std::os::raw::c_uint;

    let converted_binaryFormat = binaryFormat;
    let len_binary = (length) as usize;
    let mut vec_binary: Vec<u8> = vec![];
    vec_binary.reserve(len_binary);
    for i in 0..(len_binary as u32) {
        vec_binary.push(
            memory
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(
                    binary + i * 1,
                ))
                .unwrap() as u8,
        );
    }
    let converted_binary = vec_binary.as_mut_ptr() as *const ();

    let converted_length = length;
    let result = unsafe { (
            _context.proc_addresses.glShaderBinary
        )(converted_count, converted_shaders, converted_binaryFormat, converted_binary, converted_length) };    result
 
}


#[rustfmt::skip]
pub fn glStencilFunc(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    func: u32,
    _ref: i32,
    mask: u32,
) -> () {
    let converted_func = func;
    let converted__ref = _ref;
    let converted_mask = mask;
    let result = unsafe { (
            _context.proc_addresses.glStencilFunc
        )(converted_func, converted__ref, converted_mask) };    result
 
}


#[rustfmt::skip]
pub fn glStencilFuncSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    face: u32,
    func: u32,
    _ref: i32,
    mask: u32,
) -> () {
    let converted_face = face;
    let converted_func = func;
    let converted__ref = _ref;
    let converted_mask = mask;
    let result = unsafe { (
            _context.proc_addresses.glStencilFuncSeparate
        )(converted_face, converted_func, converted__ref, converted_mask) };    result
 
}


#[rustfmt::skip]
pub fn glStencilMask(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mask: u32,
) -> () {
    let converted_mask = mask;
    let result = unsafe { (
            _context.proc_addresses.glStencilMask
        )(converted_mask) };    result
 
}


#[rustfmt::skip]
pub fn glStencilMaskSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    face: u32,
    mask: u32,
) -> () {
    let converted_face = face;
    let converted_mask = mask;
    let result = unsafe { (
            _context.proc_addresses.glStencilMaskSeparate
        )(converted_face, converted_mask) };    result
 
}


#[rustfmt::skip]
pub fn glStencilOp(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    fail: u32,
    zfail: u32,
    zpass: u32,
) -> () {
    let converted_fail = fail;
    let converted_zfail = zfail;
    let converted_zpass = zpass;
    let result = unsafe { (
            _context.proc_addresses.glStencilOp
        )(converted_fail, converted_zfail, converted_zpass) };    result
 
}


#[rustfmt::skip]
pub fn glStencilOpSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    face: u32,
    sfail: u32,
    dpfail: u32,
    dppass: u32,
) -> () {
    let converted_face = face;
    let converted_sfail = sfail;
    let converted_dpfail = dpfail;
    let converted_dppass = dppass;
    let result = unsafe { (
            _context.proc_addresses.glStencilOpSeparate
        )(converted_face, converted_sfail, converted_dpfail, converted_dppass) };    result
 
}


#[rustfmt::skip]
pub fn glTexImage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    internalformat: i32,
    width: i32,
    height: i32,
    depth: i32,
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
    let converted_depth = depth;
    let converted_border = border;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glTexImage3D_pixels_compsize(_context, crate::ffi::GLEnumGroupPixelFormat::from_raw(format),crate::ffi::GLEnumGroupPixelType::from_raw(_type),width,height,depth)) as usize;
    let mut vec_pixels: Vec<u8> = vec![];
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
    let converted_pixels = vec_pixels.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glTexImage3D
        )(converted_target, converted_level, converted_internalformat, converted_width, converted_height, converted_depth, converted_border, converted_format, converted__type, converted_pixels) };    result
 
}


#[rustfmt::skip]
pub fn glTexParameterf(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    param: f32,
) -> () {
    let converted_target = target;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { (
            _context.proc_addresses.glTexParameterf
        )(converted_target, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glTexParameterfv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glTexParameterfv_params_compsize(_context, crate::ffi::GLEnumGroupTextureParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<f32> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    params + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glTexParameterfv
        )(converted_target, converted_pname, converted_params) };    result
 
}


#[rustfmt::skip]
pub fn glTexParameteri(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    param: i32,
) -> () {
    let converted_target = target;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { (
            _context.proc_addresses.glTexParameteri
        )(converted_target, converted_pname, converted_param) };    result
 
}


#[rustfmt::skip]
pub fn glTexParameteriv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    pname: u32,
    params: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glTexParameteriv_params_compsize(_context, crate::ffi::GLEnumGroupTextureParameterName::from_raw(pname))) as usize;
    let mut vec_params: Vec<std::os::raw::c_int> = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32) {
        vec_params.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    params + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_params = vec_params.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glTexParameteriv
        )(converted_target, converted_pname, converted_params) };    result
 
}


#[rustfmt::skip]
pub fn glTexStorage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
) -> () {
    let converted_target = target;
    let converted_levels = levels;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glTexStorage2D
        )(converted_target, converted_levels, converted_internalformat, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glTexStorage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    levels: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    depth: i32,
) -> () {
    let converted_target = target;
    let converted_levels = levels;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let converted_depth = depth;
    let result = unsafe { (
            _context.proc_addresses.glTexStorage3D
        )(converted_target, converted_levels, converted_internalformat, converted_width, converted_height, converted_depth) };    result
 
}


#[rustfmt::skip]
pub fn glTexSubImage2D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    width: i32,
    height: i32,
    format: u32,
    _type: u32,
    pixels: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glTexSubImage2D_pixels_compsize(_context, crate::ffi::GLEnumGroupPixelFormat::from_raw(format),crate::ffi::GLEnumGroupPixelType::from_raw(_type),width,height)) as usize;
    let mut vec_pixels: Vec<u8> = vec![];
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
    let converted_pixels = vec_pixels.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glTexSubImage2D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_width, converted_height, converted_format, converted__type, converted_pixels) };    result
 
}


#[rustfmt::skip]
pub fn glTexSubImage3D(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    level: i32,
    xoffset: i32,
    yoffset: i32,
    zoffset: i32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
    _type: u32,
    pixels: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_zoffset = zoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_depth = depth;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glTexSubImage3D_pixels_compsize(_context, crate::ffi::GLEnumGroupPixelFormat::from_raw(format),crate::ffi::GLEnumGroupPixelType::from_raw(_type),width,height,depth)) as usize;
    let mut vec_pixels: Vec<u8> = vec![];
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
    let converted_pixels = vec_pixels.as_mut_ptr() as *const ();

    let result = unsafe { (
            _context.proc_addresses.glTexSubImage3D
        )(converted_target, converted_level, converted_xoffset, converted_yoffset, converted_zoffset, converted_width, converted_height, converted_depth, converted_format, converted__type, converted_pixels) };    result
 
}


#[rustfmt::skip]
pub fn glTransformFeedbackVaryings(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    count: i32,
    varyings: u32,
    bufferMode: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_count = count;
    let len_varyings = (count) as usize;
    let mut vec_varyings: Vec<*const i8> = vec![];
    vec_varyings.reserve(len_varyings);
    for i in 0..(len_varyings as u32) {
        vec_varyings.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    varyings + i * 4,
                ))
                .unwrap() as *const i8,
        );
    }
    let converted_varyings = vec_varyings.as_mut_ptr() as *const *const i8;

    let converted_bufferMode = bufferMode;
    let result = unsafe { (
            _context.proc_addresses.glTransformFeedbackVaryings
        )(converted_program, converted_count, converted_varyings, converted_bufferMode) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: f32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe { (
            _context.proc_addresses.glUniform1f
        )(converted_location, converted_v0) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*1) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniform1fv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: i32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe { (
            _context.proc_addresses.glUniform1i
        )(converted_location, converted_v0) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1iv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*1) as usize;
    let mut vec_value: Vec<std::os::raw::c_int> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glUniform1iv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1ui(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: u32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe { (
            _context.proc_addresses.glUniform1ui
        )(converted_location, converted_v0) };    result
 
}


#[rustfmt::skip]
pub fn glUniform1uiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*1) as usize;
    let mut vec_value: Vec<std::os::raw::c_uint> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glUniform1uiv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: f32,
    v1: f32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let result = unsafe { (
            _context.proc_addresses.glUniform2f
        )(converted_location, converted_v0, converted_v1) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*2) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniform2fv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: i32,
    v1: i32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let result = unsafe { (
            _context.proc_addresses.glUniform2i
        )(converted_location, converted_v0, converted_v1) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2iv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*2) as usize;
    let mut vec_value: Vec<std::os::raw::c_int> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glUniform2iv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2ui(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: u32,
    v1: u32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let result = unsafe { (
            _context.proc_addresses.glUniform2ui
        )(converted_location, converted_v0, converted_v1) };    result
 
}


#[rustfmt::skip]
pub fn glUniform2uiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*2) as usize;
    let mut vec_value: Vec<std::os::raw::c_uint> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glUniform2uiv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let result = unsafe { (
            _context.proc_addresses.glUniform3f
        )(converted_location, converted_v0, converted_v1, converted_v2) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*3) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniform3fv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let result = unsafe { (
            _context.proc_addresses.glUniform3i
        )(converted_location, converted_v0, converted_v1, converted_v2) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3iv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*3) as usize;
    let mut vec_value: Vec<std::os::raw::c_int> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glUniform3iv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3ui(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let result = unsafe { (
            _context.proc_addresses.glUniform3ui
        )(converted_location, converted_v0, converted_v1, converted_v2) };    result
 
}


#[rustfmt::skip]
pub fn glUniform3uiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*3) as usize;
    let mut vec_value: Vec<std::os::raw::c_uint> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glUniform3uiv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: f32,
    v1: f32,
    v2: f32,
    v3: f32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let converted_v3 = v3;
    let result = unsafe { (
            _context.proc_addresses.glUniform4f
        )(converted_location, converted_v0, converted_v1, converted_v2, converted_v3) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*4) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniform4fv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: i32,
    v1: i32,
    v2: i32,
    v3: i32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let converted_v3 = v3;
    let result = unsafe { (
            _context.proc_addresses.glUniform4i
        )(converted_location, converted_v0, converted_v1, converted_v2, converted_v3) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4iv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*4) as usize;
    let mut vec_value: Vec<std::os::raw::c_int> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glUniform4iv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4ui(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let converted_v3 = v3;
    let result = unsafe { (
            _context.proc_addresses.glUniform4ui
        )(converted_location, converted_v0, converted_v1, converted_v2, converted_v3) };    result
 
}


#[rustfmt::skip]
pub fn glUniform4uiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*4) as usize;
    let mut vec_value: Vec<std::os::raw::c_uint> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    value + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glUniform4uiv
        )(converted_location, converted_count, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformBlockBinding(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    uniformBlockIndex: u32,
    uniformBlockBinding: u32,
) -> () {
    let converted_program = program;
    let converted_uniformBlockIndex = uniformBlockIndex;
    let converted_uniformBlockBinding = uniformBlockBinding;
    let result = unsafe { (
            _context.proc_addresses.glUniformBlockBinding
        )(converted_program, converted_uniformBlockIndex, converted_uniformBlockBinding) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix2fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*4) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix2fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix2x3fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*6) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix2x3fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix2x4fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*8) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix2x4fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix3fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*9) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix3fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix3x2fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*6) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix3x2fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix3x4fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*12) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix3x4fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix4fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*16) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix4fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix4x2fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*8) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix4x2fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUniformMatrix4x3fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    count: i32,
    transpose: u32,
    value: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*12) as usize;
    let mut vec_value: Vec<f32> = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32) {
        vec_value.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    value + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_value = vec_value.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glUniformMatrix4x3fv
        )(converted_location, converted_count, converted_transpose, converted_value) };    result
 
}


#[rustfmt::skip]
pub fn glUnmapBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> u32 {
    let converted_target = target;
    let result = unsafe { (
            _context.proc_addresses.glUnmapBuffer
        )(converted_target) };    result
.into() 
}


#[rustfmt::skip]
pub fn glUseProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { (
            _context.proc_addresses.glUseProgram
        )(converted_program) };    result
 
}


#[rustfmt::skip]
pub fn glValidateProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { (
            _context.proc_addresses.glValidateProgram
        )(converted_program) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib1f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: f32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib1f
        )(converted_index, converted_x) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib1fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (1) as usize;
    let mut vec_v: Vec<f32> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    v + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib1fv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib2f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: f32,
    y: f32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib2f
        )(converted_index, converted_x, converted_y) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib2fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (2) as usize;
    let mut vec_v: Vec<f32> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    v + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib2fv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib3f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: f32,
    y: f32,
    z: f32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib3f
        )(converted_index, converted_x, converted_y, converted_z) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib3fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (3) as usize;
    let mut vec_v: Vec<f32> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    v + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib3fv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib4f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let converted_w = w;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib4f
        )(converted_index, converted_x, converted_y, converted_z, converted_w) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttrib4fv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (4) as usize;
    let mut vec_v: Vec<f32> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(
                    v + i * 4,
                ))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const f32;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttrib4fv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttribDivisor(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    divisor: u32,
) -> () {
    let converted_index = index;
    let converted_divisor = divisor;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttribDivisor
        )(converted_index, converted_divisor) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttribI4i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let converted_w = w;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttribI4i
        )(converted_index, converted_x, converted_y, converted_z, converted_w) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttribI4iv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (4) as usize;
    let mut vec_v: Vec<std::os::raw::c_int> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(
                    v + i * 4,
                ))
                .unwrap() as std::os::raw::c_int,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const std::os::raw::c_int;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttribI4iv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttribI4ui(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: u32,
    y: u32,
    z: u32,
    w: u32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let converted_w = w;
    let result = unsafe { (
            _context.proc_addresses.glVertexAttribI4ui
        )(converted_index, converted_x, converted_y, converted_z, converted_w) };    result
 
}


#[rustfmt::skip]
pub fn glVertexAttribI4uiv(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    v: u32,
) -> () {
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (4) as usize;
    let mut vec_v: Vec<std::os::raw::c_uint> = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32) {
        vec_v.push(
            memory
                .read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(
                    v + i * 4,
                ))
                .unwrap() as std::os::raw::c_uint,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { (
            _context.proc_addresses.glVertexAttribI4uiv
        )(converted_index, converted_v) };    result
 
}


#[rustfmt::skip]
pub fn glViewport(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> () {
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe { (
            _context.proc_addresses.glViewport
        )(converted_x, converted_y, converted_width, converted_height) };    result
 
}


#[rustfmt::skip]
pub fn glWaitSync(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sync: u32,
    flags: u32,
    timeout: u64,
) -> () {
    let converted_sync = _context.resolve_opaque_sync_object(sync);
    let converted_flags = flags;
    let converted_timeout = timeout;
    let result = unsafe { (
            _context.proc_addresses.glWaitSync
        )(converted_sync, converted_flags, converted_timeout) };    result
 
}

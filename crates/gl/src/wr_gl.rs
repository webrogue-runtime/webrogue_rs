#![allow(non_snake_case)]

pub use crate::context::Context;
pub use crate::mainual_impl_wr_gl::*;

pub fn present(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) {
    _context.window.gl_swap_window();
}

pub fn glActiveTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    texture: u32,
) -> () {
    let converted_texture = texture;
    let result = unsafe { crate::ffi::glActiveTexture(converted_texture) };
    result
}

pub fn glAttachShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    shader: u32,
) -> () {
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe { crate::ffi::glAttachShader(converted_program, converted_shader) };
    result
}

pub fn glBindBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    buffer: u32,
) -> () {
    let converted_target = target;
    let converted_buffer = buffer;
    let result = unsafe { crate::ffi::glBindBuffer(converted_target, converted_buffer) };
    result
}

pub fn glBindFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    framebuffer: u32,
) -> () {
    let converted_target = target;
    let converted_framebuffer = framebuffer;
    let result = unsafe { crate::ffi::glBindFramebuffer(converted_target, converted_framebuffer) };
    result
}

pub fn glBindRenderbuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    renderbuffer: u32,
) -> () {
    let converted_target = target;
    let converted_renderbuffer = renderbuffer;
    let result =
        unsafe { crate::ffi::glBindRenderbuffer(converted_target, converted_renderbuffer) };
    result
}

pub fn glBindTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    texture: u32,
) -> () {
    let converted_target = target;
    let converted_texture = texture;
    let result = unsafe { crate::ffi::glBindTexture(converted_target, converted_texture) };
    result
}

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
    let result = unsafe {
        crate::ffi::glBlendColor(
            converted_red,
            converted_green,
            converted_blue,
            converted_alpha,
        )
    };
    result
}

pub fn glBlendEquation(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { crate::ffi::glBlendEquation(converted_mode) };
    result
}

pub fn glBlendEquationSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    modeRGB: u32,
    modeAlpha: u32,
) -> () {
    let converted_modeRGB = modeRGB;
    let converted_modeAlpha = modeAlpha;
    let result =
        unsafe { crate::ffi::glBlendEquationSeparate(converted_modeRGB, converted_modeAlpha) };
    result
}

pub fn glBlendFunc(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    sfactor: u32,
    dfactor: u32,
) -> () {
    let converted_sfactor = sfactor;
    let converted_dfactor = dfactor;
    let result = unsafe { crate::ffi::glBlendFunc(converted_sfactor, converted_dfactor) };
    result
}

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
    let result = unsafe {
        crate::ffi::glBlendFuncSeparate(
            converted_sfactorRGB,
            converted_dfactorRGB,
            converted_sfactorAlpha,
            converted_dfactorAlpha,
        )
    };
    result
}

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
                .read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data + i * 1))
                .unwrap() as u8,
        );
    }
    let converted_data = vec_data.as_mut_ptr() as *mut ();

    let converted_usage = usage;
    let result = unsafe {
        crate::ffi::glBufferData(
            converted_target,
            converted_size,
            converted_data,
            converted_usage,
        )
    };
    result
}

pub fn glCheckFramebufferStatus(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> u32 {
    let converted_target = target;
    let result = unsafe { crate::ffi::glCheckFramebufferStatus(converted_target) };
    result.into()
}

pub fn glClear(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mask: u32,
) -> () {
    let converted_mask = mask;
    let result = unsafe { crate::ffi::glClear(converted_mask) };
    result
}

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
    let result = unsafe {
        crate::ffi::glClearColor(
            converted_red,
            converted_green,
            converted_blue,
            converted_alpha,
        )
    };
    result
}

pub fn glClearDepthf(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    d: f32,
) -> () {
    let converted_d = d;
    let result = unsafe { crate::ffi::glClearDepthf(converted_d) };
    result
}

pub fn glClearStencil(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    s: i32,
) -> () {
    let converted_s = s;
    let result = unsafe { crate::ffi::glClearStencil(converted_s) };
    result
}

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
    let result = unsafe {
        crate::ffi::glColorMask(
            converted_red,
            converted_green,
            converted_blue,
            converted_alpha,
        )
    };
    result
}

pub fn glCompileShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> () {
    let converted_shader = shader;
    let result = unsafe { crate::ffi::glCompileShader(converted_shader) };
    result
}

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
    let result = unsafe {
        crate::ffi::glCopyTexImage2D(
            converted_target,
            converted_level,
            converted_internalformat,
            converted_x,
            converted_y,
            converted_width,
            converted_height,
            converted_border,
        )
    };
    result
}

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
    let result = unsafe {
        crate::ffi::glCopyTexSubImage2D(
            converted_target,
            converted_level,
            converted_xoffset,
            converted_yoffset,
            converted_x,
            converted_y,
            converted_width,
            converted_height,
        )
    };
    result
}

pub fn glCreateProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    let result = unsafe { crate::ffi::glCreateProgram() };
    result.into()
}

pub fn glCreateShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    _type: u32,
) -> u32 {
    let converted__type = _type;
    let result = unsafe { crate::ffi::glCreateShader(converted__type) };
    result.into()
}

pub fn glCullFace(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { crate::ffi::glCullFace(converted_mode) };
    result
}

pub fn glDeleteProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { crate::ffi::glDeleteProgram(converted_program) };
    result
}

pub fn glDeleteShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> () {
    let converted_shader = shader;
    let result = unsafe { crate::ffi::glDeleteShader(converted_shader) };
    result
}

pub fn glDepthFunc(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    func: u32,
) -> () {
    let converted_func = func;
    let result = unsafe { crate::ffi::glDepthFunc(converted_func) };
    result
}

pub fn glDepthMask(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    flag: u32,
) -> () {
    let converted_flag = flag as u8;
    let result = unsafe { crate::ffi::glDepthMask(converted_flag) };
    result
}

pub fn glDepthRangef(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    n: f32,
    f: f32,
) -> () {
    let converted_n = n;
    let converted_f = f;
    let result = unsafe { crate::ffi::glDepthRangef(converted_n, converted_f) };
    result
}

pub fn glDetachShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
    shader: u32,
) -> () {
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe { crate::ffi::glDetachShader(converted_program, converted_shader) };
    result
}

pub fn glDisable(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> () {
    let converted_cap = cap;
    let result = unsafe { crate::ffi::glDisable(converted_cap) };
    result
}

pub fn glDisableVertexAttribArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
) -> () {
    let converted_index = index;
    let result = unsafe { crate::ffi::glDisableVertexAttribArray(converted_index) };
    result
}

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
    let result =
        unsafe { crate::ffi::glDrawArrays(converted_mode, converted_first, converted_count) };
    result
}

pub fn glEnable(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> () {
    let converted_cap = cap;
    let result = unsafe { crate::ffi::glEnable(converted_cap) };
    result
}

pub fn glEnableVertexAttribArray(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
) -> () {
    let converted_index = index;
    let result = unsafe { crate::ffi::glEnableVertexAttribArray(converted_index) };
    result
}

pub fn glFinish(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {
    let result = unsafe { crate::ffi::glFinish() };
    result
}

pub fn glFlush(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {
    let result = unsafe { crate::ffi::glFlush() };
    result
}

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
    let result = unsafe {
        crate::ffi::glFramebufferRenderbuffer(
            converted_target,
            converted_attachment,
            converted_renderbuffertarget,
            converted_renderbuffer,
        )
    };
    result
}

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
    let result = unsafe {
        crate::ffi::glFramebufferTexture2D(
            converted_target,
            converted_attachment,
            converted_textarget,
            converted_texture,
            converted_level,
        )
    };
    result
}

pub fn glFrontFace(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mode: u32,
) -> () {
    let converted_mode = mode;
    let result = unsafe { crate::ffi::glFrontFace(converted_mode) };
    result
}

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
    let converted_buffers = vec_buffers.as_mut_ptr() as *const std::os::raw::c_uint;

    let result = unsafe { crate::ffi::glGenBuffers(converted_n, converted_buffers) };
    for (i, value) in vec_buffers.iter().enumerate() {
        memory
            .write::<u32>(
                webrogue_runtime::wiggle::GuestPtr::<u32>::new(buffers + (i as u32) * 4),
                *value as u32,
            )
            .unwrap()
    }
    result
}

pub fn glGenerateMipmap(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
) -> () {
    let converted_target = target;
    let result = unsafe { crate::ffi::glGenerateMipmap(converted_target) };
    result
}

pub fn glGetError(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> u32 {
    let result = unsafe { crate::ffi::glGetError() };
    result.into()
}

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
    let converted_length = vec_length.as_mut_ptr() as *const std::os::raw::c_int;

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
    let converted_infoLog = vec_infoLog.as_mut_ptr() as *const i8;

    let result = unsafe {
        crate::ffi::glGetProgramInfoLog(
            converted_program,
            converted_bufSize,
            converted_length,
            converted_infoLog,
        )
    };
    for (i, value) in vec_length.iter().enumerate() {
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
    }
    result
}

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
    let len_params = (crate::compsize::glGetProgramiv_params_compsize(pname)) as usize;
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

    let result =
        unsafe { crate::ffi::glGetProgramiv(converted_program, converted_pname, converted_params) };
    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    result
}

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
    let converted_length = vec_length.as_mut_ptr() as *const std::os::raw::c_int;

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
    let converted_infoLog = vec_infoLog.as_mut_ptr() as *const i8;

    let result = unsafe {
        crate::ffi::glGetShaderInfoLog(
            converted_shader,
            converted_bufSize,
            converted_length,
            converted_infoLog,
        )
    };
    for (i, value) in vec_length.iter().enumerate() {
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
    }
    result
}

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
    let len_params = (crate::compsize::glGetShaderiv_params_compsize(pname)) as usize;
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

    let result =
        unsafe { crate::ffi::glGetShaderiv(converted_shader, converted_pname, converted_params) };
    for (i, value) in vec_params.iter().enumerate() {
        memory
            .write::<i32>(
                webrogue_runtime::wiggle::GuestPtr::<i32>::new(params + (i as u32) * 4),
                *value as i32,
            )
            .unwrap()
    }
    result
}

pub fn glHint(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    target: u32,
    mode: u32,
) -> () {
    let converted_target = target;
    let converted_mode = mode;
    let result = unsafe { crate::ffi::glHint(converted_target, converted_mode) };
    result
}

pub fn glIsBuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    buffer: u32,
) -> u32 {
    let converted_buffer = buffer;
    let result = unsafe { crate::ffi::glIsBuffer(converted_buffer) };
    result.into()
}

pub fn glIsEnabled(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    cap: u32,
) -> u32 {
    let converted_cap = cap;
    let result = unsafe { crate::ffi::glIsEnabled(converted_cap) };
    result.into()
}

pub fn glIsFramebuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    framebuffer: u32,
) -> u32 {
    let converted_framebuffer = framebuffer;
    let result = unsafe { crate::ffi::glIsFramebuffer(converted_framebuffer) };
    result.into()
}

pub fn glIsProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> u32 {
    let converted_program = program;
    let result = unsafe { crate::ffi::glIsProgram(converted_program) };
    result.into()
}

pub fn glIsRenderbuffer(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    renderbuffer: u32,
) -> u32 {
    let converted_renderbuffer = renderbuffer;
    let result = unsafe { crate::ffi::glIsRenderbuffer(converted_renderbuffer) };
    result.into()
}

pub fn glIsShader(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    shader: u32,
) -> u32 {
    let converted_shader = shader;
    let result = unsafe { crate::ffi::glIsShader(converted_shader) };
    result.into()
}

pub fn glIsTexture(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    texture: u32,
) -> u32 {
    let converted_texture = texture;
    let result = unsafe { crate::ffi::glIsTexture(converted_texture) };
    result.into()
}

pub fn glLineWidth(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    width: f32,
) -> () {
    let converted_width = width;
    let result = unsafe { crate::ffi::glLineWidth(converted_width) };
    result
}

pub fn glLinkProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { crate::ffi::glLinkProgram(converted_program) };
    result
}

pub fn glPixelStorei(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    pname: u32,
    param: i32,
) -> () {
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe { crate::ffi::glPixelStorei(converted_pname, converted_param) };
    result
}

pub fn glPolygonOffset(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    factor: f32,
    units: f32,
) -> () {
    let converted_factor = factor;
    let converted_units = units;
    let result = unsafe { crate::ffi::glPolygonOffset(converted_factor, converted_units) };
    result
}

pub fn glReleaseShaderCompiler(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
) -> () {
    let result = unsafe { crate::ffi::glReleaseShaderCompiler() };
    result
}

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
    let result = unsafe {
        crate::ffi::glRenderbufferStorage(
            converted_target,
            converted_internalformat,
            converted_width,
            converted_height,
        )
    };
    result
}

pub fn glSampleCoverage(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    value: f32,
    invert: u32,
) -> () {
    let converted_value = value;
    let converted_invert = invert as u8;
    let result = unsafe { crate::ffi::glSampleCoverage(converted_value, converted_invert) };
    result
}

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
    let result = unsafe {
        crate::ffi::glScissor(converted_x, converted_y, converted_width, converted_height)
    };
    result
}

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
    let result =
        unsafe { crate::ffi::glStencilFunc(converted_func, converted__ref, converted_mask) };
    result
}

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
    let result = unsafe {
        crate::ffi::glStencilFuncSeparate(
            converted_face,
            converted_func,
            converted__ref,
            converted_mask,
        )
    };
    result
}

pub fn glStencilMask(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    mask: u32,
) -> () {
    let converted_mask = mask;
    let result = unsafe { crate::ffi::glStencilMask(converted_mask) };
    result
}

pub fn glStencilMaskSeparate(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    face: u32,
    mask: u32,
) -> () {
    let converted_face = face;
    let converted_mask = mask;
    let result = unsafe { crate::ffi::glStencilMaskSeparate(converted_face, converted_mask) };
    result
}

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
    let result =
        unsafe { crate::ffi::glStencilOp(converted_fail, converted_zfail, converted_zpass) };
    result
}

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
    let result = unsafe {
        crate::ffi::glStencilOpSeparate(
            converted_face,
            converted_sfail,
            converted_dpfail,
            converted_dppass,
        )
    };
    result
}

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
    let result =
        unsafe { crate::ffi::glTexParameterf(converted_target, converted_pname, converted_param) };
    result
}

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
    let result =
        unsafe { crate::ffi::glTexParameteri(converted_target, converted_pname, converted_param) };
    result
}

pub fn glUniform1f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: f32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe { crate::ffi::glUniform1f(converted_location, converted_v0) };
    result
}

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
    let len_value = (count * 1) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result =
        unsafe { crate::ffi::glUniform1fv(converted_location, converted_count, converted_value) };
    result
}

pub fn glUniform1i(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    location: i32,
    v0: i32,
) -> () {
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe { crate::ffi::glUniform1i(converted_location, converted_v0) };
    result
}

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
    let len_value = (count * 1) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut std::os::raw::c_int;

    let result =
        unsafe { crate::ffi::glUniform1iv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe { crate::ffi::glUniform2f(converted_location, converted_v0, converted_v1) };
    result
}

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
    let len_value = (count * 2) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result =
        unsafe { crate::ffi::glUniform2fv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe { crate::ffi::glUniform2i(converted_location, converted_v0, converted_v1) };
    result
}

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
    let len_value = (count * 2) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut std::os::raw::c_int;

    let result =
        unsafe { crate::ffi::glUniform2iv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe {
        crate::ffi::glUniform3f(converted_location, converted_v0, converted_v1, converted_v2)
    };
    result
}

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
    let len_value = (count * 3) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result =
        unsafe { crate::ffi::glUniform3fv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe {
        crate::ffi::glUniform3i(converted_location, converted_v0, converted_v1, converted_v2)
    };
    result
}

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
    let len_value = (count * 3) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut std::os::raw::c_int;

    let result =
        unsafe { crate::ffi::glUniform3iv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe {
        crate::ffi::glUniform4f(
            converted_location,
            converted_v0,
            converted_v1,
            converted_v2,
            converted_v3,
        )
    };
    result
}

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
    let len_value = (count * 4) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result =
        unsafe { crate::ffi::glUniform4fv(converted_location, converted_count, converted_value) };
    result
}

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
    let result = unsafe {
        crate::ffi::glUniform4i(
            converted_location,
            converted_v0,
            converted_v1,
            converted_v2,
            converted_v3,
        )
    };
    result
}

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
    let len_value = (count * 4) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut std::os::raw::c_int;

    let result =
        unsafe { crate::ffi::glUniform4iv(converted_location, converted_count, converted_value) };
    result
}

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
    let len_value = (count * 4) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result = unsafe {
        crate::ffi::glUniformMatrix2fv(
            converted_location,
            converted_count,
            converted_transpose,
            converted_value,
        )
    };
    result
}

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
    let len_value = (count * 9) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result = unsafe {
        crate::ffi::glUniformMatrix3fv(
            converted_location,
            converted_count,
            converted_transpose,
            converted_value,
        )
    };
    result
}

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
    let len_value = (count * 16) as usize;
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
    let converted_value = vec_value.as_mut_ptr() as *mut f32;

    let result = unsafe {
        crate::ffi::glUniformMatrix4fv(
            converted_location,
            converted_count,
            converted_transpose,
            converted_value,
        )
    };
    result
}

pub fn glUseProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { crate::ffi::glUseProgram(converted_program) };
    result
}

pub fn glValidateProgram(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    program: u32,
) -> () {
    let converted_program = program;
    let result = unsafe { crate::ffi::glValidateProgram(converted_program) };
    result
}

pub fn glVertexAttrib1f(
    _memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,
    _context: &mut Context,
    index: u32,
    x: f32,
) -> () {
    let converted_index = index;
    let converted_x = x;
    let result = unsafe { crate::ffi::glVertexAttrib1f(converted_index, converted_x) };
    result
}

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
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v + i * 4))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *mut f32;

    let result = unsafe { crate::ffi::glVertexAttrib1fv(converted_index, converted_v) };
    result
}

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
    let result = unsafe { crate::ffi::glVertexAttrib2f(converted_index, converted_x, converted_y) };
    result
}

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
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v + i * 4))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *mut f32;

    let result = unsafe { crate::ffi::glVertexAttrib2fv(converted_index, converted_v) };
    result
}

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
    let result = unsafe {
        crate::ffi::glVertexAttrib3f(converted_index, converted_x, converted_y, converted_z)
    };
    result
}

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
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v + i * 4))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *mut f32;

    let result = unsafe { crate::ffi::glVertexAttrib3fv(converted_index, converted_v) };
    result
}

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
    let result = unsafe {
        crate::ffi::glVertexAttrib4f(
            converted_index,
            converted_x,
            converted_y,
            converted_z,
            converted_w,
        )
    };
    result
}

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
                .read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v + i * 4))
                .unwrap() as f32,
        );
    }
    let converted_v = vec_v.as_mut_ptr() as *mut f32;

    let result = unsafe { crate::ffi::glVertexAttrib4fv(converted_index, converted_v) };
    result
}

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
    let result = unsafe {
        crate::ffi::glViewport(converted_x, converted_y, converted_width, converted_height)
    };
    result
}

#![allow(non_snake_case)]

pub use crate::context::Context;

pub fn glActiveTexture(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,texture:u32,) -> (){
    let converted_texture = texture;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(texture:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glActiveTexture"))(converted_texture)
    };
    result
}
pub fn glAttachShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,shader:u32,) -> (){
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,shader:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glAttachShader"))(converted_program,converted_shader)
    };
    result
}
pub fn glBindBuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,buffer:u32,) -> (){
    let converted_target = target;
    let converted_buffer = buffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,buffer:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBindBuffer"))(converted_target,converted_buffer)
    };
    result
}
pub fn glBindFramebuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,framebuffer:u32,) -> (){
    let converted_target = target;
    let converted_framebuffer = framebuffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,framebuffer:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBindFramebuffer"))(converted_target,converted_framebuffer)
    };
    result
}
pub fn glBindRenderbuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,renderbuffer:u32,) -> (){
    let converted_target = target;
    let converted_renderbuffer = renderbuffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,renderbuffer:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBindRenderbuffer"))(converted_target,converted_renderbuffer)
    };
    result
}
pub fn glBindTexture(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,texture:u32,) -> (){
    let converted_target = target;
    let converted_texture = texture;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,texture:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBindTexture"))(converted_target,converted_texture)
    };
    result
}
pub fn glBlendColor(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,red:f32,green:f32,blue:f32,alpha:f32,) -> (){
    let converted_red = red;
    let converted_green = green;
    let converted_blue = blue;
    let converted_alpha = alpha;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(red:f32,green:f32,blue:f32,alpha:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBlendColor"))(converted_red,converted_green,converted_blue,converted_alpha)
    };
    result
}
pub fn glBlendEquation(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mode:u32,) -> (){
    let converted_mode = mode;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mode:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBlendEquation"))(converted_mode)
    };
    result
}
pub fn glBlendEquationSeparate(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,modeRGB:u32,modeAlpha:u32,) -> (){
    let converted_modeRGB = modeRGB;
    let converted_modeAlpha = modeAlpha;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(modeRGB:std::os::raw::c_uint,modeAlpha:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBlendEquationSeparate"))(converted_modeRGB,converted_modeAlpha)
    };
    result
}
pub fn glBlendFunc(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,sfactor:u32,dfactor:u32,) -> (){
    let converted_sfactor = sfactor;
    let converted_dfactor = dfactor;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(sfactor:std::os::raw::c_uint,dfactor:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBlendFunc"))(converted_sfactor,converted_dfactor)
    };
    result
}
pub fn glBlendFuncSeparate(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,sfactorRGB:u32,dfactorRGB:u32,sfactorAlpha:u32,dfactorAlpha:u32,) -> (){
    let converted_sfactorRGB = sfactorRGB;
    let converted_dfactorRGB = dfactorRGB;
    let converted_sfactorAlpha = sfactorAlpha;
    let converted_dfactorAlpha = dfactorAlpha;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(sfactorRGB:std::os::raw::c_uint,dfactorRGB:std::os::raw::c_uint,sfactorAlpha:std::os::raw::c_uint,dfactorAlpha:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBlendFuncSeparate"))(converted_sfactorRGB,converted_dfactorRGB,converted_sfactorAlpha,converted_dfactorAlpha)
    };
    result
}
pub fn glBufferData(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,size:i32,data:u32,usage:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_size = size as isize;
    let len_data = (size)as usize;
    let mut vec_data:Vec<u8>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+i*1,)).unwrap()as u8,);
    }let converted_data = vec_data.as_mut_ptr()as *mut ();
    let converted_usage = usage;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,size:isize,data: *mut (),usage:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBufferData"))(converted_target,converted_size,converted_data,converted_usage)
    };
    result
}
pub fn glBufferSubData(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,offset:i32,size:i32,data:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_offset = offset as isize;
    let converted_size = size as isize;
    let len_data = (size)as usize;
    let mut vec_data:Vec<u8>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+i*1,)).unwrap()as u8,);
    }let converted_data = vec_data.as_mut_ptr()as *mut ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,offset:isize,size:isize,data: *mut (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glBufferSubData"))(converted_target,converted_offset,converted_size,converted_data)
    };
    result
}
pub fn glCheckFramebufferStatus(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,) -> u32 {
    let converted_target = target;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCheckFramebufferStatus"))(converted_target)
    };
    result.into()
}
pub fn glClear(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mask:u32,) -> (){
    let converted_mask = mask;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mask:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glClear"))(converted_mask)
    };
    result
}
pub fn glClearColor(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,red:f32,green:f32,blue:f32,alpha:f32,) -> (){
    let converted_red = red;
    let converted_green = green;
    let converted_blue = blue;
    let converted_alpha = alpha;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(red:f32,green:f32,blue:f32,alpha:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glClearColor"))(converted_red,converted_green,converted_blue,converted_alpha)
    };
    result
}
pub fn glClearDepthf(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,d:f32,) -> (){
    let converted_d = d;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(d:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glClearDepthf"))(converted_d)
    };
    result
}
pub fn glClearStencil(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,s:i32,) -> (){
    let converted_s = s;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(s:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glClearStencil"))(converted_s)
    };
    result
}
pub fn glColorMask(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,red:u32,green:u32,blue:u32,alpha:u32,) -> (){
    let converted_red = red as u8;
    let converted_green = green as u8;
    let converted_blue = blue as u8;
    let converted_alpha = alpha as u8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(red:u8,green:u8,blue:u8,alpha:u8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glColorMask"))(converted_red,converted_green,converted_blue,converted_alpha)
    };
    result
}
pub fn glCompileShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,) -> (){
    let converted_shader = shader;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCompileShader"))(converted_shader)
    };
    result
}
pub fn glCompressedTexImage2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,level:i32,internalformat:u32,width:i32,height:i32,border:i32,imageSize:i32,data:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let converted_border = border;
    let converted_imageSize = imageSize;
    let len_data = (imageSize)as usize;
    let mut vec_data:Vec<u8>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+i*1,)).unwrap()as u8,);
    }let converted_data = vec_data.as_mut_ptr()as *mut ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,level:std::os::raw::c_int,internalformat:std::os::raw::c_uint,width:std::os::raw::c_int,height:std::os::raw::c_int,border:std::os::raw::c_int,imageSize:std::os::raw::c_int,data: *mut (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCompressedTexImage2D"))(converted_target,converted_level,converted_internalformat,converted_width,converted_height,converted_border,converted_imageSize,converted_data)
    };
    result
}
pub fn glCompressedTexSubImage2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,level:i32,xoffset:i32,yoffset:i32,width:i32,height:i32,format:u32,imageSize:i32,data:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted_imageSize = imageSize;
    let len_data = (imageSize)as usize;
    let mut vec_data:Vec<u8>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+i*1,)).unwrap()as u8,);
    }let converted_data = vec_data.as_mut_ptr()as *mut ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,level:std::os::raw::c_int,xoffset:std::os::raw::c_int,yoffset:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,format:std::os::raw::c_uint,imageSize:std::os::raw::c_int,data: *mut (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCompressedTexSubImage2D"))(converted_target,converted_level,converted_xoffset,converted_yoffset,converted_width,converted_height,converted_format,converted_imageSize,converted_data)
    };
    result
}
pub fn glCopyTexImage2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,level:i32,internalformat:u32,x:i32,y:i32,width:i32,height:i32,border:i32,) -> (){
    let converted_target = target;
    let converted_level = level;
    let converted_internalformat = internalformat;
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let converted_border = border;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,level:std::os::raw::c_int,internalformat:std::os::raw::c_uint,x:std::os::raw::c_int,y:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,border:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCopyTexImage2D"))(converted_target,converted_level,converted_internalformat,converted_x,converted_y,converted_width,converted_height,converted_border)
    };
    result
}
pub fn glCopyTexSubImage2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,level:i32,xoffset:i32,yoffset:i32,x:i32,y:i32,width:i32,height:i32,) -> (){
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,level:std::os::raw::c_int,xoffset:std::os::raw::c_int,yoffset:std::os::raw::c_int,x:std::os::raw::c_int,y:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCopyTexSubImage2D"))(converted_target,converted_level,converted_xoffset,converted_yoffset,converted_x,converted_y,converted_width,converted_height)
    };
    result
}
pub fn glCreateProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,) -> u32 {
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn() -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCreateProgram"))()
    };
    result.into()
}
pub fn glCreateShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,_type:u32,) -> u32 {
    let converted__type = _type;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(_type:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCreateShader"))(converted__type)
    };
    result.into()
}
pub fn glCullFace(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mode:u32,) -> (){
    let converted_mode = mode;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mode:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glCullFace"))(converted_mode)
    };
    result
}
pub fn glDeleteBuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,buffers:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_buffers = (n)as usize;
    let mut vec_buffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_buffers.reserve(len_buffers);
    for i in 0..(len_buffers as u32){
        vec_buffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(buffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_buffers = vec_buffers.as_mut_ptr()as *mut std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,buffers: *mut std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteBuffers"))(converted_n,converted_buffers)
    };
    result
}
pub fn glDeleteFramebuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,framebuffers:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_framebuffers = (n)as usize;
    let mut vec_framebuffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_framebuffers.reserve(len_framebuffers);
    for i in 0..(len_framebuffers as u32){
        vec_framebuffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(framebuffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_framebuffers = vec_framebuffers.as_mut_ptr()as *mut std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,framebuffers: *mut std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteFramebuffers"))(converted_n,converted_framebuffers)
    };
    result
}
pub fn glDeleteProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,) -> (){
    let converted_program = program;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteProgram"))(converted_program)
    };
    result
}
pub fn glDeleteRenderbuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,renderbuffers:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_renderbuffers = (n)as usize;
    let mut vec_renderbuffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_renderbuffers.reserve(len_renderbuffers);
    for i in 0..(len_renderbuffers as u32){
        vec_renderbuffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(renderbuffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_renderbuffers = vec_renderbuffers.as_mut_ptr()as *mut std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,renderbuffers: *mut std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteRenderbuffers"))(converted_n,converted_renderbuffers)
    };
    result
}
pub fn glDeleteShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,) -> (){
    let converted_shader = shader;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteShader"))(converted_shader)
    };
    result
}
pub fn glDeleteTextures(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,textures:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_textures = (n)as usize;
    let mut vec_textures:Vec<std::os::raw::c_uint>  = vec![];
    vec_textures.reserve(len_textures);
    for i in 0..(len_textures as u32){
        vec_textures.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(textures+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_textures = vec_textures.as_mut_ptr()as *mut std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,textures: *mut std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDeleteTextures"))(converted_n,converted_textures)
    };
    result
}
pub fn glDepthFunc(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,func:u32,) -> (){
    let converted_func = func;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(func:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDepthFunc"))(converted_func)
    };
    result
}
pub fn glDepthMask(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,flag:u32,) -> (){
    let converted_flag = flag as u8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(flag:u8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDepthMask"))(converted_flag)
    };
    result
}
pub fn glDepthRangef(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:f32,f:f32,) -> (){
    let converted_n = n;
    let converted_f = f;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:f32,f:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDepthRangef"))(converted_n,converted_f)
    };
    result
}
pub fn glDetachShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,shader:u32,) -> (){
    let converted_program = program;
    let converted_shader = shader;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,shader:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDetachShader"))(converted_program,converted_shader)
    };
    result
}
pub fn glDisable(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,cap:u32,) -> (){
    let converted_cap = cap;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(cap:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDisable"))(converted_cap)
    };
    result
}
pub fn glDisableVertexAttribArray(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,) -> (){
    let converted_index = index;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDisableVertexAttribArray"))(converted_index)
    };
    result
}
pub fn glDrawArrays(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mode:u32,first:i32,count:i32,) -> (){
    let converted_mode = mode;
    let converted_first = first;
    let converted_count = count;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mode:std::os::raw::c_uint,first:std::os::raw::c_int,count:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDrawArrays"))(converted_mode,converted_first,converted_count)
    };
    result
}
pub fn glDrawElements(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mode:u32,count:i32,_type:u32,indices:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_mode = mode;
    let converted_count = count;
    let converted__type = _type;
    let len_indices = (crate::compsize::glDrawElements_indices_compsize(count,_type))as usize;
    let mut vec_indices:Vec<u8>  = vec![];
    vec_indices.reserve(len_indices);
    for i in 0..(len_indices as u32){
        vec_indices.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(indices+i*1,)).unwrap()as u8,);
    }let converted_indices = vec_indices.as_mut_ptr()as *mut ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mode:std::os::raw::c_uint,count:std::os::raw::c_int,_type:std::os::raw::c_uint,indices: *mut (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glDrawElements"))(converted_mode,converted_count,converted__type,converted_indices)
    };
    result
}
pub fn glEnable(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,cap:u32,) -> (){
    let converted_cap = cap;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(cap:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glEnable"))(converted_cap)
    };
    result
}
pub fn glEnableVertexAttribArray(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,) -> (){
    let converted_index = index;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glEnableVertexAttribArray"))(converted_index)
    };
    result
}
pub fn glFinish(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,) -> (){
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn() -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glFinish"))()
    };
    result
}
pub fn glFlush(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,) -> (){
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn() -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glFlush"))()
    };
    result
}
pub fn glFramebufferRenderbuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,attachment:u32,renderbuffertarget:u32,renderbuffer:u32,) -> (){
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_renderbuffertarget = renderbuffertarget;
    let converted_renderbuffer = renderbuffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,attachment:std::os::raw::c_uint,renderbuffertarget:std::os::raw::c_uint,renderbuffer:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glFramebufferRenderbuffer"))(converted_target,converted_attachment,converted_renderbuffertarget,converted_renderbuffer)
    };
    result
}
pub fn glFramebufferTexture2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,attachment:u32,textarget:u32,texture:u32,level:i32,) -> (){
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_textarget = textarget;
    let converted_texture = texture;
    let converted_level = level;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,attachment:std::os::raw::c_uint,textarget:std::os::raw::c_uint,texture:std::os::raw::c_uint,level:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glFramebufferTexture2D"))(converted_target,converted_attachment,converted_textarget,converted_texture,converted_level)
    };
    result
}
pub fn glFrontFace(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mode:u32,) -> (){
    let converted_mode = mode;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mode:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glFrontFace"))(converted_mode)
    };
    result
}
pub fn glGenBuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,buffers:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_buffers = (n)as usize;
    let mut vec_buffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_buffers.reserve(len_buffers);
    for i in 0..(len_buffers as u32){
        vec_buffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(buffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_buffers = vec_buffers.as_mut_ptr()as *const std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,buffers: *const std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGenBuffers"))(converted_n,converted_buffers)
    };
    for(i,value)in vec_buffers.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(buffers+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGenFramebuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,framebuffers:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_framebuffers = (n)as usize;
    let mut vec_framebuffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_framebuffers.reserve(len_framebuffers);
    for i in 0..(len_framebuffers as u32){
        vec_framebuffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(framebuffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_framebuffers = vec_framebuffers.as_mut_ptr()as *const std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,framebuffers: *const std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGenFramebuffers"))(converted_n,converted_framebuffers)
    };
    for(i,value)in vec_framebuffers.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(framebuffers+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGenRenderbuffers(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,renderbuffers:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_renderbuffers = (n)as usize;
    let mut vec_renderbuffers:Vec<std::os::raw::c_uint>  = vec![];
    vec_renderbuffers.reserve(len_renderbuffers);
    for i in 0..(len_renderbuffers as u32){
        vec_renderbuffers.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(renderbuffers+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_renderbuffers = vec_renderbuffers.as_mut_ptr()as *const std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,renderbuffers: *const std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGenRenderbuffers"))(converted_n,converted_renderbuffers)
    };
    for(i,value)in vec_renderbuffers.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(renderbuffers+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGenTextures(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,n:i32,textures:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_n = n;
    let len_textures = (n)as usize;
    let mut vec_textures:Vec<std::os::raw::c_uint>  = vec![];
    vec_textures.reserve(len_textures);
    for i in 0..(len_textures as u32){
        vec_textures.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(textures+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_textures = vec_textures.as_mut_ptr()as *const std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(n:std::os::raw::c_int,textures: *const std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGenTextures"))(converted_n,converted_textures)
    };
    for(i,value)in vec_textures.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(textures+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGenerateMipmap(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,) -> (){
    let converted_target = target;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGenerateMipmap"))(converted_target)
    };
    result
}
pub fn glGetActiveAttrib(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,index:u32,bufSize:i32,length:u32,size:u32,_type:u32,name:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let converted_bufSize = bufSize;
    let len_length = (1)as usize;
    let mut vec_length:Vec<std::os::raw::c_int>  = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32){
        vec_length.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_length = vec_length.as_mut_ptr()as *const std::os::raw::c_int;
    let len_size = (1)as usize;
    let mut vec_size:Vec<std::os::raw::c_int>  = vec![];
    vec_size.reserve(len_size);
    for i in 0..(len_size as u32){
        vec_size.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(size+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_size = vec_size.as_mut_ptr()as *const std::os::raw::c_int;
    let len__type = (1)as usize;
    let mut vec__type:Vec<std::os::raw::c_uint>  = vec![];
    vec__type.reserve(len__type);
    for i in 0..(len__type as u32){
        vec__type.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted__type = vec__type.as_mut_ptr()as *const std::os::raw::c_uint;
    let len_name = (bufSize)as usize;
    let mut vec_name:Vec<i8>  = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32){
        vec_name.push(memory.read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(name+i*1,)).unwrap()as i8,);
    }let converted_name = vec_name.as_mut_ptr()as *const i8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,index:std::os::raw::c_uint,bufSize:std::os::raw::c_int,length: *const std::os::raw::c_int,size: *const std::os::raw::c_int,_type: *const std::os::raw::c_uint,name: *const i8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetActiveAttrib"))(converted_program,converted_index,converted_bufSize,converted_length,converted_size,converted__type,converted_name)
    };
    for(i,value)in vec_length.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_size.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(size+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec__type.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type+(i as u32)*4), *value as u32,).unwrap()
    }for(i,value)in vec_name.iter().enumerate(){
        memory.write::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(name+(i as u32)*1), *value as i8,).unwrap()
    }result
}
pub fn glGetActiveUniform(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,index:u32,bufSize:i32,length:u32,size:u32,_type:u32,name:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_index = index;
    let converted_bufSize = bufSize;
    let len_length = (1)as usize;
    let mut vec_length:Vec<std::os::raw::c_int>  = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32){
        vec_length.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_length = vec_length.as_mut_ptr()as *const std::os::raw::c_int;
    let len_size = (1)as usize;
    let mut vec_size:Vec<std::os::raw::c_int>  = vec![];
    vec_size.reserve(len_size);
    for i in 0..(len_size as u32){
        vec_size.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(size+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_size = vec_size.as_mut_ptr()as *const std::os::raw::c_int;
    let len__type = (1)as usize;
    let mut vec__type:Vec<std::os::raw::c_uint>  = vec![];
    vec__type.reserve(len__type);
    for i in 0..(len__type as u32){
        vec__type.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted__type = vec__type.as_mut_ptr()as *const std::os::raw::c_uint;
    let len_name = (bufSize)as usize;
    let mut vec_name:Vec<i8>  = vec![];
    vec_name.reserve(len_name);
    for i in 0..(len_name as u32){
        vec_name.push(memory.read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(name+i*1,)).unwrap()as i8,);
    }let converted_name = vec_name.as_mut_ptr()as *const i8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,index:std::os::raw::c_uint,bufSize:std::os::raw::c_int,length: *const std::os::raw::c_int,size: *const std::os::raw::c_int,_type: *const std::os::raw::c_uint,name: *const i8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetActiveUniform"))(converted_program,converted_index,converted_bufSize,converted_length,converted_size,converted__type,converted_name)
    };
    for(i,value)in vec_length.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_size.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(size+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec__type.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(_type+(i as u32)*4), *value as u32,).unwrap()
    }for(i,value)in vec_name.iter().enumerate(){
        memory.write::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(name+(i as u32)*1), *value as i8,).unwrap()
    }result
}
pub fn glGetAttachedShaders(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,maxCount:i32,count:u32,shaders:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_maxCount = maxCount;
    let len_count = (1)as usize;
    let mut vec_count:Vec<std::os::raw::c_int>  = vec![];
    vec_count.reserve(len_count);
    for i in 0..(len_count as u32){
        vec_count.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(count+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_count = vec_count.as_mut_ptr()as *const std::os::raw::c_int;
    let len_shaders = (maxCount)as usize;
    let mut vec_shaders:Vec<std::os::raw::c_uint>  = vec![];
    vec_shaders.reserve(len_shaders);
    for i in 0..(len_shaders as u32){
        vec_shaders.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(shaders+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_shaders = vec_shaders.as_mut_ptr()as *const std::os::raw::c_uint;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,maxCount:std::os::raw::c_int,count: *const std::os::raw::c_int,shaders: *const std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetAttachedShaders"))(converted_program,converted_maxCount,converted_count,converted_shaders)
    };
    for(i,value)in vec_count.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(count+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_shaders.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(shaders+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGetBooleanv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,pname:u32,data:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetBooleanv_data_compsize(pname))as usize;
    let mut vec_data:Vec<u8>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+i*1,)).unwrap()as u8,);
    }let converted_data = vec_data.as_mut_ptr()as *const u8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(pname:std::os::raw::c_uint,data: *const u8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetBooleanv"))(converted_pname,converted_data)
    };
    for(i,value)in vec_data.iter().enumerate(){
        memory.write::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(data+(i as u32)*1), *value as u8,).unwrap()
    }result
}
pub fn glGetBufferParameteriv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetBufferParameteriv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetBufferParameteriv"))(converted_target,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetError(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,) -> u32 {
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn() -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetError"))()
    };
    result.into()
}
pub fn glGetFloatv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,pname:u32,data:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetFloatv_data_compsize(pname))as usize;
    let mut vec_data:Vec<f32>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(data+i*4,)).unwrap()as f32,);
    }let converted_data = vec_data.as_mut_ptr()as *const f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(pname:std::os::raw::c_uint,data: *const f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetFloatv"))(converted_pname,converted_data)
    };
    for(i,value)in vec_data.iter().enumerate(){
        memory.write::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(data+(i as u32)*4), *value as f32,).unwrap()
    }result
}
pub fn glGetFramebufferAttachmentParameteriv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,attachment:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_attachment = attachment;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetFramebufferAttachmentParameteriv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,attachment:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetFramebufferAttachmentParameteriv"))(converted_target,converted_attachment,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetIntegerv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,pname:u32,data:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_pname = pname;
    let len_data = (crate::compsize::glGetIntegerv_data_compsize(pname))as usize;
    let mut vec_data:Vec<std::os::raw::c_int>  = vec![];
    vec_data.reserve(len_data);
    for i in 0..(len_data as u32){
        vec_data.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(data+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_data = vec_data.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(pname:std::os::raw::c_uint,data: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetIntegerv"))(converted_pname,converted_data)
    };
    for(i,value)in vec_data.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(data+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetProgramInfoLog(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,bufSize:i32,length:u32,infoLog:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_bufSize = bufSize;
    let len_length = (1)as usize;
    let mut vec_length:Vec<std::os::raw::c_int>  = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32){
        vec_length.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_length = vec_length.as_mut_ptr()as *const std::os::raw::c_int;
    let len_infoLog = (bufSize)as usize;
    let mut vec_infoLog:Vec<i8>  = vec![];
    vec_infoLog.reserve(len_infoLog);
    for i in 0..(len_infoLog as u32){
        vec_infoLog.push(memory.read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog+i*1,)).unwrap()as i8,);
    }let converted_infoLog = vec_infoLog.as_mut_ptr()as *const i8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,bufSize:std::os::raw::c_int,length: *const std::os::raw::c_int,infoLog: *const i8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetProgramInfoLog"))(converted_program,converted_bufSize,converted_length,converted_infoLog)
    };
    for(i,value)in vec_length.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_infoLog.iter().enumerate(){
        memory.write::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog+(i as u32)*1), *value as i8,).unwrap()
    }result
}
pub fn glGetProgramiv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetProgramiv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetProgramiv"))(converted_program,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetRenderbufferParameteriv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetRenderbufferParameteriv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetRenderbufferParameteriv"))(converted_target,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetShaderInfoLog(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,bufSize:i32,length:u32,infoLog:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_bufSize = bufSize;
    let len_length = (1)as usize;
    let mut vec_length:Vec<std::os::raw::c_int>  = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32){
        vec_length.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_length = vec_length.as_mut_ptr()as *const std::os::raw::c_int;
    let len_infoLog = (bufSize)as usize;
    let mut vec_infoLog:Vec<i8>  = vec![];
    vec_infoLog.reserve(len_infoLog);
    for i in 0..(len_infoLog as u32){
        vec_infoLog.push(memory.read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog+i*1,)).unwrap()as i8,);
    }let converted_infoLog = vec_infoLog.as_mut_ptr()as *const i8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,bufSize:std::os::raw::c_int,length: *const std::os::raw::c_int,infoLog: *const i8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetShaderInfoLog"))(converted_shader,converted_bufSize,converted_length,converted_infoLog)
    };
    for(i,value)in vec_length.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_infoLog.iter().enumerate(){
        memory.write::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(infoLog+(i as u32)*1), *value as i8,).unwrap()
    }result
}
pub fn glGetShaderPrecisionFormat(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shadertype:u32,precisiontype:u32,range:u32,precision:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_shadertype = shadertype;
    let converted_precisiontype = precisiontype;
    let len_range = (2)as usize;
    let mut vec_range:Vec<std::os::raw::c_int>  = vec![];
    vec_range.reserve(len_range);
    for i in 0..(len_range as u32){
        vec_range.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(range+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_range = vec_range.as_mut_ptr()as *const std::os::raw::c_int;
    let len_precision = (1)as usize;
    let mut vec_precision:Vec<std::os::raw::c_int>  = vec![];
    vec_precision.reserve(len_precision);
    for i in 0..(len_precision as u32){
        vec_precision.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(precision+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_precision = vec_precision.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shadertype:std::os::raw::c_uint,precisiontype:std::os::raw::c_uint,range: *const std::os::raw::c_int,precision: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetShaderPrecisionFormat"))(converted_shadertype,converted_precisiontype,converted_range,converted_precision)
    };
    for(i,value)in vec_range.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(range+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_precision.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(precision+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetShaderSource(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,bufSize:i32,length:u32,source:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_bufSize = bufSize;
    let len_length = (1)as usize;
    let mut vec_length:Vec<std::os::raw::c_int>  = vec![];
    vec_length.reserve(len_length);
    for i in 0..(len_length as u32){
        vec_length.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_length = vec_length.as_mut_ptr()as *const std::os::raw::c_int;
    let len_source = (bufSize)as usize;
    let mut vec_source:Vec<i8>  = vec![];
    vec_source.reserve(len_source);
    for i in 0..(len_source as u32){
        vec_source.push(memory.read::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(source+i*1,)).unwrap()as i8,);
    }let converted_source = vec_source.as_mut_ptr()as *const i8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,bufSize:std::os::raw::c_int,length: *const std::os::raw::c_int,source: *const i8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetShaderSource"))(converted_shader,converted_bufSize,converted_length,converted_source)
    };
    for(i,value)in vec_length.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(length+(i as u32)*4), *value as i32,).unwrap()
    }for(i,value)in vec_source.iter().enumerate(){
        memory.write::<i8>(webrogue_runtime::wiggle::GuestPtr::<i8>::new(source+(i as u32)*1), *value as i8,).unwrap()
    }result
}
pub fn glGetShaderiv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_shader = shader;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetShaderiv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetShaderiv"))(converted_shader,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetTexParameterfv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetTexParameterfv_params_compsize(pname))as usize;
    let mut vec_params:Vec<f32>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+i*4,)).unwrap()as f32,);
    }let converted_params = vec_params.as_mut_ptr()as *const f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetTexParameterfv"))(converted_target,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+(i as u32)*4), *value as f32,).unwrap()
    }result
}
pub fn glGetTexParameteriv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glGetTexParameteriv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetTexParameteriv"))(converted_target,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetUniformfv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,location:i32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_location = location;
    let len_params = (crate::compsize::glGetUniformfv_params_compsize(program,location))as usize;
    let mut vec_params:Vec<f32>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+i*4,)).unwrap()as f32,);
    }let converted_params = vec_params.as_mut_ptr()as *const f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,location:std::os::raw::c_int,params: *const f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetUniformfv"))(converted_program,converted_location,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+(i as u32)*4), *value as f32,).unwrap()
    }result
}
pub fn glGetUniformiv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,location:i32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_program = program;
    let converted_location = location;
    let len_params = (crate::compsize::glGetUniformiv_params_compsize(program,location))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,location:std::os::raw::c_int,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetUniformiv"))(converted_program,converted_location,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glGetVertexAttribPointerv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,pname:u32,pointer:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_pointer = (1)as usize;
    let mut vec_pointer:Vec<*const ()>  = vec![];
    vec_pointer.reserve(len_pointer);
    for i in 0..(len_pointer as u32){
        vec_pointer.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(pointer+i*4,)).unwrap()as *const (),);
    }let converted_pointer = vec_pointer.as_mut_ptr()as *const *const ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,pname:std::os::raw::c_uint,pointer: *const *const (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetVertexAttribPointerv"))(converted_index,converted_pname,converted_pointer)
    };
    for(i,value)in vec_pointer.iter().enumerate(){
        memory.write::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(pointer+(i as u32)*4), *value as u32,).unwrap()
    }result
}
pub fn glGetVertexAttribfv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (4)as usize;
    let mut vec_params:Vec<f32>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+i*4,)).unwrap()as f32,);
    }let converted_params = vec_params.as_mut_ptr()as *const f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetVertexAttribfv"))(converted_index,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+(i as u32)*4), *value as f32,).unwrap()
    }result
}
pub fn glGetVertexAttribiv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,pname:u32,params:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_index = index;
    let converted_pname = pname;
    let len_params = (4)as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *const std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *const std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glGetVertexAttribiv"))(converted_index,converted_pname,converted_params)
    };
    for(i,value)in vec_params.iter().enumerate(){
        memory.write::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+(i as u32)*4), *value as i32,).unwrap()
    }result
}
pub fn glHint(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,mode:u32,) -> (){
    let converted_target = target;
    let converted_mode = mode;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,mode:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glHint"))(converted_target,converted_mode)
    };
    result
}
pub fn glIsBuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,buffer:u32,) -> u32 {
    let converted_buffer = buffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(buffer:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsBuffer"))(converted_buffer)
    };
    result.into()
}
pub fn glIsEnabled(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,cap:u32,) -> u32 {
    let converted_cap = cap;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(cap:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsEnabled"))(converted_cap)
    };
    result.into()
}
pub fn glIsFramebuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,framebuffer:u32,) -> u32 {
    let converted_framebuffer = framebuffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(framebuffer:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsFramebuffer"))(converted_framebuffer)
    };
    result.into()
}
pub fn glIsProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,) -> u32 {
    let converted_program = program;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsProgram"))(converted_program)
    };
    result.into()
}
pub fn glIsRenderbuffer(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,renderbuffer:u32,) -> u32 {
    let converted_renderbuffer = renderbuffer;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(renderbuffer:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsRenderbuffer"))(converted_renderbuffer)
    };
    result.into()
}
pub fn glIsShader(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,shader:u32,) -> u32 {
    let converted_shader = shader;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(shader:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsShader"))(converted_shader)
    };
    result.into()
}
pub fn glIsTexture(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,texture:u32,) -> u32 {
    let converted_texture = texture;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(texture:std::os::raw::c_uint,) -> u32, >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glIsTexture"))(converted_texture)
    };
    result.into()
}
pub fn glLineWidth(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,width:f32,) -> (){
    let converted_width = width;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(width:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glLineWidth"))(converted_width)
    };
    result
}
pub fn glLinkProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,) -> (){
    let converted_program = program;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glLinkProgram"))(converted_program)
    };
    result
}
pub fn glPixelStorei(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,pname:u32,param:i32,) -> (){
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(pname:std::os::raw::c_uint,param:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glPixelStorei"))(converted_pname,converted_param)
    };
    result
}
pub fn glPolygonOffset(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,factor:f32,units:f32,) -> (){
    let converted_factor = factor;
    let converted_units = units;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(factor:f32,units:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glPolygonOffset"))(converted_factor,converted_units)
    };
    result
}
pub fn glReadPixels(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,x:i32,y:i32,width:i32,height:i32,format:u32,_type:u32,pixels:u32,) -> (){
    let mut memory = _memory_factory.make_memory();
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glReadPixels_pixels_compsize(format,_type,width,height))as usize;
    let mut vec_pixels:Vec<u8>  = vec![];
    vec_pixels.reserve(len_pixels);
    for i in 0..(len_pixels as u32){
        vec_pixels.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(pixels+i*1,)).unwrap()as u8,);
    }let converted_pixels = vec_pixels.as_mut_ptr()as *const ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(x:std::os::raw::c_int,y:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,format:std::os::raw::c_uint,_type:std::os::raw::c_uint,pixels: *const (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glReadPixels"))(converted_x,converted_y,converted_width,converted_height,converted_format,converted__type,converted_pixels)
    };
    for(i,value)in vec_pixels.iter().enumerate(){
        memory.write::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(pixels+(i as u32)*1), *value as u8,).unwrap()
    }result
}
pub fn glReleaseShaderCompiler(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,) -> (){
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn() -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glReleaseShaderCompiler"))()
    };
    result
}
pub fn glRenderbufferStorage(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,internalformat:u32,width:i32,height:i32,) -> (){
    let converted_target = target;
    let converted_internalformat = internalformat;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,internalformat:std::os::raw::c_uint,width:std::os::raw::c_int,height:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glRenderbufferStorage"))(converted_target,converted_internalformat,converted_width,converted_height)
    };
    result
}
pub fn glSampleCoverage(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,value:f32,invert:u32,) -> (){
    let converted_value = value;
    let converted_invert = invert as u8;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(value:f32,invert:u8,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glSampleCoverage"))(converted_value,converted_invert)
    };
    result
}
pub fn glScissor(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,x:i32,y:i32,width:i32,height:i32,) -> (){
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(x:std::os::raw::c_int,y:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glScissor"))(converted_x,converted_y,converted_width,converted_height)
    };
    result
}
pub fn glShaderBinary(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,count:i32,shaders:u32,binaryFormat:u32,binary:u32,length:i32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_count = count;
    let len_shaders = (count)as usize;
    let mut vec_shaders:Vec<std::os::raw::c_uint>  = vec![];
    vec_shaders.reserve(len_shaders);
    for i in 0..(len_shaders as u32){
        vec_shaders.push(memory.read::<u32>(webrogue_runtime::wiggle::GuestPtr::<u32>::new(shaders+i*4,)).unwrap()as std::os::raw::c_uint,);
    }let converted_shaders = vec_shaders.as_mut_ptr()as *mut std::os::raw::c_uint;
    let converted_binaryFormat = binaryFormat;
    let len_binary = (length)as usize;
    let mut vec_binary:Vec<u8>  = vec![];
    vec_binary.reserve(len_binary);
    for i in 0..(len_binary as u32){
        vec_binary.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(binary+i*1,)).unwrap()as u8,);
    }let converted_binary = vec_binary.as_mut_ptr()as *mut ();
    let converted_length = length;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(count:std::os::raw::c_int,shaders: *mut std::os::raw::c_uint,binaryFormat:std::os::raw::c_uint,binary: *mut (),length:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glShaderBinary"))(converted_count,converted_shaders,converted_binaryFormat,converted_binary,converted_length)
    };
    result
}
pub fn glStencilFunc(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,func:u32,_ref:i32,mask:u32,) -> (){
    let converted_func = func;
    let converted__ref = _ref;
    let converted_mask = mask;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(func:std::os::raw::c_uint,_ref:std::os::raw::c_int,mask:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilFunc"))(converted_func,converted__ref,converted_mask)
    };
    result
}
pub fn glStencilFuncSeparate(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,face:u32,func:u32,_ref:i32,mask:u32,) -> (){
    let converted_face = face;
    let converted_func = func;
    let converted__ref = _ref;
    let converted_mask = mask;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(face:std::os::raw::c_uint,func:std::os::raw::c_uint,_ref:std::os::raw::c_int,mask:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilFuncSeparate"))(converted_face,converted_func,converted__ref,converted_mask)
    };
    result
}
pub fn glStencilMask(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,mask:u32,) -> (){
    let converted_mask = mask;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(mask:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilMask"))(converted_mask)
    };
    result
}
pub fn glStencilMaskSeparate(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,face:u32,mask:u32,) -> (){
    let converted_face = face;
    let converted_mask = mask;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(face:std::os::raw::c_uint,mask:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilMaskSeparate"))(converted_face,converted_mask)
    };
    result
}
pub fn glStencilOp(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,fail:u32,zfail:u32,zpass:u32,) -> (){
    let converted_fail = fail;
    let converted_zfail = zfail;
    let converted_zpass = zpass;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(fail:std::os::raw::c_uint,zfail:std::os::raw::c_uint,zpass:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilOp"))(converted_fail,converted_zfail,converted_zpass)
    };
    result
}
pub fn glStencilOpSeparate(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,face:u32,sfail:u32,dpfail:u32,dppass:u32,) -> (){
    let converted_face = face;
    let converted_sfail = sfail;
    let converted_dpfail = dpfail;
    let converted_dppass = dppass;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(face:std::os::raw::c_uint,sfail:std::os::raw::c_uint,dpfail:std::os::raw::c_uint,dppass:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glStencilOpSeparate"))(converted_face,converted_sfail,converted_dpfail,converted_dppass)
    };
    result
}
pub fn glTexParameterf(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,param:f32,) -> (){
    let converted_target = target;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,param:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glTexParameterf"))(converted_target,converted_pname,converted_param)
    };
    result
}
pub fn glTexParameterfv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glTexParameterfv_params_compsize(pname))as usize;
    let mut vec_params:Vec<f32>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(params+i*4,)).unwrap()as f32,);
    }let converted_params = vec_params.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glTexParameterfv"))(converted_target,converted_pname,converted_params)
    };
    result
}
pub fn glTexParameteri(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,param:i32,) -> (){
    let converted_target = target;
    let converted_pname = pname;
    let converted_param = param;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,param:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glTexParameteri"))(converted_target,converted_pname,converted_param)
    };
    result
}
pub fn glTexParameteriv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,pname:u32,params:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_pname = pname;
    let len_params = (crate::compsize::glTexParameteriv_params_compsize(pname))as usize;
    let mut vec_params:Vec<std::os::raw::c_int>  = vec![];
    vec_params.reserve(len_params);
    for i in 0..(len_params as u32){
        vec_params.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(params+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_params = vec_params.as_mut_ptr()as *mut std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,pname:std::os::raw::c_uint,params: *mut std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glTexParameteriv"))(converted_target,converted_pname,converted_params)
    };
    result
}
pub fn glTexSubImage2D(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,target:u32,level:i32,xoffset:i32,yoffset:i32,width:i32,height:i32,format:u32,_type:u32,pixels:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_target = target;
    let converted_level = level;
    let converted_xoffset = xoffset;
    let converted_yoffset = yoffset;
    let converted_width = width;
    let converted_height = height;
    let converted_format = format;
    let converted__type = _type;
    let len_pixels = (crate::compsize::glTexSubImage2D_pixels_compsize(format,_type,width,height))as usize;
    let mut vec_pixels:Vec<u8>  = vec![];
    vec_pixels.reserve(len_pixels);
    for i in 0..(len_pixels as u32){
        vec_pixels.push(memory.read::<u8>(webrogue_runtime::wiggle::GuestPtr::<u8>::new(pixels+i*1,)).unwrap()as u8,);
    }let converted_pixels = vec_pixels.as_mut_ptr()as *mut ();
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(target:std::os::raw::c_uint,level:std::os::raw::c_int,xoffset:std::os::raw::c_int,yoffset:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,format:std::os::raw::c_uint,_type:std::os::raw::c_uint,pixels: *mut (),) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glTexSubImage2D"))(converted_target,converted_level,converted_xoffset,converted_yoffset,converted_width,converted_height,converted_format,converted__type,converted_pixels)
    };
    result
}
pub fn glUniform1f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:f32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform1f"))(converted_location,converted_v0)
    };
    result
}
pub fn glUniform1fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*1)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform1fv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform1i(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:i32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform1i"))(converted_location,converted_v0)
    };
    result
}
pub fn glUniform1iv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*1)as usize;
    let mut vec_value:Vec<std::os::raw::c_int>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(value+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_value = vec_value.as_mut_ptr()as *mut std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform1iv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform2f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:f32,v1:f32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:f32,v1:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform2f"))(converted_location,converted_v0,converted_v1)
    };
    result
}
pub fn glUniform2fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*2)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform2fv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform2i(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:i32,v1:i32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:std::os::raw::c_int,v1:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform2i"))(converted_location,converted_v0,converted_v1)
    };
    result
}
pub fn glUniform2iv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*2)as usize;
    let mut vec_value:Vec<std::os::raw::c_int>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(value+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_value = vec_value.as_mut_ptr()as *mut std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform2iv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform3f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:f32,v1:f32,v2:f32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:f32,v1:f32,v2:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform3f"))(converted_location,converted_v0,converted_v1,converted_v2)
    };
    result
}
pub fn glUniform3fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*3)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform3fv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform3i(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:i32,v1:i32,v2:i32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:std::os::raw::c_int,v1:std::os::raw::c_int,v2:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform3i"))(converted_location,converted_v0,converted_v1,converted_v2)
    };
    result
}
pub fn glUniform3iv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*3)as usize;
    let mut vec_value:Vec<std::os::raw::c_int>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(value+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_value = vec_value.as_mut_ptr()as *mut std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform3iv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform4f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:f32,v1:f32,v2:f32,v3:f32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let converted_v3 = v3;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:f32,v1:f32,v2:f32,v3:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform4f"))(converted_location,converted_v0,converted_v1,converted_v2,converted_v3)
    };
    result
}
pub fn glUniform4fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*4)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform4fv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniform4i(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,v0:i32,v1:i32,v2:i32,v3:i32,) -> (){
    let converted_location = location;
    let converted_v0 = v0;
    let converted_v1 = v1;
    let converted_v2 = v2;
    let converted_v3 = v3;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,v0:std::os::raw::c_int,v1:std::os::raw::c_int,v2:std::os::raw::c_int,v3:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform4i"))(converted_location,converted_v0,converted_v1,converted_v2,converted_v3)
    };
    result
}
pub fn glUniform4iv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let len_value = (count*4)as usize;
    let mut vec_value:Vec<std::os::raw::c_int>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<i32>(webrogue_runtime::wiggle::GuestPtr::<i32>::new(value+i*4,)).unwrap()as std::os::raw::c_int,);
    }let converted_value = vec_value.as_mut_ptr()as *mut std::os::raw::c_int;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,value: *mut std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniform4iv"))(converted_location,converted_count,converted_value)
    };
    result
}
pub fn glUniformMatrix2fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,transpose:u32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*4)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,transpose:u8,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniformMatrix2fv"))(converted_location,converted_count,converted_transpose,converted_value)
    };
    result
}
pub fn glUniformMatrix3fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,transpose:u32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*9)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,transpose:u8,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniformMatrix3fv"))(converted_location,converted_count,converted_transpose,converted_value)
    };
    result
}
pub fn glUniformMatrix4fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,location:i32,count:i32,transpose:u32,value:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_location = location;
    let converted_count = count;
    let converted_transpose = transpose as u8;
    let len_value = (count*16)as usize;
    let mut vec_value:Vec<f32>  = vec![];
    vec_value.reserve(len_value);
    for i in 0..(len_value as u32){
        vec_value.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(value+i*4,)).unwrap()as f32,);
    }let converted_value = vec_value.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(location:std::os::raw::c_int,count:std::os::raw::c_int,transpose:u8,value: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUniformMatrix4fv"))(converted_location,converted_count,converted_transpose,converted_value)
    };
    result
}
pub fn glUseProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,) -> (){
    let converted_program = program;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glUseProgram"))(converted_program)
    };
    result
}
pub fn glValidateProgram(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,program:u32,) -> (){
    let converted_program = program;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(program:std::os::raw::c_uint,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glValidateProgram"))(converted_program)
    };
    result
}
pub fn glVertexAttrib1f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,x:f32,) -> (){
    let converted_index = index;
    let converted_x = x;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,x:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib1f"))(converted_index,converted_x)
    };
    result
}
pub fn glVertexAttrib1fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,v:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (1)as usize;
    let mut vec_v:Vec<f32>  = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32){
        vec_v.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v+i*4,)).unwrap()as f32,);
    }let converted_v = vec_v.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,v: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib1fv"))(converted_index,converted_v)
    };
    result
}
pub fn glVertexAttrib2f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,x:f32,y:f32,) -> (){
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,x:f32,y:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib2f"))(converted_index,converted_x,converted_y)
    };
    result
}
pub fn glVertexAttrib2fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,v:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (2)as usize;
    let mut vec_v:Vec<f32>  = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32){
        vec_v.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v+i*4,)).unwrap()as f32,);
    }let converted_v = vec_v.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,v: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib2fv"))(converted_index,converted_v)
    };
    result
}
pub fn glVertexAttrib3f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,x:f32,y:f32,z:f32,) -> (){
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,x:f32,y:f32,z:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib3f"))(converted_index,converted_x,converted_y,converted_z)
    };
    result
}
pub fn glVertexAttrib3fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,v:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (3)as usize;
    let mut vec_v:Vec<f32>  = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32){
        vec_v.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v+i*4,)).unwrap()as f32,);
    }let converted_v = vec_v.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,v: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib3fv"))(converted_index,converted_v)
    };
    result
}
pub fn glVertexAttrib4f(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,x:f32,y:f32,z:f32,w:f32,) -> (){
    let converted_index = index;
    let converted_x = x;
    let converted_y = y;
    let converted_z = z;
    let converted_w = w;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,x:f32,y:f32,z:f32,w:f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib4f"))(converted_index,converted_x,converted_y,converted_z,converted_w)
    };
    result
}
pub fn glVertexAttrib4fv(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,index:u32,v:u32,) -> (){
    let memory = _memory_factory.make_memory();
    let converted_index = index;
    let len_v = (4)as usize;
    let mut vec_v:Vec<f32>  = vec![];
    vec_v.reserve(len_v);
    for i in 0..(len_v as u32){
        vec_v.push(memory.read::<f32>(webrogue_runtime::wiggle::GuestPtr::<f32>::new(v+i*4,)).unwrap()as f32,);
    }let converted_v = vec_v.as_mut_ptr()as *mut f32;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(index:std::os::raw::c_uint,v: *mut f32,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glVertexAttrib4fv"))(converted_index,converted_v)
    };
    result
}
pub fn glViewport(_memory_factory: &mut Box<dyn webrogue_runtime::MemoryFactory>,_context: &mut Context,x:i32,y:i32,width:i32,height:i32,) -> (){
    let converted_x = x;
    let converted_y = y;
    let converted_width = width;
    let converted_height = height;
    let result = unsafe {
        std::mem::transmute::< *const (),unsafe extern "C" fn(x:std::os::raw::c_int,y:std::os::raw::c_int,width:std::os::raw::c_int,height:std::os::raw::c_int,) -> (), >(_context.gfx_context.as_mut().unwrap().video_subsystem.as_mut().unwrap().gl_get_proc_address("glViewport"))(converted_x,converted_y,converted_width,converted_height)
    };
    result
}

pub use crate::mainual_impl::*;

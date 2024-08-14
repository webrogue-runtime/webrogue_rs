use crate::ffi::{self};

fn shader_param_count(pname: u32) -> usize {
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/glGet.xhtml
    match pname {
        // likely missed
        ffi::GL_COMPILE_STATUS => 1,
        // likely missed
        ffi::GL_CONTEXT_PROFILE_MASK => 1,
        // likely missed
        ffi::GL_CONTEXT_RELEASE_BEHAVIOR => 1,
        //     data returns a single value indicating the active multitexture unit. The initial value is ffi::GL_TEXTURE0. See glActiveTexture.
        ffi::GL_ACTIVE_TEXTURE => 1,
        //     data returns a pair of values indicating the range of widths supported for aliased lines. See glLineWidth.
        ffi::GL_ALIASED_LINE_WIDTH_RANGE => 1,
        //     data returns two values: the smallest and largest supported sizes for points. The smallest size must be at most 1, and the largest size must be at least 1.
        ffi::GL_ALIASED_POINT_SIZE_RANGE => 1,
        //     data returns one value, the number of alpha bitplanes in the color buffer of the currently bound draw framebuffer. This is deﬁned only if all color attachments of the draw framebuffer have identical formats, in which case the number of alpha bits of color attachment zero are returned.
        ffi::GL_ALPHA_BITS => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_ARRAY_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_ARRAY_BUFFER_BINDING => 1,
        //     data returns a single boolean value indicating whether blending is enabled. The initial value is ffi::GL_FALSE. See glBlendFunc.
        ffi::GL_BLEND => 1,
        //     data returns four values, the red, green, blue, and alpha values which are the components of the blend color. See glBlendColor.
        ffi::GL_BLEND_COLOR => 4,
        //     data returns one value, the symbolic constant identifying the alpha destination blend function. The initial value is ffi::GL_ZERO. See glBlendFunc and glBlendFuncSeparate.
        ffi::GL_BLEND_DST_ALPHA => 1,
        //     data returns one value, the symbolic constant identifying the RGB destination blend function. The initial value is ffi::GL_ZERO. See glBlendFunc and glBlendFuncSeparate.
        ffi::GL_BLEND_DST_RGB => 1,
        //     data returns one value, a symbolic constant indicating whether the Alpha blend equation is ffi::GL_FUNC_ADD, ffi::GL_FUNC_SUBTRACT, ffi::GL_FUNC_REVERSE_SUBTRACT, ffi::GL_MIN or ffi::GL_MAX. See glBlendEquationSeparate.
        ffi::GL_BLEND_EQUATION_ALPHA => 1,
        //     data returns one value, a symbolic constant indicating whether the RGB blend equation is ffi::GL_FUNC_ADD, ffi::GL_FUNC_SUBTRACT, ffi::GL_FUNC_REVERSE_SUBTRACT, ffi::GL_MIN or ffi::GL_MAX. See glBlendEquationSeparate.
        ffi::GL_BLEND_EQUATION_RGB => 1,
        //     data returns one value, the symbolic constant identifying the alpha source blend function. The initial value is ffi::GL_ONE. See glBlendFunc and glBlendFuncSeparate.
        ffi::GL_BLEND_SRC_ALPHA => 1,
        //     data returns one value, the symbolic constant identifying the RGB source blend function. The initial value is ffi::GL_ONE. See glBlendFunc and glBlendFuncSeparate.
        ffi::GL_BLEND_SRC_RGB => 1,
        //     data returns one value, the number of blue bitplanes in the color buffer of the currently bound draw framebuffer. This is deﬁned only if all color attachments of the draw framebuffer have identical formats, in which case the number of blue bits of color attachment zero are returned.
        ffi::GL_BLUE_BITS => 1,
        //     data returns four values: the red, green, blue, and alpha values used to clear the color buffers. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0
        //     returns the most negative representable integer value. The initial value is (0, 0, 0, 0). See glClearColor.
        ffi::GL_COLOR_CLEAR_VALUE => 4,
        //     data returns four boolean values: the red, green, blue, and alpha write enables for the color buffers. The initial value is (GL_TRUE, ffi::GL_TRUE, ffi::GL_TRUE, ffi::GL_TRUE). See glColorMask.
        ffi::GL_COLOR_WRITEMASK => 4,
        //     data returns a list of symbolic constants of length ffi::GL_NUM_COMPRESSED_TEXTURE_FORMATS indicating which compressed texture formats are available. See glCompressedTexImage2D.
        ffi::GL_COMPRESSED_TEXTURE_FORMATS => ffi::GL_NUM_COMPRESSED_TEXTURE_FORMATS as usize,
        //     data returns one value, the flags with which the context was created (such as debugging functionality).
        ffi::GL_CONTEXT_FLAGS => 1,
        //     data returns one boolean value, indicating if robust buffer access has been enabled at context creation time.
        ffi::GL_CONTEXT_ROBUST_ACCESS => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_COPY_READ_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_COPY_READ_BUFFER_BINDING => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_COPY_WRITE_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_COPY_WRITE_BUFFER_BINDING => 1,
        //     data returns a single boolean value indicating whether polygon culling is enabled. The initial value is ffi::GL_FALSE. See glCullFace.
        ffi::GL_CULL_FACE => 1,
        //     data returns a single value indicating the mode of polygon culling. The initial value is ffi::GL_BACK. See glCullFace.
        ffi::GL_CULL_FACE_MODE => 1,
        //     data returns one value, the name of the program object that is currently active, or 0 if no program object is active. See glUseProgram.
        ffi::GL_CURRENT_PROGRAM => 1,
        //     data returns a single value, the current depth of the debug message group stack.
        ffi::GL_DEBUG_GROUP_STACK_DEPTH => 1,
        //     data returns a single value, the number of messages currently in the debug log.
        ffi::GL_DEBUG_LOGGED_MESSAGES => 1,
        //     data returns a single value, the length, including the NULL terminator, of the oldest message in the debug log.
        ffi::GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH => 1,
        //     data returns one value, the number of bitplanes in the depth buffer of the currently bound framebuffer.
        ffi::GL_DEPTH_BITS => 1,
        //     data returns one value, the value that is used to clear the depth buffer. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0
        //     returns the most negative representable integer value. The initial value is 1. See glClearDepthf.
        ffi::GL_DEPTH_CLEAR_VALUE => 1,
        //     data returns one value, the symbolic constant that indicates the depth comparison function. The initial value is ffi::GL_LESS. See glDepthFunc.
        ffi::GL_DEPTH_FUNC => 1,
        //     data returns two values: the near and far mapping limits for the depth buffer. Integer values, if requested, are linearly mapped from the internal floating-point representation such that 1.0 returns the most positive representable integer value, and −1.0
        //     returns the most negative representable integer value. The initial value is (0, 1). See glDepthRangef.
        ffi::GL_DEPTH_RANGE => 2,
        //     data returns a single boolean value indicating whether depth testing of fragments is enabled. The initial value is ffi::GL_FALSE. See glDepthFunc and glDepthRangef.
        ffi::GL_DEPTH_TEST => 1,
        //     data returns a single boolean value indicating if the depth buffer is enabled for writing. The initial value is ffi::GL_TRUE. See glDepthMask.
        ffi::GL_DEPTH_WRITEMASK => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_DISPATCH_INDIRECT_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_DISPATCH_INDIRECT_BUFFER_BINDING => 1,
        //     data returns a single boolean value indicating whether dithering of fragment colors and indices is enabled. The initial value is ffi::GL_TRUE.
        ffi::GL_DITHER => 1,
        //  i
        //     data returns one value, a symbolic constant indicating which buffers are being drawn to by the corresponding output color. See glDrawBuffers. The initial value of ffi::GL_DRAW_BUFFER0 is ffi::GL_BACK. The initial values of draw buffers for all other output colors is ffi::GL_NONE.
        ffi::GL_DRAW_BUFFER => 1,
        //     data returns one value, the name of the framebuffer object currently bound to the ffi::GL_DRAW_FRAMEBUFFER target. If the default framebuffer is bound, this value will be zero. The initial value is zero. See glBindFramebuffer.
        ffi::GL_DRAW_FRAMEBUFFER_BINDING => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_ELEMENT_ARRAY_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_ELEMENT_ARRAY_BUFFER_BINDING => 1,
        //     data returns a single integer value indicating the number of subpixels bits available in the offset for interpolation. See interpolateAtOffset.
        ffi::GL_FRAGMENT_INTERPOLATION_OFFSET_BITS => 1,
        //     data returns one value, a symbolic constant indicating the mode of the derivative accuracy hint for fragment shaders. The initial value is ffi::GL_DONT_CARE. See glHint.
        ffi::GL_FRAGMENT_SHADER_DERIVATIVE_HINT => 1,
        //     data returns a single value indicating the winding order of polygon front faces. The initial value is ffi::GL_CCW. See glFrontFace.
        ffi::GL_FRONT_FACE => 1,
        //     data returns one value, a symbolic constant indicating the mode of the generate mipmap quality hint. The initial value is ffi::GL_DONT_CARE. See glHint.
        ffi::GL_GENERATE_MIPMAP_HINT => 1,
        //     data returns one value, the number of green bitplanes in the color buffer of the currently bound draw framebuffer. This is deﬁned only if all color attachments of the draw framebuffer have identical formats, in which case the number of green bits of color attachment zero are returned.
        ffi::GL_GREEN_BITS => 1,
        //     data returns one value, corresponding to whether the image bound to the indexed image unit is layered or not.
        ffi::GL_IMAGE_BINDING_LAYERED => 1,
        //     data returns one value, the format chosen by the implementation in which pixels may be read from the color buffer of the currently bound framebuffer in conjunction with ffi::GL_IMPLEMENTATION_COLOR_READ_TYPE. See glReadPixels.
        ffi::GL_IMPLEMENTATION_COLOR_READ_FORMAT => 1,
        //     data returns one value, the type chosen by the implementation with which pixels may be read from the color buffer of the currently bound framebuffer in conjunction with ffi::GL_IMPLEMENTATION_COLOR_READ_FORMAT. See glReadPixels.
        ffi::GL_IMPLEMENTATION_COLOR_READ_TYPE => 1,
        //     data returns one value, the implementation dependent specifc vertex of a primitive that is used to select the rendering layer. If the value returned is ffi::GL_FIRST_VERTEX_CONVENTION, then the selection is always taken from the first vertex in the primitive. If the value returned is ffi::GL_LAST_VERTEX_CONVENTION, then the selection is always taken from the last vertex in the primitive. If the value returned is ffi::GL_UNDEFINED_VERTEX, then the selection is not guaranteed to be taken from any specific vertex in the primitive. Portable applications will therefore assign the same layer for all vertices in a primitive.
        ffi::GL_LAYER_PROVOKING_VERTEX => 1,
        //     data returns one value, the line width as specified with glLineWidth. The initial value is 1.
        ffi::GL_LINE_WIDTH => 1,
        //     data returns one value, the major version number of the OpenGL ES API supported by the current context. This must be 3.
        ffi::GL_MAJOR_VERSION => 1,
        //     data returns one value, a rough estimate of the largest 3D texture that the GL can handle. The value must be at least 256. See glTexImage3D.
        ffi::GL_MAX_3D_TEXTURE_SIZE => 1,
        //     data returns one value. The value indicates the maximum number of layers allowed in an array texture, and must be at least 256. See glTexImage2D.
        ffi::GL_MAX_ARRAY_TEXTURE_LAYERS => 1,
        //     data returns one value, the maximum number of atomic counter buffer binding points. The value must be at least 1. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
        ffi::GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS => 1,
        //     data returns one value, the maximum number of color attachment points in a framebuffer object. The value must be at least 4. See glFramebufferRenderbuffer and glFramebufferTexture2D.
        ffi::GL_MAX_COLOR_ATTACHMENTS => 1,
        //     data returns one value, the maximum number of samples in a color multisample texture.
        ffi::GL_MAX_COLOR_TEXTURE_SAMPLES => 1,
        //     data returns a single value, the maximum number of atomic counters available to all active shaders.
        ffi::GL_MAX_COMBINED_ATOMIC_COUNTERS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by all active shaders.
        ffi::GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns one value, the number of words for compute shader uniform variables in all uniform blocks (including default). The value must be at least 1. See glUniform.
        ffi::GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the number of words for fragment shader uniform variables in all uniform blocks (including default). The value must be at least ffi::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS + ffi::GL_MAX_UNIFORM_BLOCK_SIZE * ffi::GL_MAX_FRAGMENT_UNIFORM_BLOCKS / 4. See glUniform.
        ffi::GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the number of words for geometry shader uniform variables in all uniform blocks (including default). The value must be at least ffi::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS + ffi::GL_MAX_UNIFORM_BLOCK_SIZE * ffi::GL_MAX_GEOMETRY_UNIFORM_BLOCKS / 4. See glUniform.
        ffi::GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the maximum total number of active shader storage blocks that may be accessed by all active shaders.
        ffi::GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the number of words for tesselation control shader uniform variables in all uniform blocks (including default). The value must be at least ffi::GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS + ffi::GL_MAX_UNIFORM_BLOCK_SIZE * ffi::GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS / 4. See glUniform.
        ffi::GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the number of words for tesselation evaluation shader uniform variables in all uniform blocks (including default). The value must be at least ffi::GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS + ffi::GL_MAX_UNIFORM_BLOCK_SIZE * ffi::GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS / 4. See glUniform.
        ffi::GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the all the shader stages combined. If both the vertex shader and the fragment processing stage access the same texture image unit, then that counts as using two texture image units against this limit. The value must be at least 96. See glActiveTexture.
        ffi::GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum number of uniform blocks per program. The value must be at least 60. See glUniformBlockBinding.
        ffi::GL_MAX_COMBINED_UNIFORM_BLOCKS => 1,
        //     data returns one value, the number of words for vertex shader uniform variables in all uniform blocks (including default). The value must be at least . ffi::GL_MAX_VERTEX_UNIFORM_COMPONENTS + ffi::GL_MAX_UNIFORM_BLOCK_SIZE * ffi::GL_MAX_VERTEX_UNIFORM_BLOCKS / 4. See glUniform.
        ffi::GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS => 1,
        //     data returns a single value, the maximum number of atomic counters available to compute shaders.
        ffi::GL_MAX_COMPUTE_ATOMIC_COUNTERS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a compute shader.
        ffi::GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns one value, the maximum supported number of image variables in compute shaders.
        ffi::GL_MAX_COMPUTE_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a compute shader.
        ffi::GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the compute shader. The value may be at least 16. See glActiveTexture.
        ffi::GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum number of uniform blocks per compute shader. The value must be at least 14. See glUniformBlockBinding.
        ffi::GL_MAX_COMPUTE_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a compute shader. The value must be at least 1024. See glUniform.
        ffi::GL_MAX_COMPUTE_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the number of invocations in a single local work group (i.e., the product of the three dimensions) that may be dispatched to a compute shader.
        ffi::GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS => 1,
        //     Accepted by the indexed versions of glGet. data the maximum number of work groups that may be dispatched to a compute shader. Indices 0, 1, and 2 correspond to the X, Y and Z dimensions, respectively.
        ffi::GL_MAX_COMPUTE_WORK_GROUP_COUNT => panic!(),
        //     Accepted by the indexed versions of glGet. data the maximum size of a work groups that may be used during compilation of a compute shader. Indices 0, 1, and 2 correspond to the X, Y and Z dimensions, respectively.
        ffi::GL_MAX_COMPUTE_WORK_GROUP_SIZE => panic!(),
        //     data returns one value. The value gives a rough estimate of the largest cube-map texture that the GL can handle. The value must be at least 2048. See glTexImage2D.
        ffi::GL_MAX_CUBE_MAP_TEXTURE_SIZE => 1,
        //     data returns a single value, the maximum depth of the debug message group stack.
        ffi::GL_MAX_DEBUG_GROUP_STACK_DEPTH => 1,
        //     data returns a single value, the maximum number of messages stored in the debug message log.
        ffi::GL_MAX_DEBUG_LOGGED_MESSAGES => 1,
        //     data returns a single value, the maximum length of a debug message string, including its null terminator.
        ffi::GL_MAX_DEBUG_MESSAGE_LENGTH => 1,
        //     params returns one value, the maximum number of samples in a depth/stencil multisample texture.
        ffi::GL_MAX_DEPTH_TEXTURE_SAMPLES => 1,
        //     data returns one value, the maximum number of simultaneous outputs that may be written in a fragment shader. The value must be at least 4. See glDrawBuffers.
        ffi::GL_MAX_DRAW_BUFFERS => 1,
        //     data returns one value, the maximum index supported by the implementation. The value must be at least 224−1
        //     .
        ffi::GL_MAX_ELEMENT_INDEX => 1,
        //     data returns one value, the recommended maximum number of vertex array indices. See glDrawRangeElements.
        ffi::GL_MAX_ELEMENTS_INDICES => 1,
        //     data returns one value, the recommended maximum number of vertex array vertices. See glDrawRangeElements.
        ffi::GL_MAX_ELEMENTS_VERTICES => 1,
        //     data returns a single value, the maximum number of atomic counters available to fragment shaders. In GL ES 3.1, the minimum value is 0. In GL ES 3.2, the minimum value is 8.
        ffi::GL_MAX_FRAGMENT_ATOMIC_COUNTERS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a fragment shader. In GL ES 3.1, the minimum value is 0. In GL ES 3.2, the minimum value is 1.
        ffi::GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns one value, the maximum supported number of image variables in fragment shaders. In GL ES 3.1, the minimum value is 0. In GL ES 3.2, the minimum value is 4.
        ffi::GL_MAX_FRAGMENT_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of components of the inputs read by the fragment shader, which must be at least 60.
        ffi::GL_MAX_FRAGMENT_INPUT_COMPONENTS => 1,
        //     data returns a single floating-point value indicating the maximum valid offset for interpolation. See interpolateAtOffset.
        ffi::GL_MAX_FRAGMENT_INTERPOLATION_OFFSET => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a fragment shader. In GL ES 3.1, the minimum value is 0. In GL ES 3.2, the minimum value is 4.
        ffi::GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum number of uniform blocks per fragment shader. The value must be at least 12. See glUniformBlockBinding.
        ffi::GL_MAX_FRAGMENT_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a fragment shader. The value must be at least 896. See glUniform.
        ffi::GL_MAX_FRAGMENT_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the maximum number of vector floating-point, integer, or boolean values that can be held in uniform variable storage for a fragment shader. The value must be at least 224. See glUniform.
        ffi::GL_MAX_FRAGMENT_UNIFORM_VECTORS => 1,
        //     data returns one value, the maximum height for a framebuffer that has no attachments, which must be at least 16384. See glFramebufferParameteri.
        ffi::GL_MAX_FRAMEBUFFER_HEIGHT => 1,
        //     data returns one value, the maximum number of layers for a framebuffer that has no attachments, which must be at least 256. See glFramebufferParameteri.
        ffi::GL_MAX_FRAMEBUFFER_LAYERS => 1,
        //     data returns one value, the maximum samples in a framebuffer that has no attachments, which must be at least 4. See glFramebufferParameteri.
        ffi::GL_MAX_FRAMEBUFFER_SAMPLES => 1,
        //     data returns one value, the maximum width for a framebuffer that has no attachments, which must be at least 16384. See glFramebufferParameteri.
        ffi::GL_MAX_FRAMEBUFFER_WIDTH => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a geometry shader.
        ffi::GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns a single value, the maximum number of atomic counters available to geometry shaders.
        ffi::GL_MAX_GEOMETRY_ATOMIC_COUNTERS => 1,
        //     data returns one value, the maximum supported number of image variables in geometry shaders.
        ffi::GL_MAX_GEOMETRY_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of components of inputs read by a geometry shader, which must be at least 64.
        ffi::GL_MAX_GEOMETRY_INPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of components of outputs written by a geometry shader, which must be at least 64.
        ffi::GL_MAX_GEOMETRY_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of vertices emit by a geometry shader, which must be at least 256.
        ffi::GL_MAX_GEOMETRY_OUTPUT_VERTICES => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a geometry shader.
        ffi::GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum supported number of invocations per primitive of a geometry shader.
        ffi::GL_MAX_GEOMETRY_SHADER_INVOCATIONS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the geometry shader. The value must be at least 16. See glActiveTexture.
        ffi::GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum total number of components of active outputs for all vertices written by a geometry shader, which must be at least 1024.
        ffi::GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of uniform blocks per geometry shader. The value must be at least 12. See glUniformBlockBinding.
        ffi::GL_MAX_GEOMETRY_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a geometry shader. The value must be at least 1024. See glUniform.
        ffi::GL_MAX_GEOMETRY_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the maximum number of samples supported in integer format multisample buffers.
        ffi::GL_MAX_INTEGER_SAMPLES => 1,
        //     data returns a single value, the maximum length of a label string.
        ffi::GL_MAX_LABEL_LENGTH => 1,
        //     data returns one value, the maximum texel offset allowed in a texture lookup, which must be at least 7.
        ffi::GL_MAX_PROGRAM_TEXEL_OFFSET => 1,
        //     data returns one value. The value indicates the maximum supported size for renderbuffers and must be at least 2048. See glFramebufferRenderbuffer.
        ffi::GL_MAX_RENDERBUFFER_SIZE => 1,
        //     data returns one value, the maximum number of sample mask words.
        ffi::GL_MAX_SAMPLE_MASK_WORDS => 1,
        //     data returns one value. The value indicates the maximum supported number of samples for multisampling. The value must be at least 4. See glGetInternalformativ.
        ffi::GL_MAX_SAMPLES => 1,
        //     data returns one value, the maximum glWaitSync timeout interval.
        ffi::GL_MAX_SERVER_WAIT_TIMEOUT => 1,
        //     data returns one value, the maximum size in basic machine units of a shader storage block. The value must be at least 227
        //     .
        ffi::GL_MAX_SHADER_STORAGE_BLOCK_SIZE => 1,
        //     data returns one value, the maximum number of shader storage buffer binding points on the context, which must be at least 8.
        ffi::GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a tesselation control shader.
        ffi::GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns a single value, the maximum number of atomic counters available to tessellation control shaders.
        ffi::GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS => 1,
        //     data returns one value, the maximum supported number of image variables in tesselation control shaders.
        ffi::GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of components of inputs read by a tesselation control shader, which must be at least 64.
        ffi::GL_MAX_TESS_CONTROL_INPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of components of outputs written by a tesselation control shader, which must be at least 64.
        ffi::GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a tessellation control shader.
        ffi::GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the tesselation control shader. The value may be at least 16. See glActiveTexture.
        ffi::GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum total number of components of active outputs for all vertices written by a tesselation control shader, including per-vertex and per-patch outputs, which must be at least 2048.
        ffi::GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of uniform blocks per tesselation control shader. The value must be at least 12. See glUniformBlockBinding.
        ffi::GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a tesselation control shader. The value must be at least 1024. See glUniform.
        ffi::GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a tesselation evaluation shader.
        ffi::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns a single value, the maximum number of atomic counters available to tessellation evaluation shaders.
        ffi::GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS => 1,
        //     data returns one value, the maximum supported number of image variables in tesselation evaluation shaders.
        ffi::GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of components of inputs read by a tesselation evaluation shader, which must be at least 64.
        ffi::GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of components of outputs written by a tesselation evaluation shader, which must be at least 64.
        ffi::GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a tessellation evaluation shader.
        ffi::GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the tesselation evaluation shader. The value may be at least 16. See glActiveTexture.
        ffi::GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum number of uniform blocks per tesselation evaluation shader. The value must be at least 12. See glUniformBlockBinding.
        ffi::GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a tesselation evaluation shader. The value must be at least 1024. See glUniform.
        ffi::GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS => 1,
        //     data returns a single value, the maximum tessellation level supported by the tesselation primitive generator.
        ffi::GL_MAX_TESS_GEN_LEVEL => 1,
        //     data returns one value, the maximum number of components of per-patch outputs written by a tesselation control shader, which must be at least 120.
        ffi::GL_MAX_TESS_PATCH_COMPONENTS => 1,
        //     data returns one value. The value gives the maximum number of texels allowed in the texel array of a texture buffer object. Value must be at least 65536.
        ffi::GL_MAX_TEXTURE_BUFFER_SIZE => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the fragment shader. The value must be at least 16. See glActiveTexture.
        ffi::GL_MAX_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum, absolute value of the texture level-of-detail bias. The value must be at least 2.0.
        ffi::GL_MAX_TEXTURE_LOD_BIAS => 1,
        //     data returns one value. The value gives a rough estimate of the largest texture that the GL can handle. The value must be at least 2048. See glTexImage2D.
        ffi::GL_MAX_TEXTURE_SIZE => 1,
        //     data returns one value, the maximum number of components which can be written to a single transform feedback buffer in interleaved mode. The value must be at least 64. See glTransformFeedbackVaryings.
        ffi::GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS => 1,
        //     data returns one value, the maximum separate attributes or outputs which can be captured in separate transform feedback mode. The value must be at least 4. See glTransformFeedbackVaryings.
        ffi::GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS => 1,
        //     data returns one value, the maximum number of components which can be written per attribute or output in separate transform feedback mode. The value must be at least 4. See glTransformFeedbackVaryings.
        ffi::GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS => 1,
        //     data returns one value, the maximum size in basic machine units of a uniform block. The value must be at least 16384. See glUniformBlockBinding.
        ffi::GL_MAX_UNIFORM_BLOCK_SIZE => 1,
        //     data returns one value, the maximum number of uniform buffer binding points on the context, which must be at least 72.
        ffi::GL_MAX_UNIFORM_BUFFER_BINDINGS => 1,
        //     data returns one value, the maximum number of explicitly assignable uniform locations, which must be at least 1024.
        ffi::GL_MAX_UNIFORM_LOCATIONS => 1,
        //     data returns one value, the number components for varying variables, which must be at least 60.
        ffi::GL_MAX_VARYING_COMPONENTS => 1,
        //     data returns one value, the maximum number of interpolators available for processing varying variables used by vertex and fragment shaders. This value represents the number of vector values that can be interpolated; varying variables declared as matrices and arrays will consume multiple interpolators. The value must be at least 15.
        ffi::GL_MAX_VARYING_VECTORS => 1,
        //     data returns a single value, the maximum number of atomic counters available to vertex shaders.
        ffi::GL_MAX_VERTEX_ATOMIC_COUNTERS => 1,
        //     data returns a single value, the maximum number of atomic counter buffers that may be accessed by a vertex shader.
        ffi::GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS => 1,
        //     data returns a single integer value containing the maximum number of vertex buffers that may be bound.
        ffi::GL_MAX_VERTEX_ATTRIB_BINDINGS => 1,
        //     data returns a single integer value containing the maximum offset that may be added to a vertex binding offset.
        ffi::GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET => 1,
        //     data returns one value, the maximum number of 4-component generic vertex attributes accessible to a vertex shader. The value must be at least 16. See glVertexAttrib.
        ffi::GL_MAX_VERTEX_ATTRIBS => 1,
        //     data returns one value, the maximum supported number of image variables in vertex shaders.
        ffi::GL_MAX_VERTEX_IMAGE_UNIFORMS => 1,
        //     data returns one value, the maximum number of active shader storage blocks that may be accessed by a vertex shader.
        ffi::GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS => 1,
        //     data returns one value, the maximum supported texture image units that can be used to access texture maps from the vertex shader. The value may be at least 16. See glActiveTexture.
        ffi::GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS => 1,
        //     data returns one value, the maximum number of components of output written by a vertex shader, which must be at least 64.
        ffi::GL_MAX_VERTEX_OUTPUT_COMPONENTS => 1,
        //     data returns one value, the maximum number of uniform blocks per vertex shader. The value must be at least 12. See glUniformBlockBinding.
        ffi::GL_MAX_VERTEX_UNIFORM_BLOCKS => 1,
        //     data returns one value, the maximum number of individual floating-point, integer, or boolean values that can be held in uniform variable storage for a vertex shader. The value must be at least 1024. See glUniform.
        ffi::GL_MAX_VERTEX_UNIFORM_COMPONENTS => 1,
        //     data returns one value, the maximum number of vector floating-point, integer, or boolean values that can be held in uniform variable storage for a vertex shader. The value must be at least 256. See glUniform.
        ffi::GL_MAX_VERTEX_UNIFORM_VECTORS => 1,
        //     data returns two values: the maximum supported width and height of the viewport. These must be at least as large as the visible dimensions of the display being rendered to. See glViewport.
        ffi::GL_MAX_VIEWPORT_DIMS => 2,
        //     data returns a single floating-point value indicating the minimum valid offset for interpolation. See interpolateAtOffset.
        ffi::GL_MIN_FRAGMENT_INTERPOLATION_OFFSET => 1,
        //     data returns one value, the minimum texel offset allowed in a texture lookup, which must be at most -8.
        ffi::GL_MIN_PROGRAM_TEXEL_OFFSET => 1,
        //     data returns a single floating-point value indicating the minimum sample shading fraction. See glMinSampleShading.
        ffi::GL_MIN_SAMPLE_SHADING_VALUE => 1,
        //     data returns one value, the minor version number of the OpenGL ES API supported by the current context.
        ffi::GL_MINOR_VERSION => 1,
        //     data returns a pair of values indicating the range of widths supported for lines drawn when ffi::GL_SAMPLE_BUFFERS is one. See glLineWidth.
        ffi::GL_MULTISAMPLE_LINE_WIDTH_RANGE => 1,
        //     data returns a single integer value indicating the number of available compressed texture formats. The minimum value is 38. See glCompressedTexImage2D.
        ffi::GL_NUM_COMPRESSED_TEXTURE_FORMATS => 1,
        //     data returns one value, the number of extensions supported by the GL implementation for the current context. See glGetString.
        ffi::GL_NUM_EXTENSIONS => 1,
        //     data returns a single integer value indicating the number of available program binary formats. The minimum value is 0. See glProgramBinary.
        ffi::GL_NUM_PROGRAM_BINARY_FORMATS => 1,
        //     data returns a single integer value indicating the number of available shader binary formats. The minimum value is 0. See glShaderBinary.
        ffi::GL_NUM_SHADER_BINARY_FORMATS => 1,
        //     data returns one value, the byte alignment used for writing pixel data to memory. The initial value is 4. See glPixelStorei.
        ffi::GL_PACK_ALIGNMENT => 1,
        //     data returns one value, the row length used for writing pixel data to memory. The initial value is 0. See glPixelStorei.
        ffi::GL_PACK_ROW_LENGTH => 1,
        //     data returns one value, the number of pixel locations skipped before the first pixel is written into memory. The initial value is 0. See glPixelStorei.
        ffi::GL_PACK_SKIP_PIXELS => 1,
        //     data returns one value, the number of rows of pixel locations skipped before the first pixel is written into memory. The initial value is 0. See glPixelStorei.
        ffi::GL_PACK_SKIP_ROWS => 1,
        //     data returns one value, the number of vertices in an input patch. The initial value is 3. See glPatchParameteri.
        ffi::GL_PATCH_VERTICES => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_PIXEL_PACK_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_PIXEL_PACK_BUFFER_BINDING => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_PIXEL_UNPACK_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_PIXEL_UNPACK_BUFFER_BINDING => 1,
        //     data returns one value, the scaling factor used to determine the variable offset that is added to the depth value of each fragment generated when a polygon is rasterized. The initial value is 0. See glPolygonOffset.
        ffi::GL_POLYGON_OFFSET_FACTOR => 1,
        //     data returns a single boolean value indicating whether polygon offset is enabled for polygons. The initial value is ffi::GL_FALSE. See glPolygonOffset.
        ffi::GL_POLYGON_OFFSET_FILL => 1,
        //     data returns one value. This value is multiplied by an implementation-specific value and then added to the depth value of each fragment generated when a polygon is rasterized. The initial value is 0. See glPolygonOffset.
        ffi::GL_POLYGON_OFFSET_UNITS => 1,
        //     data returns eight values minX
        // , minY, minZ, minW, and maxX, maxY, maxZ, maxW
        //     corresponding to the clip space coordinates of the primitive bounding box. See glPrimitiveBoundingBox.
        ffi::GL_PRIMITIVE_BOUNDING_BOX => 8,
        //     data returns a single boolean value indicating whether primitive restart with a fixed index is enabled. The initial value is ffi::GL_FALSE.
        ffi::GL_PRIMITIVE_RESTART_FIXED_INDEX => 1,
        //     data returns a list of symbolic constants of length ffi::GL_NUM_PROGRAM_BINARY_FORMATS indicating which program binary formats are available. See glProgramBinary.
        ffi::GL_PROGRAM_BINARY_FORMATS => ffi::GL_NUM_PROGRAM_BINARY_FORMATS as usize,
        //     data a single value, the name of the currently bound program pipeline object, or zero if no program pipeline object is bound. See glBindProgramPipeline.
        ffi::GL_PROGRAM_PIPELINE_BINDING => 1,
        //     data returns one value, a single boolean value indicating whether primitives are discarded immediately before the rasterization stage, but after the optional transform feedback stage. See glEnable.
        ffi::GL_RASTERIZER_DISCARD => 1,
        //     data returns one value, a symbolic constant indicating which color buffer is selected for reading. The initial value is ffi::GL_BACK. See glReadPixels.
        ffi::GL_READ_BUFFER => 1,
        //     data returns one value, the name of the framebuffer object currently bound to the ffi::GL_READ_FRAMEBUFFER target. If the default framebuffer is bound, this value will be zero. The initial value is zero. See glBindFramebuffer.
        ffi::GL_READ_FRAMEBUFFER_BINDING => 1,
        //     data returns one value, the number of red bitplanes in the color buffer of the currently bound draw framebuffer. This is deﬁned only if all color attachments of the draw framebuffer have identical formats, in which case the number of red bits of color attachment zero are returned.
        ffi::GL_RED_BITS => 1,
        //     data returns a single value, the name of the renderbuffer object currently bound to the target ffi::GL_RENDERBUFFER. If no renderbuffer object is bound to this target, 0 is returned. The initial value is 0. See glBindRenderbuffer.
        ffi::GL_RENDERBUFFER_BINDING => 1,
        //     data returns a single value, the behaviour of reset notification. Valid values are ffi::GL_NO_RESET_NOTIFICATION, indicating that no reset notification events will be provided by the implementation, or ffi::GL_LOSE_CONTEXT_ON_RESET, indicating that a reset will result in the loss of graphics context. This loss can be found by querying glGetGraphicsResetStatus.
        ffi::GL_RESET_NOTIFICATION_STRATEGY => 1,
        //     data returns a single boolean value indicating whether modification of sample coverage based on alpha is enabled. The initial value is ffi::GL_FALSE. See glSampleCoverage.
        ffi::GL_SAMPLE_ALPHA_TO_COVERAGE => 1,
        //     data returns a single integer value indicating the number of sample buffers associated with the framebuffer. See glSampleCoverage.
        ffi::GL_SAMPLE_BUFFERS => 1,
        //     data returns a single boolean value indicating whether modification of sample coverage based on the value specified by glSampleCoverage is enabled. The initial value is ffi::GL_FALSE.
        ffi::GL_SAMPLE_COVERAGE => 1,
        //     data returns a single boolean value indicating if the temporary coverage value should be inverted. See glSampleCoverage.
        ffi::GL_SAMPLE_COVERAGE_INVERT => 1,
        //     data returns a single positive floating-point value indicating the current sample coverage value. See glSampleCoverage.
        ffi::GL_SAMPLE_COVERAGE_VALUE => 1,
        //     params returns one value indicating the current sample mask value. See glSampleMaski.
        ffi::GL_SAMPLE_MASK_VALUE => 1,
        //     data returns a single integer value indicating whether sample rate shading is enabled. See glEnable.
        ffi::GL_SAMPLE_SHADING => 1,
        //     data returns a single value, the name of the sampler object currently bound to the active texture unit. The initial value is 0. See glBindSampler.
        ffi::GL_SAMPLER_BINDING => 1,
        //     data returns a single integer value indicating the coverage mask size. See glSampleCoverage.
        ffi::GL_SAMPLES => 1,
        //     data returns four values: the x
        // and y window coordinates of the scissor box, followed by its width and height. Initially the x and y
        //     window coordinates are both 0 and the width and height are set to the size of the window. See glScissor.
        ffi::GL_SCISSOR_BOX => 4,
        //     data returns a single boolean value indicating whether scissoring is enabled. The initial value is ffi::GL_FALSE. See glScissor.
        ffi::GL_SCISSOR_TEST => 1,
        //     data returns a list of symbolic constants of length ffi::GL_NUM_SHADER_BINARY_FORMATS indicating which shader binary formats are available. See glShaderBinary.
        ffi::GL_SHADER_BINARY_FORMATS => ffi::GL_NUM_SHADER_BINARY_FORMATS as usize,
        //     data returns a single boolean value indicating whether a shader compiler is supported. This value is always ffi::GL_TRUE. See glCompileShader.
        ffi::GL_SHADER_COMPILER => 1,
        //     When used with non-indexed variants of glGet (such as glGetIntegerv), data returns a single value, the name of the buffer object currently bound to the target ffi::GL_SHADER_STORAGE_BUFFER. If no buffer object is bound to this target, 0 is returned. When used with indexed variants of glGet (such as glGetIntegeri_v), data returns a single value, the name of the buffer object bound to the indexed shader storage buffer binding points. The initial value is 0 for all targets. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
        ffi::GL_SHADER_STORAGE_BUFFER_BINDING => 1,
        //     data returns a single value, the minimum required alignment for shader storage buffer sizes and offset. The initial value is 1. See glShaderStorateBlockBinding.
        ffi::GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT => 1,
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the size of the binding range for each indexed shader storage buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
        ffi::GL_SHADER_STORAGE_BUFFER_SIZE => 1,
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the start offset of the binding range for each indexed shader storage buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
        ffi::GL_SHADER_STORAGE_BUFFER_START => 1,
        //     data returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test fails. The initial value is ffi::GL_KEEP. See glStencilOpSeparate.
        ffi::GL_STENCIL_BACK_FAIL => 1,
        //     data returns one value, a symbolic constant indicating what function is used for back-facing polygons to compare the stencil reference value with the stencil buffer value. The initial value is ffi::GL_ALWAYS. See glStencilFuncSeparate.
        ffi::GL_STENCIL_BACK_FUNC => 1,
        //     data returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test passes, but the depth test fails. The initial value is ffi::GL_KEEP. See glStencilOpSeparate.
        ffi::GL_STENCIL_BACK_PASS_DEPTH_FAIL => 1,
        //     data returns one value, a symbolic constant indicating what action is taken for back-facing polygons when the stencil test passes and the depth test passes. The initial value is ffi::GL_KEEP. See glStencilOpSeparate.
        ffi::GL_STENCIL_BACK_PASS_DEPTH_PASS => 1,
        //     data returns one value, the reference value that is compared with the contents of the stencil buffer for back-facing polygons. The initial value is 0. See glStencilFuncSeparate.
        ffi::GL_STENCIL_BACK_REF => 1,
        //     data returns one value, the mask that is used for back-facing polygons to mask both the stencil reference value and the stencil buffer value before they are compared. The initial value is all 1's. See glStencilFuncSeparate.
        ffi::GL_STENCIL_BACK_VALUE_MASK => 1,
        //     data returns one value, the mask that controls writing of the stencil bitplanes for back-facing polygons. The initial value is all 1's. See glStencilMaskSeparate.
        ffi::GL_STENCIL_BACK_WRITEMASK => 1,
        //     data returns one value, the number of bitplanes in the stencil buffer of the currently bound framebuffer.
        ffi::GL_STENCIL_BITS => 1,
        //     data returns one value, the index to which the stencil bitplanes are cleared. The initial value is 0. See glClearStencil.
        ffi::GL_STENCIL_CLEAR_VALUE => 1,
        //     data returns one value, a symbolic constant indicating what action is taken when the stencil test fails. The initial value is ffi::GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
        ffi::GL_STENCIL_FAIL => 1,
        //     data returns one value, a symbolic constant indicating what function is used to compare the stencil reference value with the stencil buffer value. The initial value is ffi::GL_ALWAYS. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
        ffi::GL_STENCIL_FUNC => 1,
        //     data returns one value, a symbolic constant indicating what action is taken when the stencil test passes, but the depth test fails. The initial value is ffi::GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
        ffi::GL_STENCIL_PASS_DEPTH_FAIL => 1,
        //     data returns one value, a symbolic constant indicating what action is taken when the stencil test passes and the depth test passes. The initial value is ffi::GL_KEEP. See glStencilOp. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilOpSeparate.
        ffi::GL_STENCIL_PASS_DEPTH_PASS => 1,
        //     data returns one value, the reference value that is compared with the contents of the stencil buffer. The initial value is 0. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
        ffi::GL_STENCIL_REF => 1,
        //     data returns a single boolean value indicating whether stencil testing of fragments is enabled. The initial value is ffi::GL_FALSE. See glStencilFunc and glStencilOp.
        ffi::GL_STENCIL_TEST => 1,
        //     data returns one value, the mask that is used to mask both the stencil reference value and the stencil buffer value before they are compared. The initial value is all 1's. See glStencilFunc. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilFuncSeparate.
        ffi::GL_STENCIL_VALUE_MASK => 1,
        //     data returns one value, the mask that controls writing of the stencil bitplanes. The initial value is all 1's. See glStencilMask. This stencil state only affects non-polygons and front-facing polygons. Back-facing polygons use separate stencil state. See glStencilMaskSeparate.
        ffi::GL_STENCIL_WRITEMASK => 1,
        //     data returns one value, an estimate of the number of bits of subpixel resolution that are used to position rasterized geometry in window coordinates. The value must be at least 4.
        ffi::GL_SUBPIXEL_BITS => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_2D. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_2D => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_2D_ARRAY. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_2D_ARRAY => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_3D. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_3D => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_BUFFER. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_BUFFER => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_CUBE_MAP. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_CUBE_MAP => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_2D_MULTISAMPLE. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_2D_MULTISAMPLE => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_2D_MULTISAMPLE_ARRAY. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY => 1,
        //     data returns a single value, the name of the texture currently bound to the target ffi::GL_TEXTURE_CUBE_MAP_ARRAY. The initial value is 0. See glBindTexture.
        ffi::GL_TEXTURE_BINDING_CUBE_MAP_ARRAY => 1,
        //     data returns a single value, the name of the buffer object currently bound to the target ffi::GL_TEXTURE_BUFFER. If no buffer object is bound to this target, 0 is returned. The initial value is 0. See glBindBuffer.
        ffi::GL_TEXTURE_BUFFER_BINDING => 1,
        //     data returns a single value, the minimum required alignment for texture buffer sizes and offset. The initial value is 1. See glUniformBlockBinding.
        ffi::GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT => 1,
        //     data returns a single value, the name of the transform feedback object currently bound to the ffi::GL_TRANSFORM_FEEDBACK target. If no transform feedback object is bound to this target, 0 is returned. The initial value is 0. See glBindTransformFeedback.
        ffi::GL_TRANSFORM_FEEDBACK_BINDING => 1,
        //     data returns a single boolean value indicating if the currently bound transform feedback object is active. See glBeginTransformFeedback and glResumeTransformFeedback.
        ffi::GL_TRANSFORM_FEEDBACK_ACTIVE => 1,
        //     When used with non-indexed variants of glGet (such as glGetIntegerv), data returns a single value, the name of the buffer object currently bound to the target ffi::GL_TRANSFORM_FEEDBACK_BUFFER. If no buffer object is bound to this target, 0 is returned. When used with indexed variants of glGet (such as glGetIntegeri_v), data returns a single value, the name of the buffer object bound to the indexed transform feedback attribute stream. The initial value is 0 for all targets. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
        ffi::GL_TRANSFORM_FEEDBACK_BUFFER_BINDING => 1,
        //     data returns a single boolean value indicating if the currently bound transform feedback object is paused. See glPauseTransformFeedback.
        ffi::GL_TRANSFORM_FEEDBACK_PAUSED => 1,
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the size of the binding range for each transform feedback attribute stream. The initial value is 0 for all streams. See glBindBufferRange.
        ffi::GL_TRANSFORM_FEEDBACK_BUFFER_SIZE => panic!(),
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the start offset of the binding range for each transform feedback attribute stream. The initial value is 0 for all streams. See glBindBufferRange.
        ffi::GL_TRANSFORM_FEEDBACK_BUFFER_START => panic!(),
        //     When used with non-indexed variants of glGet (such as glGetIntegerv), data returns a single value, the name of the buffer object currently bound to the target ffi::GL_UNIFORM_BUFFER. If no buffer object is bound to this target, 0 is returned. When used with indexed variants of glGet (such as glGetIntegeri_v), data returns a single value, the name of the buffer object bound to the indexed uniform buffer binding point. The initial value is 0 for all targets. See glBindBuffer, glBindBufferBase, and glBindBufferRange.
        ffi::GL_UNIFORM_BUFFER_BINDING => 1,
        //     data returns a single value, the minimum required alignment for uniform buffer sizes and offset. The initial value is 1. See glUniformBlockBinding.
        ffi::GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT => 1,
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the size of the binding range for each indexed uniform buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
        ffi::GL_UNIFORM_BUFFER_SIZE => panic!(),
        //     When used with indexed variants of glGet (such as glGetInteger64i_v), data returns a single value, the start offset of the binding range for each indexed uniform buffer binding. The initial value is 0 for all bindings. See glBindBufferRange.
        ffi::GL_UNIFORM_BUFFER_START => panic!(),
        //     data returns one value, the byte alignment used for reading pixel data from memory. The initial value is 4. See glPixelStorei.
        ffi::GL_UNPACK_ALIGNMENT => 1,
        //     data returns one value, the image height used for reading pixel data from memory. The initial is 0. See glPixelStorei.
        ffi::GL_UNPACK_IMAGE_HEIGHT => 1,
        //     data returns one value, the row length used for reading pixel data from memory. The initial value is 0. See glPixelStorei.
        ffi::GL_UNPACK_ROW_LENGTH => 1,
        //     data returns one value, the number of pixel images skipped before the first pixel is read from memory. The initial value is 0. See glPixelStorei.
        ffi::GL_UNPACK_SKIP_IMAGES => 1,
        //     data returns one value, the number of pixel locations skipped before the first pixel is read from memory. The initial value is 0. See glPixelStorei.
        ffi::GL_UNPACK_SKIP_PIXELS => 1,
        //     data returns one value, the number of rows of pixel locations skipped before the first pixel is read from memory. The initial value is 0. See glPixelStorei.
        ffi::GL_UNPACK_SKIP_ROWS => 1,
        //     data returns a single value, the name of the vertex array object currently bound. If no vertex array object is bound, 0 is returned. The initial value is 0. See glBindVertexArray.
        ffi::GL_VERTEX_ARRAY_BINDING => 1,
        //     Accepted by the indexed forms. data returns a single integer value representing the instance step divisor of the first element in the bound buffer's data store for vertex attribute bound to index.
        ffi::GL_VERTEX_BINDING_DIVISOR => panic!(),
        //     Accepted by the indexed forms. data returns a single integer value representing the byte offset of the first element in the bound buffer's data store for vertex attribute bound to index.
        ffi::GL_VERTEX_BINDING_OFFSET => panic!(),
        //     Accepted by the indexed forms. data returns a single integer value representing the byte offset between the start of each element in the bound buffer's data store for vertex attribute bound to index.
        ffi::GL_VERTEX_BINDING_STRIDE => panic!(),
        //     data returns four values: the x
        // and y window coordinates of the viewport, followed by its width and height. Initially the x and y window coordinates are both set to 0, and the width and height are set to the width and height of the window into which the GL will do its rendering. See glViewport.
        ffi::GL_VIEWPORT => 4,
        _ => panic!(),
    }
}

#[allow(non_snake_case)]
pub(crate) fn glGetShaderiv_params_compsize(_pname: u32) -> usize {
    1
}

#[allow(non_snake_case)]
pub(crate) fn glGetProgramiv_params_compsize(_pname: u32) -> usize {
    1
}

#[allow(non_snake_case)]
pub(crate) fn glGetBooleanv_data_compsize(pname: u32) -> usize {
    shader_param_count(pname)
}

#[allow(non_snake_case)]
pub(crate) fn glGetBufferParameteriv_params_compsize(_pname: u32) -> usize {
    1
}

#[allow(non_snake_case)]
pub(crate) fn glGetFloatv_data_compsize(pname: u32) -> usize {
    shader_param_count(pname)
}

#[allow(non_snake_case)]
pub(crate) fn glGetFramebufferAttachmentParameteriv_params_compsize(_pname: u32) -> usize {
    1
}

#[allow(non_snake_case)]
pub(crate) fn glGetIntegerv_data_compsize(pname: u32) -> usize {
    shader_param_count(pname)
}

#[allow(non_snake_case)]
pub(crate) fn glGetRenderbufferParameteriv_params_compsize(_pname: u32) -> usize {
    1
}

fn tex_param_count(pname: u32) -> usize {
    // https://registry.khronos.org/OpenGL-Refpages/es3/html/glGetTexParameter.xhtml
    match pname {
        ffi::GL_TEXTURE_BORDER_COLOR => 4,
        _ => 1,
    }
}

#[allow(non_snake_case)]
pub(crate) fn glGetTexParameterfv_params_compsize(pname: u32) -> usize {
    tex_param_count(pname)
}

#[allow(non_snake_case)]
pub(crate) fn glGetTexParameteriv_params_compsize(pname: u32) -> usize {
    tex_param_count(pname)
}
#[allow(non_snake_case)]
pub(crate) fn glGetUniformfv_params_compsize(_program: u32, _location: i32) -> usize {
    // TODO somehow check, cz it is unsafe
    4
}

#[allow(non_snake_case)]
pub(crate) fn glGetUniformiv_params_compsize(_program: u32, _location: i32) -> usize {
    // TODO somehow check, cz it is unsafe
    4
}

#[allow(non_snake_case)]
pub(crate) fn glReadPixels_pixels_compsize(
    _format: u32,
    _type: u32,
    width: i32,
    height: i32,
) -> usize {
    return (width * height) as usize
        * pixel_type_width(_type);
}

#[allow(non_snake_case)]
pub(crate) fn glTexImage2D_pixels_compsize(
    _format: u32,
    _type: u32,
    width: i32,
    height: i32,
) -> usize {
    return (width * height) as usize
        * pixel_type_width(_type);
}

fn pixel_type_width(_type: u32) -> usize {
    match _type {
        ffi::GL_UNSIGNED_BYTE => 1,
        ffi::GL_BYTE => 1,
        ffi::GL_UNSIGNED_SHORT => 2,
        ffi::GL_SHORT => 2,
        ffi::GL_UNSIGNED_INT => 4,
        ffi::GL_INT => 4,
        ffi::GL_HALF_FLOAT => 2,
        ffi::GL_FLOAT => 4,
        ffi::GL_UNSIGNED_BYTE_3_3_2 => 1,
        ffi::GL_UNSIGNED_BYTE_2_3_3_REV => 1,
        ffi::GL_UNSIGNED_SHORT_5_6_5 => 2,
        ffi::GL_UNSIGNED_SHORT_5_6_5_REV => 2,
        ffi::GL_UNSIGNED_SHORT_4_4_4_4 => 2,
        ffi::GL_UNSIGNED_SHORT_4_4_4_4_REV => 2,
        ffi::GL_UNSIGNED_SHORT_5_5_5_1 => 2,
        ffi::GL_UNSIGNED_SHORT_1_5_5_5_REV => 2,
        ffi::GL_UNSIGNED_INT_8_8_8_8 => 4,
        ffi::GL_UNSIGNED_INT_8_8_8_8_REV => 4,
        ffi::GL_UNSIGNED_INT_10_10_10_2 => 4,
        ffi::GL_UNSIGNED_INT_2_10_10_10_REV => 4,
        _ => panic!(),
    }
}

#[allow(non_snake_case)]
pub(crate) fn glTexParameterfv_params_compsize(pname: u32) -> usize {
    tex_param_count(pname)
}
#[allow(non_snake_case)]
pub(crate) fn glTexParameteriv_params_compsize(pname: u32) -> usize {
    tex_param_count(pname)
}

#[allow(non_snake_case)]
pub(crate) fn glTexSubImage2D_pixels_compsize(
    _format: u32,
    _type: u32,
    width: i32,
    height: i32,
) -> usize {
    return (width * height) as usize
        * pixel_type_width(_type);
}

#[allow(non_snake_case)]
pub(crate) fn glDrawElements_indices_compsize(
    count: i32,
    _type: u32,
) -> usize {
    return count as usize
        * pixel_type_width(_type);
}


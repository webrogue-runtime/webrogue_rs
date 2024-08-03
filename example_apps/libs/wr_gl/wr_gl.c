
#include <GLES2/gl2.h>

// clang-format off

__attribute__((import_name("present")))
__attribute__((import_module("wr_gl")))
void imported_wr_gl_present();

void wr_gl_present() { imported_wr_gl_present(); }

__attribute__((import_name("glActiveTexture")))
__attribute__((import_module("wr_gl")))
void imported_glActiveTexture(unsigned int texture);

void glActiveTexture(unsigned int texture) {
    imported_glActiveTexture(texture);
}
        
__attribute__((import_name("glAttachShader")))
__attribute__((import_module("wr_gl")))
void imported_glAttachShader(unsigned int program, unsigned int shader);

void glAttachShader(unsigned int program, unsigned int shader) {
    imported_glAttachShader(program, shader);
}
        
__attribute__((import_name("glBindAttribLocation")))
__attribute__((import_module("wr_gl")))
void imported_glBindAttribLocation(unsigned int program, unsigned int index, char const* name);

void glBindAttribLocation(unsigned int program, unsigned int index, char const* name) {
    imported_glBindAttribLocation(program, index, name);
}
        
__attribute__((import_name("glBindBuffer")))
__attribute__((import_module("wr_gl")))
void imported_glBindBuffer(unsigned int target, unsigned int buffer);

void glBindBuffer(unsigned int target, unsigned int buffer) {
    imported_glBindBuffer(target, buffer);
}
        
__attribute__((import_name("glBindFramebuffer")))
__attribute__((import_module("wr_gl")))
void imported_glBindFramebuffer(unsigned int target, unsigned int framebuffer);

void glBindFramebuffer(unsigned int target, unsigned int framebuffer) {
    imported_glBindFramebuffer(target, framebuffer);
}
        
__attribute__((import_name("glBindRenderbuffer")))
__attribute__((import_module("wr_gl")))
void imported_glBindRenderbuffer(unsigned int target, unsigned int renderbuffer);

void glBindRenderbuffer(unsigned int target, unsigned int renderbuffer) {
    imported_glBindRenderbuffer(target, renderbuffer);
}
        
__attribute__((import_name("glBindTexture")))
__attribute__((import_module("wr_gl")))
void imported_glBindTexture(unsigned int target, unsigned int texture);

void glBindTexture(unsigned int target, unsigned int texture) {
    imported_glBindTexture(target, texture);
}
        
__attribute__((import_name("glBlendColor")))
__attribute__((import_module("wr_gl")))
void imported_glBlendColor(float red, float green, float blue, float alpha);

void glBlendColor(float red, float green, float blue, float alpha) {
    imported_glBlendColor(red, green, blue, alpha);
}
        
__attribute__((import_name("glBlendEquation")))
__attribute__((import_module("wr_gl")))
void imported_glBlendEquation(unsigned int mode);

void glBlendEquation(unsigned int mode) {
    imported_glBlendEquation(mode);
}
        
__attribute__((import_name("glBlendEquationSeparate")))
__attribute__((import_module("wr_gl")))
void imported_glBlendEquationSeparate(unsigned int modeRGB, unsigned int modeAlpha);

void glBlendEquationSeparate(unsigned int modeRGB, unsigned int modeAlpha) {
    imported_glBlendEquationSeparate(modeRGB, modeAlpha);
}
        
__attribute__((import_name("glBlendFunc")))
__attribute__((import_module("wr_gl")))
void imported_glBlendFunc(unsigned int sfactor, unsigned int dfactor);

void glBlendFunc(unsigned int sfactor, unsigned int dfactor) {
    imported_glBlendFunc(sfactor, dfactor);
}
        
__attribute__((import_name("glBlendFuncSeparate")))
__attribute__((import_module("wr_gl")))
void imported_glBlendFuncSeparate(unsigned int sfactorRGB, unsigned int dfactorRGB, unsigned int sfactorAlpha, unsigned int dfactorAlpha);

void glBlendFuncSeparate(unsigned int sfactorRGB, unsigned int dfactorRGB, unsigned int sfactorAlpha, unsigned int dfactorAlpha) {
    imported_glBlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
}
        
__attribute__((import_name("glBufferData")))
__attribute__((import_module("wr_gl")))
void imported_glBufferData(unsigned int target, signed long int size, void const* data, unsigned int usage);

void glBufferData(unsigned int target, signed long int size, void const* data, unsigned int usage) {
    imported_glBufferData(target, size, data, usage);
}
        
__attribute__((import_name("glBufferSubData")))
__attribute__((import_module("wr_gl")))
void imported_glBufferSubData(unsigned int target, signed long int offset, signed long int size, void const* data);

void glBufferSubData(unsigned int target, signed long int offset, signed long int size, void const* data) {
    imported_glBufferSubData(target, offset, size, data);
}
        
__attribute__((import_name("glCheckFramebufferStatus")))
__attribute__((import_module("wr_gl")))
unsigned int imported_glCheckFramebufferStatus(unsigned int target);

unsigned int glCheckFramebufferStatus(unsigned int target) {
    return imported_glCheckFramebufferStatus(target);
}
        
__attribute__((import_name("glClear")))
__attribute__((import_module("wr_gl")))
void imported_glClear(unsigned int mask);

void glClear(unsigned int mask) {
    imported_glClear(mask);
}
        
__attribute__((import_name("glClearColor")))
__attribute__((import_module("wr_gl")))
void imported_glClearColor(float red, float green, float blue, float alpha);

void glClearColor(float red, float green, float blue, float alpha) {
    imported_glClearColor(red, green, blue, alpha);
}
        
__attribute__((import_name("glClearDepthf")))
__attribute__((import_module("wr_gl")))
void imported_glClearDepthf(float d);

void glClearDepthf(float d) {
    imported_glClearDepthf(d);
}
        
__attribute__((import_name("glClearStencil")))
__attribute__((import_module("wr_gl")))
void imported_glClearStencil(int s);

void glClearStencil(int s) {
    imported_glClearStencil(s);
}
        
__attribute__((import_name("glColorMask")))
__attribute__((import_module("wr_gl")))
void imported_glColorMask(unsigned char red, unsigned char green, unsigned char blue, unsigned char alpha);

void glColorMask(unsigned char red, unsigned char green, unsigned char blue, unsigned char alpha) {
    imported_glColorMask(red, green, blue, alpha);
}
        
__attribute__((import_name("glCompileShader")))
__attribute__((import_module("wr_gl")))
void imported_glCompileShader(unsigned int shader);

void glCompileShader(unsigned int shader) {
    imported_glCompileShader(shader);
}
        
__attribute__((import_name("glCompressedTexImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glCompressedTexImage2D(unsigned int target, int level, unsigned int internalformat, int width, int height, int border, int imageSize, void const* data);

void glCompressedTexImage2D(unsigned int target, int level, unsigned int internalformat, int width, int height, int border, int imageSize, void const* data) {
    imported_glCompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data);
}
        
__attribute__((import_name("glCompressedTexSubImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glCompressedTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int width, int height, unsigned int format, int imageSize, void const* data);

void glCompressedTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int width, int height, unsigned int format, int imageSize, void const* data) {
    imported_glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data);
}
        
__attribute__((import_name("glCopyTexImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glCopyTexImage2D(unsigned int target, int level, unsigned int internalformat, int x, int y, int width, int height, int border);

void glCopyTexImage2D(unsigned int target, int level, unsigned int internalformat, int x, int y, int width, int height, int border) {
    imported_glCopyTexImage2D(target, level, internalformat, x, y, width, height, border);
}
        
__attribute__((import_name("glCopyTexSubImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glCopyTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int x, int y, int width, int height);

void glCopyTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int x, int y, int width, int height) {
    imported_glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
}
        
__attribute__((import_name("glCreateProgram")))
__attribute__((import_module("wr_gl")))
unsigned int imported_glCreateProgram();

unsigned int glCreateProgram() {
    return imported_glCreateProgram();
}
        
__attribute__((import_name("glCreateShader")))
__attribute__((import_module("wr_gl")))
unsigned int imported_glCreateShader(unsigned int type);

unsigned int glCreateShader(unsigned int type) {
    return imported_glCreateShader(type);
}
        
__attribute__((import_name("glCullFace")))
__attribute__((import_module("wr_gl")))
void imported_glCullFace(unsigned int mode);

void glCullFace(unsigned int mode) {
    imported_glCullFace(mode);
}
        
__attribute__((import_name("glDeleteBuffers")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteBuffers(int n, unsigned int const* buffers);

void glDeleteBuffers(int n, unsigned int const* buffers) {
    imported_glDeleteBuffers(n, buffers);
}
        
__attribute__((import_name("glDeleteFramebuffers")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteFramebuffers(int n, unsigned int const* framebuffers);

void glDeleteFramebuffers(int n, unsigned int const* framebuffers) {
    imported_glDeleteFramebuffers(n, framebuffers);
}
        
__attribute__((import_name("glDeleteProgram")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteProgram(unsigned int program);

void glDeleteProgram(unsigned int program) {
    imported_glDeleteProgram(program);
}
        
__attribute__((import_name("glDeleteRenderbuffers")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteRenderbuffers(int n, unsigned int const* renderbuffers);

void glDeleteRenderbuffers(int n, unsigned int const* renderbuffers) {
    imported_glDeleteRenderbuffers(n, renderbuffers);
}
        
__attribute__((import_name("glDeleteShader")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteShader(unsigned int shader);

void glDeleteShader(unsigned int shader) {
    imported_glDeleteShader(shader);
}
        
__attribute__((import_name("glDeleteTextures")))
__attribute__((import_module("wr_gl")))
void imported_glDeleteTextures(int n, unsigned int const* textures);

void glDeleteTextures(int n, unsigned int const* textures) {
    imported_glDeleteTextures(n, textures);
}
        
__attribute__((import_name("glDepthFunc")))
__attribute__((import_module("wr_gl")))
void imported_glDepthFunc(unsigned int func);

void glDepthFunc(unsigned int func) {
    imported_glDepthFunc(func);
}
        
__attribute__((import_name("glDepthMask")))
__attribute__((import_module("wr_gl")))
void imported_glDepthMask(unsigned char flag);

void glDepthMask(unsigned char flag) {
    imported_glDepthMask(flag);
}
        
__attribute__((import_name("glDepthRangef")))
__attribute__((import_module("wr_gl")))
void imported_glDepthRangef(float n, float f);

void glDepthRangef(float n, float f) {
    imported_glDepthRangef(n, f);
}
        
__attribute__((import_name("glDetachShader")))
__attribute__((import_module("wr_gl")))
void imported_glDetachShader(unsigned int program, unsigned int shader);

void glDetachShader(unsigned int program, unsigned int shader) {
    imported_glDetachShader(program, shader);
}
        
__attribute__((import_name("glDisable")))
__attribute__((import_module("wr_gl")))
void imported_glDisable(unsigned int cap);

void glDisable(unsigned int cap) {
    imported_glDisable(cap);
}
        
__attribute__((import_name("glDisableVertexAttribArray")))
__attribute__((import_module("wr_gl")))
void imported_glDisableVertexAttribArray(unsigned int index);

void glDisableVertexAttribArray(unsigned int index) {
    imported_glDisableVertexAttribArray(index);
}
        
__attribute__((import_name("glDrawArrays")))
__attribute__((import_module("wr_gl")))
void imported_glDrawArrays(unsigned int mode, int first, int count);

void glDrawArrays(unsigned int mode, int first, int count) {
    imported_glDrawArrays(mode, first, count);
}
        
__attribute__((import_name("glDrawElements")))
__attribute__((import_module("wr_gl")))
void imported_glDrawElements(unsigned int mode, int count, unsigned int type, void const* indices);

void glDrawElements(unsigned int mode, int count, unsigned int type, void const* indices) {
    imported_glDrawElements(mode, count, type, indices);
}
        
__attribute__((import_name("glEnable")))
__attribute__((import_module("wr_gl")))
void imported_glEnable(unsigned int cap);

void glEnable(unsigned int cap) {
    imported_glEnable(cap);
}
        
__attribute__((import_name("glEnableVertexAttribArray")))
__attribute__((import_module("wr_gl")))
void imported_glEnableVertexAttribArray(unsigned int index);

void glEnableVertexAttribArray(unsigned int index) {
    imported_glEnableVertexAttribArray(index);
}
        
__attribute__((import_name("glFinish")))
__attribute__((import_module("wr_gl")))
void imported_glFinish();

void glFinish() {
    imported_glFinish();
}
        
__attribute__((import_name("glFlush")))
__attribute__((import_module("wr_gl")))
void imported_glFlush();

void glFlush() {
    imported_glFlush();
}
        
__attribute__((import_name("glFramebufferRenderbuffer")))
__attribute__((import_module("wr_gl")))
void imported_glFramebufferRenderbuffer(unsigned int target, unsigned int attachment, unsigned int renderbuffertarget, unsigned int renderbuffer);

void glFramebufferRenderbuffer(unsigned int target, unsigned int attachment, unsigned int renderbuffertarget, unsigned int renderbuffer) {
    imported_glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
}
        
__attribute__((import_name("glFramebufferTexture2D")))
__attribute__((import_module("wr_gl")))
void imported_glFramebufferTexture2D(unsigned int target, unsigned int attachment, unsigned int textarget, unsigned int texture, int level);

void glFramebufferTexture2D(unsigned int target, unsigned int attachment, unsigned int textarget, unsigned int texture, int level) {
    imported_glFramebufferTexture2D(target, attachment, textarget, texture, level);
}
        
__attribute__((import_name("glFrontFace")))
__attribute__((import_module("wr_gl")))
void imported_glFrontFace(unsigned int mode);

void glFrontFace(unsigned int mode) {
    imported_glFrontFace(mode);
}
        
__attribute__((import_name("glGenBuffers")))
__attribute__((import_module("wr_gl")))
void imported_glGenBuffers(int n, unsigned int * buffers);

void glGenBuffers(int n, unsigned int * buffers) {
    imported_glGenBuffers(n, buffers);
}
        
__attribute__((import_name("glGenFramebuffers")))
__attribute__((import_module("wr_gl")))
void imported_glGenFramebuffers(int n, unsigned int * framebuffers);

void glGenFramebuffers(int n, unsigned int * framebuffers) {
    imported_glGenFramebuffers(n, framebuffers);
}
        
__attribute__((import_name("glGenRenderbuffers")))
__attribute__((import_module("wr_gl")))
void imported_glGenRenderbuffers(int n, unsigned int * renderbuffers);

void glGenRenderbuffers(int n, unsigned int * renderbuffers) {
    imported_glGenRenderbuffers(n, renderbuffers);
}
        
__attribute__((import_name("glGenTextures")))
__attribute__((import_module("wr_gl")))
void imported_glGenTextures(int n, unsigned int * textures);

void glGenTextures(int n, unsigned int * textures) {
    imported_glGenTextures(n, textures);
}
        
__attribute__((import_name("glGenerateMipmap")))
__attribute__((import_module("wr_gl")))
void imported_glGenerateMipmap(unsigned int target);

void glGenerateMipmap(unsigned int target) {
    imported_glGenerateMipmap(target);
}
        
__attribute__((import_name("glGetActiveAttrib")))
__attribute__((import_module("wr_gl")))
void imported_glGetActiveAttrib(unsigned int program, unsigned int index, int bufSize, int * length, int * size, unsigned int * type, char * name);

void glGetActiveAttrib(unsigned int program, unsigned int index, int bufSize, int * length, int * size, unsigned int * type, char * name) {
    imported_glGetActiveAttrib(program, index, bufSize, length, size, type, name);
}
        
__attribute__((import_name("glGetActiveUniform")))
__attribute__((import_module("wr_gl")))
void imported_glGetActiveUniform(unsigned int program, unsigned int index, int bufSize, int * length, int * size, unsigned int * type, char * name);

void glGetActiveUniform(unsigned int program, unsigned int index, int bufSize, int * length, int * size, unsigned int * type, char * name) {
    imported_glGetActiveUniform(program, index, bufSize, length, size, type, name);
}
        
__attribute__((import_name("glGetAttachedShaders")))
__attribute__((import_module("wr_gl")))
void imported_glGetAttachedShaders(unsigned int program, int maxCount, int * count, unsigned int * shaders);

void glGetAttachedShaders(unsigned int program, int maxCount, int * count, unsigned int * shaders) {
    imported_glGetAttachedShaders(program, maxCount, count, shaders);
}
        
__attribute__((import_name("glGetAttribLocation")))
__attribute__((import_module("wr_gl")))
int imported_glGetAttribLocation(unsigned int program, char const* name);

int glGetAttribLocation(unsigned int program, char const* name) {
    return imported_glGetAttribLocation(program, name);
}
        
__attribute__((import_name("glGetBooleanv")))
__attribute__((import_module("wr_gl")))
void imported_glGetBooleanv(unsigned int pname, unsigned char * data);

void glGetBooleanv(unsigned int pname, unsigned char * data) {
    imported_glGetBooleanv(pname, data);
}
        
__attribute__((import_name("glGetBufferParameteriv")))
__attribute__((import_module("wr_gl")))
void imported_glGetBufferParameteriv(unsigned int target, unsigned int pname, int * params);

void glGetBufferParameteriv(unsigned int target, unsigned int pname, int * params) {
    imported_glGetBufferParameteriv(target, pname, params);
}
        
__attribute__((import_name("glGetError")))
__attribute__((import_module("wr_gl")))
unsigned int imported_glGetError();

unsigned int glGetError() {
    return imported_glGetError();
}
        
__attribute__((import_name("glGetFloatv")))
__attribute__((import_module("wr_gl")))
void imported_glGetFloatv(unsigned int pname, float * data);

void glGetFloatv(unsigned int pname, float * data) {
    imported_glGetFloatv(pname, data);
}
        
__attribute__((import_name("glGetFramebufferAttachmentParameteriv")))
__attribute__((import_module("wr_gl")))
void imported_glGetFramebufferAttachmentParameteriv(unsigned int target, unsigned int attachment, unsigned int pname, int * params);

void glGetFramebufferAttachmentParameteriv(unsigned int target, unsigned int attachment, unsigned int pname, int * params) {
    imported_glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);
}
        
__attribute__((import_name("glGetIntegerv")))
__attribute__((import_module("wr_gl")))
void imported_glGetIntegerv(unsigned int pname, int * data);

void glGetIntegerv(unsigned int pname, int * data) {
    imported_glGetIntegerv(pname, data);
}
        
__attribute__((import_name("glGetProgramInfoLog")))
__attribute__((import_module("wr_gl")))
void imported_glGetProgramInfoLog(unsigned int program, int bufSize, int * length, char * infoLog);

void glGetProgramInfoLog(unsigned int program, int bufSize, int * length, char * infoLog) {
    imported_glGetProgramInfoLog(program, bufSize, length, infoLog);
}
        
__attribute__((import_name("glGetProgramiv")))
__attribute__((import_module("wr_gl")))
void imported_glGetProgramiv(unsigned int program, unsigned int pname, int * params);

void glGetProgramiv(unsigned int program, unsigned int pname, int * params) {
    imported_glGetProgramiv(program, pname, params);
}
        
__attribute__((import_name("glGetRenderbufferParameteriv")))
__attribute__((import_module("wr_gl")))
void imported_glGetRenderbufferParameteriv(unsigned int target, unsigned int pname, int * params);

void glGetRenderbufferParameteriv(unsigned int target, unsigned int pname, int * params) {
    imported_glGetRenderbufferParameteriv(target, pname, params);
}
        
__attribute__((import_name("glGetShaderInfoLog")))
__attribute__((import_module("wr_gl")))
void imported_glGetShaderInfoLog(unsigned int shader, int bufSize, int * length, char * infoLog);

void glGetShaderInfoLog(unsigned int shader, int bufSize, int * length, char * infoLog) {
    imported_glGetShaderInfoLog(shader, bufSize, length, infoLog);
}
        
__attribute__((import_name("glGetShaderPrecisionFormat")))
__attribute__((import_module("wr_gl")))
void imported_glGetShaderPrecisionFormat(unsigned int shadertype, unsigned int precisiontype, int * range, int * precision);

void glGetShaderPrecisionFormat(unsigned int shadertype, unsigned int precisiontype, int * range, int * precision) {
    imported_glGetShaderPrecisionFormat(shadertype, precisiontype, range, precision);
}
        
__attribute__((import_name("glGetShaderSource")))
__attribute__((import_module("wr_gl")))
void imported_glGetShaderSource(unsigned int shader, int bufSize, int * length, char * source);

void glGetShaderSource(unsigned int shader, int bufSize, int * length, char * source) {
    imported_glGetShaderSource(shader, bufSize, length, source);
}
        
__attribute__((import_name("glGetShaderiv")))
__attribute__((import_module("wr_gl")))
void imported_glGetShaderiv(unsigned int shader, unsigned int pname, int * params);

void glGetShaderiv(unsigned int shader, unsigned int pname, int * params) {
    imported_glGetShaderiv(shader, pname, params);
}
        
__attribute__((import_name("glGetString")))
__attribute__((import_module("wr_gl")))
unsigned char const* imported_glGetString(unsigned int name);

unsigned char const* glGetString(unsigned int name) {
    return imported_glGetString(name);
}
        
__attribute__((import_name("glGetTexParameterfv")))
__attribute__((import_module("wr_gl")))
void imported_glGetTexParameterfv(unsigned int target, unsigned int pname, float * params);

void glGetTexParameterfv(unsigned int target, unsigned int pname, float * params) {
    imported_glGetTexParameterfv(target, pname, params);
}
        
__attribute__((import_name("glGetTexParameteriv")))
__attribute__((import_module("wr_gl")))
void imported_glGetTexParameteriv(unsigned int target, unsigned int pname, int * params);

void glGetTexParameteriv(unsigned int target, unsigned int pname, int * params) {
    imported_glGetTexParameteriv(target, pname, params);
}
        
__attribute__((import_name("glGetUniformLocation")))
__attribute__((import_module("wr_gl")))
int imported_glGetUniformLocation(unsigned int program, char const* name);

int glGetUniformLocation(unsigned int program, char const* name) {
    return imported_glGetUniformLocation(program, name);
}
        
__attribute__((import_name("glGetUniformfv")))
__attribute__((import_module("wr_gl")))
void imported_glGetUniformfv(unsigned int program, int location, float * params);

void glGetUniformfv(unsigned int program, int location, float * params) {
    imported_glGetUniformfv(program, location, params);
}
        
__attribute__((import_name("glGetUniformiv")))
__attribute__((import_module("wr_gl")))
void imported_glGetUniformiv(unsigned int program, int location, int * params);

void glGetUniformiv(unsigned int program, int location, int * params) {
    imported_glGetUniformiv(program, location, params);
}
        
__attribute__((import_name("glGetVertexAttribPointerv")))
__attribute__((import_module("wr_gl")))
void imported_glGetVertexAttribPointerv(unsigned int index, unsigned int pname, void * * pointer);

void glGetVertexAttribPointerv(unsigned int index, unsigned int pname, void * * pointer) {
    imported_glGetVertexAttribPointerv(index, pname, pointer);
}
        
__attribute__((import_name("glGetVertexAttribfv")))
__attribute__((import_module("wr_gl")))
void imported_glGetVertexAttribfv(unsigned int index, unsigned int pname, float * params);

void glGetVertexAttribfv(unsigned int index, unsigned int pname, float * params) {
    imported_glGetVertexAttribfv(index, pname, params);
}
        
__attribute__((import_name("glGetVertexAttribiv")))
__attribute__((import_module("wr_gl")))
void imported_glGetVertexAttribiv(unsigned int index, unsigned int pname, int * params);

void glGetVertexAttribiv(unsigned int index, unsigned int pname, int * params) {
    imported_glGetVertexAttribiv(index, pname, params);
}
        
__attribute__((import_name("glHint")))
__attribute__((import_module("wr_gl")))
void imported_glHint(unsigned int target, unsigned int mode);

void glHint(unsigned int target, unsigned int mode) {
    imported_glHint(target, mode);
}
        
__attribute__((import_name("glIsBuffer")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsBuffer(unsigned int buffer);

unsigned char glIsBuffer(unsigned int buffer) {
    return imported_glIsBuffer(buffer);
}
        
__attribute__((import_name("glIsEnabled")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsEnabled(unsigned int cap);

unsigned char glIsEnabled(unsigned int cap) {
    return imported_glIsEnabled(cap);
}
        
__attribute__((import_name("glIsFramebuffer")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsFramebuffer(unsigned int framebuffer);

unsigned char glIsFramebuffer(unsigned int framebuffer) {
    return imported_glIsFramebuffer(framebuffer);
}
        
__attribute__((import_name("glIsProgram")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsProgram(unsigned int program);

unsigned char glIsProgram(unsigned int program) {
    return imported_glIsProgram(program);
}
        
__attribute__((import_name("glIsRenderbuffer")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsRenderbuffer(unsigned int renderbuffer);

unsigned char glIsRenderbuffer(unsigned int renderbuffer) {
    return imported_glIsRenderbuffer(renderbuffer);
}
        
__attribute__((import_name("glIsShader")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsShader(unsigned int shader);

unsigned char glIsShader(unsigned int shader) {
    return imported_glIsShader(shader);
}
        
__attribute__((import_name("glIsTexture")))
__attribute__((import_module("wr_gl")))
unsigned char imported_glIsTexture(unsigned int texture);

unsigned char glIsTexture(unsigned int texture) {
    return imported_glIsTexture(texture);
}
        
__attribute__((import_name("glLineWidth")))
__attribute__((import_module("wr_gl")))
void imported_glLineWidth(float width);

void glLineWidth(float width) {
    imported_glLineWidth(width);
}
        
__attribute__((import_name("glLinkProgram")))
__attribute__((import_module("wr_gl")))
void imported_glLinkProgram(unsigned int program);

void glLinkProgram(unsigned int program) {
    imported_glLinkProgram(program);
}
        
__attribute__((import_name("glPixelStorei")))
__attribute__((import_module("wr_gl")))
void imported_glPixelStorei(unsigned int pname, int param);

void glPixelStorei(unsigned int pname, int param) {
    imported_glPixelStorei(pname, param);
}
        
__attribute__((import_name("glPolygonOffset")))
__attribute__((import_module("wr_gl")))
void imported_glPolygonOffset(float factor, float units);

void glPolygonOffset(float factor, float units) {
    imported_glPolygonOffset(factor, units);
}
        
__attribute__((import_name("glReadPixels")))
__attribute__((import_module("wr_gl")))
void imported_glReadPixels(int x, int y, int width, int height, unsigned int format, unsigned int type, void * pixels);

void glReadPixels(int x, int y, int width, int height, unsigned int format, unsigned int type, void * pixels) {
    imported_glReadPixels(x, y, width, height, format, type, pixels);
}
        
__attribute__((import_name("glReleaseShaderCompiler")))
__attribute__((import_module("wr_gl")))
void imported_glReleaseShaderCompiler();

void glReleaseShaderCompiler() {
    imported_glReleaseShaderCompiler();
}
        
__attribute__((import_name("glRenderbufferStorage")))
__attribute__((import_module("wr_gl")))
void imported_glRenderbufferStorage(unsigned int target, unsigned int internalformat, int width, int height);

void glRenderbufferStorage(unsigned int target, unsigned int internalformat, int width, int height) {
    imported_glRenderbufferStorage(target, internalformat, width, height);
}
        
__attribute__((import_name("glSampleCoverage")))
__attribute__((import_module("wr_gl")))
void imported_glSampleCoverage(float value, unsigned char invert);

void glSampleCoverage(float value, unsigned char invert) {
    imported_glSampleCoverage(value, invert);
}
        
__attribute__((import_name("glScissor")))
__attribute__((import_module("wr_gl")))
void imported_glScissor(int x, int y, int width, int height);

void glScissor(int x, int y, int width, int height) {
    imported_glScissor(x, y, width, height);
}
        
__attribute__((import_name("glShaderBinary")))
__attribute__((import_module("wr_gl")))
void imported_glShaderBinary(int count, unsigned int const* shaders, unsigned int binaryFormat, void const* binary, int length);

void glShaderBinary(int count, unsigned int const* shaders, unsigned int binaryFormat, void const* binary, int length) {
    imported_glShaderBinary(count, shaders, binaryFormat, binary, length);
}
        
__attribute__((import_name("glShaderSource")))
__attribute__((import_module("wr_gl")))
void imported_glShaderSource(unsigned int shader, int count, char const* const* string, int const* length);

void glShaderSource(unsigned int shader, int count, char const* const* string, int const* length) {
    imported_glShaderSource(shader, count, string, length);
}
        
__attribute__((import_name("glStencilFunc")))
__attribute__((import_module("wr_gl")))
void imported_glStencilFunc(unsigned int func, int ref, unsigned int mask);

void glStencilFunc(unsigned int func, int ref, unsigned int mask) {
    imported_glStencilFunc(func, ref, mask);
}
        
__attribute__((import_name("glStencilFuncSeparate")))
__attribute__((import_module("wr_gl")))
void imported_glStencilFuncSeparate(unsigned int face, unsigned int func, int ref, unsigned int mask);

void glStencilFuncSeparate(unsigned int face, unsigned int func, int ref, unsigned int mask) {
    imported_glStencilFuncSeparate(face, func, ref, mask);
}
        
__attribute__((import_name("glStencilMask")))
__attribute__((import_module("wr_gl")))
void imported_glStencilMask(unsigned int mask);

void glStencilMask(unsigned int mask) {
    imported_glStencilMask(mask);
}
        
__attribute__((import_name("glStencilMaskSeparate")))
__attribute__((import_module("wr_gl")))
void imported_glStencilMaskSeparate(unsigned int face, unsigned int mask);

void glStencilMaskSeparate(unsigned int face, unsigned int mask) {
    imported_glStencilMaskSeparate(face, mask);
}
        
__attribute__((import_name("glStencilOp")))
__attribute__((import_module("wr_gl")))
void imported_glStencilOp(unsigned int fail, unsigned int zfail, unsigned int zpass);

void glStencilOp(unsigned int fail, unsigned int zfail, unsigned int zpass) {
    imported_glStencilOp(fail, zfail, zpass);
}
        
__attribute__((import_name("glStencilOpSeparate")))
__attribute__((import_module("wr_gl")))
void imported_glStencilOpSeparate(unsigned int face, unsigned int sfail, unsigned int dpfail, unsigned int dppass);

void glStencilOpSeparate(unsigned int face, unsigned int sfail, unsigned int dpfail, unsigned int dppass) {
    imported_glStencilOpSeparate(face, sfail, dpfail, dppass);
}
        
__attribute__((import_name("glTexImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glTexImage2D(unsigned int target, int level, int internalformat, int width, int height, int border, unsigned int format, unsigned int type, void const* pixels);

void glTexImage2D(unsigned int target, int level, int internalformat, int width, int height, int border, unsigned int format, unsigned int type, void const* pixels) {
    imported_glTexImage2D(target, level, internalformat, width, height, border, format, type, pixels);
}
        
__attribute__((import_name("glTexParameterf")))
__attribute__((import_module("wr_gl")))
void imported_glTexParameterf(unsigned int target, unsigned int pname, float param);

void glTexParameterf(unsigned int target, unsigned int pname, float param) {
    imported_glTexParameterf(target, pname, param);
}
        
__attribute__((import_name("glTexParameterfv")))
__attribute__((import_module("wr_gl")))
void imported_glTexParameterfv(unsigned int target, unsigned int pname, float const* params);

void glTexParameterfv(unsigned int target, unsigned int pname, float const* params) {
    imported_glTexParameterfv(target, pname, params);
}
        
__attribute__((import_name("glTexParameteri")))
__attribute__((import_module("wr_gl")))
void imported_glTexParameteri(unsigned int target, unsigned int pname, int param);

void glTexParameteri(unsigned int target, unsigned int pname, int param) {
    imported_glTexParameteri(target, pname, param);
}
        
__attribute__((import_name("glTexParameteriv")))
__attribute__((import_module("wr_gl")))
void imported_glTexParameteriv(unsigned int target, unsigned int pname, int const* params);

void glTexParameteriv(unsigned int target, unsigned int pname, int const* params) {
    imported_glTexParameteriv(target, pname, params);
}
        
__attribute__((import_name("glTexSubImage2D")))
__attribute__((import_module("wr_gl")))
void imported_glTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int width, int height, unsigned int format, unsigned int type, void const* pixels);

void glTexSubImage2D(unsigned int target, int level, int xoffset, int yoffset, int width, int height, unsigned int format, unsigned int type, void const* pixels) {
    imported_glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
}
        
__attribute__((import_name("glUniform1f")))
__attribute__((import_module("wr_gl")))
void imported_glUniform1f(int location, float v0);

void glUniform1f(int location, float v0) {
    imported_glUniform1f(location, v0);
}
        
__attribute__((import_name("glUniform1fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform1fv(int location, int count, float const* value);

void glUniform1fv(int location, int count, float const* value) {
    imported_glUniform1fv(location, count, value);
}
        
__attribute__((import_name("glUniform1i")))
__attribute__((import_module("wr_gl")))
void imported_glUniform1i(int location, int v0);

void glUniform1i(int location, int v0) {
    imported_glUniform1i(location, v0);
}
        
__attribute__((import_name("glUniform1iv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform1iv(int location, int count, int const* value);

void glUniform1iv(int location, int count, int const* value) {
    imported_glUniform1iv(location, count, value);
}
        
__attribute__((import_name("glUniform2f")))
__attribute__((import_module("wr_gl")))
void imported_glUniform2f(int location, float v0, float v1);

void glUniform2f(int location, float v0, float v1) {
    imported_glUniform2f(location, v0, v1);
}
        
__attribute__((import_name("glUniform2fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform2fv(int location, int count, float const* value);

void glUniform2fv(int location, int count, float const* value) {
    imported_glUniform2fv(location, count, value);
}
        
__attribute__((import_name("glUniform2i")))
__attribute__((import_module("wr_gl")))
void imported_glUniform2i(int location, int v0, int v1);

void glUniform2i(int location, int v0, int v1) {
    imported_glUniform2i(location, v0, v1);
}
        
__attribute__((import_name("glUniform2iv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform2iv(int location, int count, int const* value);

void glUniform2iv(int location, int count, int const* value) {
    imported_glUniform2iv(location, count, value);
}
        
__attribute__((import_name("glUniform3f")))
__attribute__((import_module("wr_gl")))
void imported_glUniform3f(int location, float v0, float v1, float v2);

void glUniform3f(int location, float v0, float v1, float v2) {
    imported_glUniform3f(location, v0, v1, v2);
}
        
__attribute__((import_name("glUniform3fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform3fv(int location, int count, float const* value);

void glUniform3fv(int location, int count, float const* value) {
    imported_glUniform3fv(location, count, value);
}
        
__attribute__((import_name("glUniform3i")))
__attribute__((import_module("wr_gl")))
void imported_glUniform3i(int location, int v0, int v1, int v2);

void glUniform3i(int location, int v0, int v1, int v2) {
    imported_glUniform3i(location, v0, v1, v2);
}
        
__attribute__((import_name("glUniform3iv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform3iv(int location, int count, int const* value);

void glUniform3iv(int location, int count, int const* value) {
    imported_glUniform3iv(location, count, value);
}
        
__attribute__((import_name("glUniform4f")))
__attribute__((import_module("wr_gl")))
void imported_glUniform4f(int location, float v0, float v1, float v2, float v3);

void glUniform4f(int location, float v0, float v1, float v2, float v3) {
    imported_glUniform4f(location, v0, v1, v2, v3);
}
        
__attribute__((import_name("glUniform4fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform4fv(int location, int count, float const* value);

void glUniform4fv(int location, int count, float const* value) {
    imported_glUniform4fv(location, count, value);
}
        
__attribute__((import_name("glUniform4i")))
__attribute__((import_module("wr_gl")))
void imported_glUniform4i(int location, int v0, int v1, int v2, int v3);

void glUniform4i(int location, int v0, int v1, int v2, int v3) {
    imported_glUniform4i(location, v0, v1, v2, v3);
}
        
__attribute__((import_name("glUniform4iv")))
__attribute__((import_module("wr_gl")))
void imported_glUniform4iv(int location, int count, int const* value);

void glUniform4iv(int location, int count, int const* value) {
    imported_glUniform4iv(location, count, value);
}
        
__attribute__((import_name("glUniformMatrix2fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniformMatrix2fv(int location, int count, unsigned char transpose, float const* value);

void glUniformMatrix2fv(int location, int count, unsigned char transpose, float const* value) {
    imported_glUniformMatrix2fv(location, count, transpose, value);
}
        
__attribute__((import_name("glUniformMatrix3fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniformMatrix3fv(int location, int count, unsigned char transpose, float const* value);

void glUniformMatrix3fv(int location, int count, unsigned char transpose, float const* value) {
    imported_glUniformMatrix3fv(location, count, transpose, value);
}
        
__attribute__((import_name("glUniformMatrix4fv")))
__attribute__((import_module("wr_gl")))
void imported_glUniformMatrix4fv(int location, int count, unsigned char transpose, float const* value);

void glUniformMatrix4fv(int location, int count, unsigned char transpose, float const* value) {
    imported_glUniformMatrix4fv(location, count, transpose, value);
}
        
__attribute__((import_name("glUseProgram")))
__attribute__((import_module("wr_gl")))
void imported_glUseProgram(unsigned int program);

void glUseProgram(unsigned int program) {
    imported_glUseProgram(program);
}
        
__attribute__((import_name("glValidateProgram")))
__attribute__((import_module("wr_gl")))
void imported_glValidateProgram(unsigned int program);

void glValidateProgram(unsigned int program) {
    imported_glValidateProgram(program);
}
        
__attribute__((import_name("glVertexAttrib1f")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib1f(unsigned int index, float x);

void glVertexAttrib1f(unsigned int index, float x) {
    imported_glVertexAttrib1f(index, x);
}
        
__attribute__((import_name("glVertexAttrib1fv")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib1fv(unsigned int index, float const* v);

void glVertexAttrib1fv(unsigned int index, float const* v) {
    imported_glVertexAttrib1fv(index, v);
}
        
__attribute__((import_name("glVertexAttrib2f")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib2f(unsigned int index, float x, float y);

void glVertexAttrib2f(unsigned int index, float x, float y) {
    imported_glVertexAttrib2f(index, x, y);
}
        
__attribute__((import_name("glVertexAttrib2fv")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib2fv(unsigned int index, float const* v);

void glVertexAttrib2fv(unsigned int index, float const* v) {
    imported_glVertexAttrib2fv(index, v);
}
        
__attribute__((import_name("glVertexAttrib3f")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib3f(unsigned int index, float x, float y, float z);

void glVertexAttrib3f(unsigned int index, float x, float y, float z) {
    imported_glVertexAttrib3f(index, x, y, z);
}
        
__attribute__((import_name("glVertexAttrib3fv")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib3fv(unsigned int index, float const* v);

void glVertexAttrib3fv(unsigned int index, float const* v) {
    imported_glVertexAttrib3fv(index, v);
}
        
__attribute__((import_name("glVertexAttrib4f")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib4f(unsigned int index, float x, float y, float z, float w);

void glVertexAttrib4f(unsigned int index, float x, float y, float z, float w) {
    imported_glVertexAttrib4f(index, x, y, z, w);
}
        
__attribute__((import_name("glVertexAttrib4fv")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttrib4fv(unsigned int index, float const* v);

void glVertexAttrib4fv(unsigned int index, float const* v) {
    imported_glVertexAttrib4fv(index, v);
}
        
__attribute__((import_name("glVertexAttribPointer")))
__attribute__((import_module("wr_gl")))
void imported_glVertexAttribPointer(unsigned int index, int size, unsigned int type, unsigned char normalized, int stride, void const* pointer);

void glVertexAttribPointer(unsigned int index, int size, unsigned int type, unsigned char normalized, int stride, void const* pointer) {
    imported_glVertexAttribPointer(index, size, type, normalized, stride, pointer);
}
        
__attribute__((import_name("glViewport")))
__attribute__((import_module("wr_gl")))
void imported_glViewport(int x, int y, int width, int height);

void glViewport(int x, int y, int width, int height) {
    imported_glViewport(x, y, width, height);
}
        
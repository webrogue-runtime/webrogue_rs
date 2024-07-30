#include <GLES2/gl2.h>

__attribute__((import_name("present")))
__attribute__((import_module("wr_gl"))) void
imported_wr_gl_present();

void wr_gl_present() { imported_wr_gl_present(); }

__attribute__((import_name("glViewport")))
__attribute__((import_module("wr_gl"))) void
imported_glViewport(GLint x, GLint y, GLsizei width, GLsizei height);

void glViewport(GLint x, GLint y, GLsizei width, GLsizei height) {
  imported_glViewport(x, y, width, height);
}

__attribute__((import_name("glClearColor")))
__attribute__((import_module("wr_gl"))) void
imported_glClearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);

void glClearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha) {
  imported_glClearColor(red, green, blue, alpha);
}

__attribute__((import_name("glClear")))
__attribute__((import_module("wr_gl"))) void
imported_glClear(GLbitfield mask);

void glClear(GLbitfield mask) { imported_glClear(mask); }

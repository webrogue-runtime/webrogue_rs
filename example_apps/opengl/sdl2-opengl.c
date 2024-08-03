#include <GLES2/gl2.h>
#include <string.h>

#ifdef __WEBROGUE__
#include <wr_gl/wr_gl.h>
#else
#include <SDL2/SDL.h>
#include <SDL2/SDL_opengl.h>
#endif

#include <assert.h>
#include <stdint.h>

typedef int32_t i32;
typedef uint32_t u32;
typedef int32_t b32;

#define WinWidth 600
#define WinHeight 300

// int main(int ArgCount, char **Args) {
//   b32 Running = 1;
// #ifdef __WEBROGUE__
// #else
//   u32 WindowFlags = SDL_WINDOW_OPENGL;
//   SDL_Window *Window =
//       SDL_CreateWindow("OpenGL Test", 0, 0, WinWidth, WinHeight,
//       WindowFlags);
//   assert(Window);
//   SDL_GLContext Context = SDL_GL_CreateContext(Window);
// #endif

//   b32 FullScreen = 0;
//   while (Running) {
//     // SDL_Event Event;
//     // while (SDL_PollEvent(&Event)) {
//     //   if (Event.type == SDL_KEYDOWN) {
//     //     switch (Event.key.keysym.sym) {
//     //     case SDLK_ESCAPE:
//     //       Running = 0;
//     //       break;
//     //     case 'f':
//     //       FullScreen = !FullScreen;
//     //       if (FullScreen) {
//     //         SDL_SetWindowFullscreen(Window, WindowFlags |
//     // SDL_WINDOW_FULLSCREEN_DESKTOP);
//     //       } else {
//     //         SDL_SetWindowFullscreen(Window, WindowFlags);
//     //       }
//     //       break;
//     //     default:
//     //       break;
//     //     }
//     //   } else if (Event.type == SDL_QUIT) {
//     //     Running = 0;
//     //   }
//     // }

//     glViewport(0, 0, WinWidth, WinHeight);
//     glClearColor(1.f, 0.f, 1.f, 0.f);
//     glClear(GL_COLOR_BUFFER_BIT);
// #ifdef __WEBROGUE__
//     wr_gl_present();
// #else
//     SDL_GL_SwapWindow(Window);
// #endif
//   }
//   return 0;
// }

#include <stdio.h>
#include <stdlib.h>

static const GLuint WIDTH = 800;
static const GLuint HEIGHT = 600;
static const GLchar *vertex_shader_source =
    "#version 100\n"
    "attribute vec3 position;\n"
    "void main() {\n"
    "   gl_Position = vec4(position, 1.0);\n"
    "}\n";
static const GLchar *fragment_shader_source =
    "#version 100\n"
    "void main() {\n"
    "   gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);\n"
    "}\n";
static const GLfloat vertices[] = {
    0.0f, 0.5f, 0.0f, 0.5f, -0.5f, 0.0f, -0.5f, -0.5f, 0.0f,
};

GLint common_get_shader_program(const char *vertex_shader_source,
                                const char *fragment_shader_source) {
  enum Consts { INFOLOG_LEN = 512 };
  GLchar infoLog[INFOLOG_LEN];
  GLint fragment_shader;
  GLint shader_program;
  GLint success;
  GLint vertex_shader;

  /* Vertex shader */
  vertex_shader = glCreateShader(GL_VERTEX_SHADER);
  glShaderSource(vertex_shader, 1, &vertex_shader_source, NULL);
  glCompileShader(vertex_shader);
  glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(vertex_shader, INFOLOG_LEN, NULL, infoLog);
    printf("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n%s\n", infoLog);
  }

  /* Fragment shader */
  fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
  glShaderSource(fragment_shader, 1, &fragment_shader_source, NULL);
  glCompileShader(fragment_shader);
  glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(fragment_shader, INFOLOG_LEN, NULL, infoLog);
    printf("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n%s\n", infoLog);
  }

  /* Link shaders */
  shader_program = glCreateProgram();
  glAttachShader(shader_program, vertex_shader);
  glAttachShader(shader_program, fragment_shader);
  glLinkProgram(shader_program);
  glGetProgramiv(shader_program, GL_LINK_STATUS, &success);
  if (!success) {
    glGetProgramInfoLog(shader_program, INFOLOG_LEN, NULL, infoLog);
    printf("ERROR::SHADER::PROGRAM::LINKING_FAILED\n%s\n", infoLog);
  }

  glDeleteShader(vertex_shader);
  glDeleteShader(fragment_shader);
  return shader_program;
}

int main(void) {
  GLuint shader_program, vbo;
  GLint pos;
#ifdef __WEBROGUE__
#else
  u32 WindowFlags = SDL_WINDOW_OPENGL;
  SDL_Window *Window =
      SDL_CreateWindow("OpenGL Test", 0, 0, WinWidth, WinHeight, WindowFlags);
  assert(Window);
  SDL_GLContext Context = SDL_GL_CreateContext(Window);
#endif

  // printf("GL_VERSION  : %s\n", glGetString(GL_VERSION)); TODO
  // printf("GL_RENDERER : %s\n", glGetString(GL_RENDERER)); TODO

  shader_program =
      common_get_shader_program(vertex_shader_source, fragment_shader_source);
  pos = glGetAttribLocation(shader_program, "position");

  glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
  glViewport(0, 0, WIDTH, HEIGHT);

  glGenBuffers(1, &vbo);
  glBindBuffer(GL_ARRAY_BUFFER, vbo);
  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
  glVertexAttribPointer(pos, 3, GL_FLOAT, GL_FALSE, 0, (GLvoid *)0);
  glEnableVertexAttribArray(pos);
  glBindBuffer(GL_ARRAY_BUFFER, 0);

  while (1) {
    // glfwPollEvents();
    glClear(GL_COLOR_BUFFER_BIT);
    glUseProgram(shader_program);
    glDrawArrays(GL_TRIANGLES, 0, 3);
#ifdef __WEBROGUE__
    wr_gl_present();
#else
    SDL_GL_SwapWindow(Window);
#endif
  }
  glDeleteBuffers(1, &vbo);
  // glfwTerminate();
  return EXIT_SUCCESS;
}
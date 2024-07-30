#include <GLES2/gl2.h>

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

int main(int ArgCount, char **Args) {
  b32 Running = 1;
#ifdef __WEBROGUE__
#else
  u32 WindowFlags = SDL_WINDOW_OPENGL;
  SDL_Window *Window =
      SDL_CreateWindow("OpenGL Test", 0, 0, WinWidth, WinHeight, WindowFlags);
  assert(Window);
  SDL_GLContext Context = SDL_GL_CreateContext(Window);
#endif

  b32 FullScreen = 0;
  while (Running) {
    // SDL_Event Event;
    // while (SDL_PollEvent(&Event)) {
    //   if (Event.type == SDL_KEYDOWN) {
    //     switch (Event.key.keysym.sym) {
    //     case SDLK_ESCAPE:
    //       Running = 0;
    //       break;
    //     case 'f':
    //       FullScreen = !FullScreen;
    //       if (FullScreen) {
    //         SDL_SetWindowFullscreen(Window, WindowFlags |
    //                                             SDL_WINDOW_FULLSCREEN_DESKTOP);
    //       } else {
    //         SDL_SetWindowFullscreen(Window, WindowFlags);
    //       }
    //       break;
    //     default:
    //       break;
    //     }
    //   } else if (Event.type == SDL_QUIT) {
    //     Running = 0;
    //   }
    // }

    glViewport(0, 0, WinWidth, WinHeight);
    glClearColor(1.f, 0.f, 1.f, 0.f);
    glClear(GL_COLOR_BUFFER_BIT);
#ifdef __WEBROGUE__
    wr_gl_present();
#else
    SDL_GL_SwapWindow(Window);
#endif
  }
  return 0;
}
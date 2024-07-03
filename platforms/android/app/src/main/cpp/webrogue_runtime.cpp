extern "C" void webrogue_android_main();

extern "C" int SDL_main(int argc, char *argv[]) {
  webrogue_android_main();
  return 0;
}

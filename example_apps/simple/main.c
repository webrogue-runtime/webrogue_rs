#include "stdlib.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

__attribute__((import_name("imported_func_1")))
__attribute__((import_module("webrogue"))) void
imported_func_1(int a);

__attribute__((import_name("imported_func_2")))
__attribute__((import_module("webrogue"))) int
imported_func_2(int a);

__attribute__((import_name("imported_func_3")))
__attribute__((import_module("webrogue"))) void
imported_func_3(size_t ptr, int len);

struct example_struct {
  int a;
  int b;
};

__attribute__((export_name("exported_func_1"))) void exported_func_1() {
  struct example_struct estruct;
  estruct.a = 1;
  estruct.b = 2;
  struct example_struct *estruct_ptr = &estruct;
  int i = 1;
  char *str = "test_str";
  // imported_func_1(*str);
  // imported_func_3((size_t)str, 8);
  *((uint16_t *)str) = 52;
  // imported_func_1(*str);
  // imported_func_3((size_t)str, 8);
  // imported_func_1(*str);
  // imported_func_1(imported_func_2(20));
}
int main(int argc, char **argv) {
  // FILE *file = fopen("/example_mods/simple/test_file.txt", "r");
  char readed[12] = "unin\0\0\0\0\0\0\0\0";
  readed[11] = '\0';
  // fread(readed, 1, 11, file);
  // fclose(file);

  for (int s = 1; s < 30; s++) {
    printf("test: %s", readed);
  }
  printf("test: %s\n", readed);
  sleep(2);
  for (int s = 1; s < 40; s++) {
    usleep(200 * 1000);
    printf("s: %d\n", s);
  }
  // imported_func_1(argc);
  // imported_func_3((size_t)argv[0], strlen(argv[0]));
  exported_func_1();
}
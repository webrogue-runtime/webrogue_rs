#include "stdlib.h"
#include <stdint.h>
#include <string.h>

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
  imported_func_1(*str);
  imported_func_3((size_t)str, 8);
  *((uint16_t *)str) = 52;
  imported_func_1(*str);
  imported_func_3((size_t)str, 8);
  imported_func_1(*str);
  imported_func_1(imported_func_2(20));
}

int main(int argc, char **argv) {
  // imported_func_1(argc);
  // imported_func_3((size_t)argv[0], strlen(argv[0]));
  exported_func_1();
}
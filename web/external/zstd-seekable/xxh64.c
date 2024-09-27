#include <xxhash.h>

unsigned xxh64(const unsigned char* src, int len) {
  return XXH64(src, len, 0);
}

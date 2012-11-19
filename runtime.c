#include <stdio.h>

int run(void * (*rustFn)()) {
  printf("Hello from C\n");
  rustFn();
}


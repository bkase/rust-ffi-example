#include <stdio.h>

int run(int argc, char ** argv, void * (*rustFn)()) {
  int i;
  printf("Hello from C:\n");
  for (i = 0; i < argc; i++) {
    printf("\targ %d: %s\n", i, argv[i]);
  }
  rustFn();
}


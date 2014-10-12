#include <stdlib.h>
#include <stdio.h>

void printAndFree(int* value) {
  printf("Value was: %u\n", *value);

  free(value);
}

int main()
{
   int *value = malloc(sizeof(int));
   *value = 123;
   printAndFree(value);

   printf("Value now is: %u\n", *value);
}


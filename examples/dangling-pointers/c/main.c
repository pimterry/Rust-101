#include <stdio.h>

int* returnDanglingPointer()
{
  int myNum = 123;
  return &myNum;
}

int main()
{
   int *ptr = returnDanglingPointer();
   printf("%u", *ptr);
}

#include <stdio.h>
 
int factorial(int n) {
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}
 
int main(int argc, char ** argv) {
  int number = 5;
  int fact = factorial(number);
  printf("The factorial of %d is %d", number, fact);
}
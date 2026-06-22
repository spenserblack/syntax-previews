#ifndef SOME_MACRO
#define SOME_MACRO
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#define println(str) printf("%s\n", str)
/**
 * Block comment
 */
int main(int argc, char * argv[]) {
	// Normal comment
	const int a = 1;
	const int b = 2;
	const bool is_example = true;
	const char * nil = NULL;
	printf("%d + %d = %d\n", a, b, a + b);
	println("Hello, world!");

	return 0;
}
#endif

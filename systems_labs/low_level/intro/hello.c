// The preprocessor (cpp) modifies the original C program
// accoring to directives that begin with "#" character,
// which means:
//   * The pre-processor reads the contents of the system header file stdio.h
//   * Insert the stdio.h directly into the program text.
//   * Generates another C program, with the .i suffix
#include <stdio.h>

// Compile with gcc compiler
// -o tells how to call the file
// gcc file.c -o output-file

// Run: ./filename

int main()
{
    printf("Hello, world\n");
    return 0;
}

/*

**HELLO WORLD BIT REPRESENTATION**

#   i    n    c  l   u     d    e
35  105  110  99 108  117  100  101

In summary:

- The ASCII text representation of hello.c
*/
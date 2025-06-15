#include <stdio.h>
#include "common.h"
#include "compiler.h"
#include "scanner.h"

void compile(Scanner* scanner, const char *source) {
    initScanner(scanner, source);
    int line = -1;
    for (;;){
        Token token = scanToken(scanner);

        if (token.line != line) {
            printf("%4d ", token.line); // Print the line number
            line = token.line; // Update the current line
        } else {
            printf("   | "); // Align tokens on the same line
        }
        printf("%2d '%.*s'\n", token.type,token.lenght, token.start);

                if (token.type == TOKEN_EOF) {
            break; // End of file reached
        }
    }
}

#include "compiler.h"
#include "scanner.h"
#include <stdio.h>

void compile(Scanner *scanner, const char *source) {
  initScanner(scanner, source);
  int line = -1;
  for (;;) {
    Token token = scanToken(scanner);

    if (token.line != line) {
      printf("%4d ", token.line);
      line = token.line;
    } else {
      printf("   | ");
    }
    printf("%2d '%.*s'\n", token.type, token.lenght, token.start);

    if (token.type == TOKEN_EOF) {
      break;
    }
  }
}

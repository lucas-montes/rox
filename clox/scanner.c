#include <string.h>
#include "scanner.h"
#include "common.h"
#include <stdio.h>

void initScanner(Scanner* scanner, const char *source) {
  scanner->start = source;  // Set the start pointer to the source code
  scanner->current = source; // Initialize the current pointer to the start
  scanner->line = 1;        // Start at line 1
}

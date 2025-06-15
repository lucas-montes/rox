// clox/tests/test_scanner.c
#include "../scanner.h"
#include <assert.h>
#include <stdio.h>

void test_scanner_basic() {
    Scanner scanner;
    initScanner(&scanner, "var x = 42;");
    assert(scanner.start != NULL);
    assert(scanner.current != NULL);
    assert(scanner.line == 1);
    Token token = scanToken(&scanner);
    assert(token.type == TOKEN_VAR);
    assert(token.lenght == 3);
}

int main() {
    test_scanner_basic();
    printf("✅ Scanner tests passed.\n");
    return 0;
}

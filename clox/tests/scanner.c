// clox/tests/test_scanner.c
#include "../scanner.h"
#include <assert.h>
#include <stdio.h>

void test_scanner_basic() {
    Scanner scanner = initScanner("var x = 42;");
    char c = advance(&scanner);
    assert(c == 'v');  // basic smoke test
}

int main() {
    test_scanner_basic();
    printf("✅ Scanner tests passed.\n");
    return 0;
}

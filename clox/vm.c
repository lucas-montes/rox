#include "vm.h"
#include "chunk.h"
#include "debug.h"
#include "value.h"
#include <stdio.h>

static void resetStack(VM *vm) { vm->stackTop = vm->stack; }

VM initVM() {
  VM vm;
  resetStack(&vm);
  return vm;
}

void pushVM(VM *vm, Value value) {
  // NOTE: ponter magic. The stacktop points to the location in the array, so
  // when we set the value in puts it in the array
  // then when we increase stacktop, we are move the pointer to the next
  // location
  *vm->stackTop = value;
  vm->stackTop++;
}

Value popVM(VM *vm) {
  vm->stackTop--;
  return *vm->stackTop;
}

static InterpretResult run(VM *vm) {
#define READ_BYTE() (*vm->ip++)
#define READ_CONSTANT() (vm->chunk->constants.values[READ_BYTE()])

  for (;;) {

#ifdef DEBUG_TRACE_EXECUTION
    printf("    ");
    for (Value *slot = vm->stack; slot < vm->stackTop; slot++) {
      printf("[ ");
      printValue(*slot);
      printf(" ]");
    }
    printf("\n");
    disassembleInstruction(vm->chunk, (int)(vm->ip - vm->chunk->code));
#endif

    uint8_t instruction;
    switch (instruction = READ_BYTE()) {
    case OP_RETURN: {
      Value popedValue = popVM(vm);
      printf("return called \n");
      printValue(popedValue);
      printf("\n");
      return INTERPRET_OK;
    }
    case OP_CONSTANT: {
      Value constant = READ_CONSTANT();
      pushVM(vm, constant);
      break;
    }
    case OP_NEGATE: {
      Value popedValue = popVM(vm);
      pushVM(vm, -popedValue);
      break;
    }
    }
  }
#undef READ_BYTE
#undef READ_CONSTANT
}

InterpretResult interpret(VM *vm, Chunk *chunk) {
  vm->chunk = chunk;
  vm->ip = vm->chunk->code;
  return run(vm);
}

void freeVM(VM *vm) {}

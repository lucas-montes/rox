
#ifndef clox_vm_h
#define clox_vm_h

#define STACK_MAX 256
#include "chunk.h"
#include "value.h"

typedef enum {
  INTERPRET_OK,
  INTERPRET_COMPILE_ERROR,
  INTERPRET_RUNTIME_ERROR
} InterpretResult;

typedef struct {
  Chunk *chunk;
  uint8_t *ip; // the next instruction
  Value stack[STACK_MAX];
  Value *stackTop; // place where the next value will go
} VM;

VM initVM();
void freeVM(VM *vm);
void pushVM(VM *vm, Value value);
Value popVM(VM *vm);
InterpretResult interpret(VM *vm, Chunk *chunk);
#endif

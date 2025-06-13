#include "chunk.h"
#include "debug.h"
#include "vm.h"

int main(int argc, const char *argv[]) {
  VM vm;
  initVM(&vm);
  Chunk chunk;
  initChunk(&chunk);
  size_t constant1 = addConstant(&chunk, 43.0000000000009);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant1, 12);
  size_t constant = addConstant(&chunk, 190.90902);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant, 12);
  writeChunk(&chunk, OP_NEGATE, 12);

  writeChunk(&chunk, OP_RETURN, 13);

  disassembleChunk(&chunk, "test chunk");
  interpret(&vm, &chunk);

  freeVM(&vm);
  freeChunk(&chunk);
  return 0;
}

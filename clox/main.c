#include "chunk.h"
#include "debug.h"
#include "vm.h"

int main(int argc, const char *argv[]) {
  VM vm;
  initVM(&vm);
  Chunk chunk;
  initChunk(&chunk);
  size_t constant1 = addConstant(&chunk, 4.4);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant1, 12);
  size_t constant = addConstant(&chunk, 2);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant, 12);
  writeChunk(&chunk, OP_DIVIDE, 12);
  writeChunk(&chunk, OP_NEGATE, 12);
  size_t constant3 = addConstant(&chunk, 3);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant3, 12);
  writeChunk(&chunk, OP_MULTIPLY, 12);

  writeChunk(&chunk, OP_RETURN, 13);

  disassembleChunk(&chunk, "test chunk");
  interpret(&vm, &chunk);

  freeVM(&vm);
  freeChunk(&chunk);
  return 0;
}

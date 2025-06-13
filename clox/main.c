#include "chunk.h"
#include "debug.h"
#include "vm.h"

int main(int argc, const char *argv[]) {
  VM vm = initVM();
  Chunk chunk;
  initChunk(&chunk);
  writeChunk(&chunk, OP_RETURN, 11);
  int constant = addConstant(&chunk, 1.2);
  writeChunk(&chunk, OP_CONSTANT, 12);
  writeChunk(&chunk, constant, 12);
  writeChunk(&chunk, OP_NEGATE, 12);
  writeChunk(&chunk, OP_RETURN, 13);

  disassembleChunk(&chunk, "test chunk");
  freeVM(&vm);
  freeChunk(&chunk);
  return 0;
}

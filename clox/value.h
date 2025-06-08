
#ifndef clox_value_h
#define clox_value_h

#include "common.h"

typedef double Value;

typedef struct {
  size_t count;  // points to the next location in the array
  size_t capacity;
  uint8_t *values;
} ValueArray;

void printValue(Value value);
void initValueArray(ValueArray *array);
void freeValueArray(ValueArray *array);
void writeValueArray(ValueArray *array, Value value);
#endif

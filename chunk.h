#pragma once

#include "value.h"
#include "common.h"

typedef enum
{
    OP_CONSTANT,
    OP_CONSTANT_LONG,

    OP_NONE,
    OP_TRUE,
    OP_FALSE,
    OP_POP,

    OP_DEFINE_GLOBAL,
    OP_GET_GLOBAL,
    
    OP_BANG_EQUAL,
    OP_EQUAL_EQUAL,
    OP_GREATER,
    OP_LESS,
    OP_GREATER_EQUAL,
    OP_LESS_EQUAL,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NOT,

    OP_PRINT,
    OP_RETURN
} OpCode;

typedef struct
{
    // OpCode
    int len;
    int cap;
    uint8_t *code;

    // Line Information
    // Run-length encoded
    int line_cap;
    int *lines;

    // Constant Pool
    ValueArray constants;
} Chunk;

void initChunk(Chunk *chunk);
void freeChunk(Chunk *chunk);
void writeChunk(Chunk *chunk, uint8_t byte, int line);

int addConstant(Chunk *chunk, Value value);
void writeConstant(Chunk *chunk, Value value, int line);

// debug
int getLine(Chunk *chunk, int offset);
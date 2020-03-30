#include "chunk.h"
#include "memory.h"

void initChunk(Chunk *chunk)
{
    chunk->len = 0;
    chunk->cap = 0;
    chunk->code = NULL;

    chunk->line_cap = 0;
    chunk->lines = NULL;

    initValueArray(&chunk->constants);
}

void freeChunk(Chunk *chunk)
{
    reallocate(chunk->code, sizeof(uint8_t) * chunk->cap, 0);
    reallocate(chunk->lines, sizeof(int) * chunk->line_cap, 0);
    freeValueArray(&chunk->constants);
    initChunk(chunk);
}

void writeChunk(Chunk *chunk, uint8_t byte, int line)
{
    // Resize the chunk
    if (chunk->len + 1 > chunk->cap)
    {
        int oldCap = chunk->cap;
        chunk->cap = (oldCap < 8) ? 8 : (oldCap * 2);
        chunk->code = (uint8_t *)reallocate(chunk->code, sizeof(uint8_t) * (oldCap), sizeof(uint8_t) * chunk->cap);
    }

    // Resize the run-length encoded array
    if (line > chunk->line_cap)
    {
        int oldCap = chunk->line_cap;
        chunk->line_cap = line;
        chunk->lines = (int *)reallocate(chunk->lines, sizeof(int) * oldCap, sizeof(int) * chunk->line_cap);
        for (int i = oldCap; i < chunk->line_cap; i++)
            chunk->lines[i] = 0;
    }

    chunk->code[chunk->len] = byte;
    chunk->len++;

    chunk->lines[line - 1]++;
}

int addConstant(Chunk *chunk, Value value)
{
    writeValueArray(&chunk->constants, value);
    return chunk->constants.len - 1;
}

void writeConstant(Chunk *chunk, Value value, int line)
{
    int constant = addConstant(chunk, value);

    // If our constant pool is full (since we are representing it as 8 bits)
    // We use a OP_LONG_CONSTANT and store the bits as a 24bit number stored
    // over 3 bytes.
    if (constant > UINT8_MAX)
    {
        writeChunk(chunk, OP_CONSTANT_LONG, line);
        writeChunk(chunk, constant >> 16, line);
        writeChunk(chunk, constant >> 8, line);
        writeChunk(chunk, constant, line);
    }
    else
    {
        writeChunk(chunk, OP_CONSTANT, line);
        writeChunk(chunk, constant, line);
    }
}

// debug
int getLine(Chunk *chunk, int offset)
{
    int acc = 0;
    for (int i = 0; i < chunk->len; i++)
    {
        acc += chunk->lines[i];
        if (offset < acc)
            return i + 1;
    }
    return chunk->line_cap;
}
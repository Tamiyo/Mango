#pragma once

#include "chunk.h"
#include "value.h"
#include "table.h"

// TODO - Remove this in favor of a dynamic stack
#define STACK_MAX 1024

typedef struct VM
{
    Chunk *chunk;
    uint8_t* ip;

    // THE one and only stack
    Value stack[STACK_MAX];
    Value* stackTop;

    // Symbol table
    Table globals;

    // Table of interned strings
    Table strings;

    // Keeps track of objects allocated on the heap
    // so that memory isnt leaked. We can use this
    // Linked List later to deallocate unused objects.
    Obj* objects;
} VM;

typedef enum {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
} InterpretResult;

extern VM vm;

// VM Operations
void initVM();
void freeVM();
InterpretResult interpret(const char* source);

// Stack Operations
void push(Value value);
Value pop();
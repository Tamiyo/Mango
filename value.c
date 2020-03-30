#include <stdio.h>
#include <string.h>
#include <math.h>

#include "object.h"
#include "memory.h"
#include "value.h"

// Helper Functions

// static uint32_t hashValue(Value value)
// {
//     switch (value.type)
//     {
//     case VAL_NUMBER:
//         long v = (long)(AS_NUMBER(value));
//         return (uint32_t)(v, (v >> 32));
//     case VAL_BOOL:
//         bool b = (AS_BOOL(value));
//         return b ? 1231 : 1237;
//     case VAL_OBJ:
        
//         return ;
//     default:
//         return; // Unreachable
//     }
// }

bool valuesEqual(Value a, Value b)
{
    if (a.type != b.type)
        return false;

    switch (a.type)
    {
    case VAL_BOOL:
        return AS_BOOL(a) == AS_BOOL(b);
    case VAL_NONE:
        return false;
    case VAL_NUMBER:
        return AS_NUMBER(a) == AS_NUMBER(b);
    case VAL_OBJ:
        return AS_OBJ(a) == AS_OBJ(b);
    }
}

// Memory Management
void initValueArray(ValueArray *array)
{
    array->cap = 0;
    array->len = 0;
    array->values = NULL;
}

void writeValueArray(ValueArray *array, Value value)
{
    if (array->len + 1 > array->cap)
    {
        int oldCap = array->cap;
        array->cap = (oldCap < 8) ? 8 : oldCap * 2;
        array->values = (Value *)reallocate(array->values, sizeof(Value) * (oldCap), sizeof(Value) * array->cap);
    }

    array->values[array->len] = value;
    array->len++;
}

void freeValueArray(ValueArray *array)
{
    reallocate(array->values, sizeof(Value) * array->cap, 0);
    initValueArray(array);
}

// Debug
void printValue(Value value)
{
    switch (value.type)
    {
    case VAL_BOOL:
        printf(AS_BOOL(value) ? "true" : "false");
        break;
    case VAL_NONE:
        printf("none");
        break;
    case VAL_NUMBER:
        printf("%g", AS_NUMBER(value));
        break;
    case VAL_OBJ:
        printObject(value);
        break;
    }
}
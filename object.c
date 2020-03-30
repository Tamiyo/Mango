#include <stdio.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "value.h"
#include "vm.h"
#include "table.h"

// Memory Allocation
static Obj *allocateObject(size_t size, ObjType type)
{
    Obj *object = (Obj *)reallocate(NULL, 0, size);
    object->type = type;

    // Whenever we allocate an object, we want to add it
    // to keep track of it in the VM.
    object->next = vm.objects;
    vm.objects = object;

    return object;
}

static ObjString *allocateString(char *chars, int length, uint32_t hash)
{
    ObjString *string = (ObjString *)allocateObject(sizeof(ObjString) + sizeof(char) * length, OBJ_STRING);
    string->length = length;
    string->hash = hash;
    strcpy(string->chars, chars);

    tableSet(&vm.strings, string, NONE_VAL);

    return string;
}

// Helper Functions

// Hash function to hash string values
static uint32_t hashString(const char *key, int length)
{
    uint32_t hash = 2166136261u;

    for (int i = 0; i < length; i++)
    {
        hash ^= key[i];
        hash *= 16777619;
    }

    return hash;
}

// This function transfers ownership of the string by
// directly allocating a new ObjString and transfering
// ownership of the pointers to the new struct.
ObjString *takeString(char *chars, int length)
{
    uint32_t hash = hashString(chars, length);
    ObjString *interned = tableFindString(&vm.strings, chars, length, hash);
    if (interned != NULL)
    {
        reallocate(chars, sizeof(char) * length, 0);
        return interned;
    }

    return allocateString(chars, length, hash);
}

// We explicitly copy the string onto the heap so that every
// ObjString owns its own character array instead of pointing
// to the source code.
//
// This also assumes that we do not take ownership of the string, but
// instead we copy it.
ObjString *copyString(const char *chars, int length)
{
    uint32_t hash = hashString(chars, length);
    ObjString *interned = tableFindString(&vm.strings, chars, length, hash);
    if (interned != NULL)
        return interned;

    char *heapChars = (char *)reallocate(NULL, 0, sizeof(char) * (length + 1));
    memcpy(heapChars, chars, length);
    heapChars[length] = '\0';

    return allocateString(heapChars, length, hash);
}

// debug
void printObject(Value value)
{
    switch (OBJ_TYPE(value))
    {
    case OBJ_STRING:
        printf("%s", AS_CSTRING(value));
        break;
    }
}
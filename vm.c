#include <stdio.h>
#include <stdarg.h>
#include <string.h>

#include "common.h"
#include "vm.h"
#include "debug.h"
#include "object.h"
#include "memory.h"
#include "compiler.h"

// VM Memory Operations
void initVM()
{
    // Reset Stack
    vm.stackTop = vm.stack;

    // Reset Objects
    vm.objects = NULL;

    // Init Global Variables
    initTable(&vm.globals);

    // Init Interned Strings
    initTable(&vm.strings);
}

void freeVM()
{

    freeTable(&vm.globals);
    freeTable(&vm.strings);
    freeObjects();
}

// Error Handling
static void runtimeError(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
    fputs("\n", stderr);

    size_t instruction = vm.ip - vm.chunk->code - 1;
    int line = getLine(vm.chunk, instruction);
    fprintf(stderr, "[line %d] in script\n", line);

    // Reset Stack
    vm.stackTop = vm.stack;
}

// Stack Operations
void push(Value value)
{
    *vm.stackTop = value;
    *vm.stackTop++;
}

Value pop()
{
    *vm.stackTop--;
    return *vm.stackTop;
}

static Value peek(int distance)
{
    return vm.stackTop[-1 - distance];
}

static InterpretResult run()
{
// Will **remove** this, temporary boilerplate macro to get the VM running
#define READ_BYTE() (*vm.ip++)
#define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])
#define READ_STRING() AS_STRING(READ_CONSTANT())
#define BINARY_OP(valueType, op)                        \
    do                                                  \
    {                                                   \
        if (!IS_NUMBER(peek(0)) || !IS_NUMBER(peek(1))) \
        {                                               \
            runtimeError("Operands must be numbers.");  \
            return INTERPRET_RUNTIME_ERROR;             \
        }                                               \
                                                        \
        double b = AS_NUMBER(pop());                    \
        double a = AS_NUMBER(pop());                    \
        push(valueType(a op b));                        \
    } while (false)

    for (;;)
    {

#ifdef DEBUG_TRACE_EXECUTION
        // Print the stack cleanly
        printf("          ");
        for (Value *slot = vm.stack; slot < vm.stackTop; slot++)
        {
            printf("[ ");
            printValue(*slot);
            printf(" ]");
        }
        printf("\n");

        // Disassemble the instruction
        disassembleInstruction(vm.chunk, (int)(vm.ip - vm.chunk->code));
#endif
        uint8_t instruction = READ_BYTE();
        switch (instruction)
        {
        case OP_CONSTANT_LONG:
        case OP_CONSTANT:
        {
            Value constant = READ_CONSTANT();
            push(constant);
            break;
        }
        case OP_NONE:
            push(NONE_VAL);
            break;
        case OP_TRUE:
            push(BOOL_VAL(true));
            break;
        case OP_FALSE:
            push(BOOL_VAL(false));
            break;
        case OP_POP:
            pop();
            break;
        case OP_DEFINE_GLOBAL:
        {
            ObjString *name = READ_STRING();
            tableSet(&vm.globals, name, peek(0));
            pop();
            break;
        }
        case OP_GET_GLOBAL:
        {
            ObjString *name = READ_STRING();
            Value value;
            if (!tableGet(&vm.globals, name, &value))
            {
                runtimeError("Undefined variable '%s'.", name->chars);
                return INTERPRET_RUNTIME_ERROR;
            }
            push(value);
            break;
        }
        case OP_BANG_EQUAL:
        {
            Value b = pop();
            Value a = pop();
            push(BOOL_VAL(!valuesEqual(a, b)));
            break;
        }
        case OP_EQUAL_EQUAL:
        {
            Value b = pop();
            Value a = pop();
            push(BOOL_VAL(valuesEqual(a, b)));
            break;
        }
        case OP_LESS:
            BINARY_OP(BOOL_VAL, <);
            break;
        case OP_LESS_EQUAL:
            BINARY_OP(BOOL_VAL, <=);
            break;
        case OP_GREATER:
            BINARY_OP(BOOL_VAL, >);
            break;
        case OP_GREATER_EQUAL:
            BINARY_OP(BOOL_VAL, >=);
            break;
        case OP_ADD:
        {
            if (IS_STRING(peek(0)) && IS_STRING(peek(1)))
            {
                ObjString *b = AS_STRING(pop());
                ObjString *a = AS_STRING(pop());

                int length = a->length + b->length;
                char *chars = (char *)reallocate(NULL, 0, sizeof(char) * (length + 1));
                memcpy(chars, a->chars, a->length);
                memcpy(chars + a->length, b->chars, b->length);
                chars[length] = '\0';

                ObjString *result = takeString(chars, length);
                push(OBJ_VAL(result));
            }
            else if (IS_NUMBER(peek(0)) && IS_NUMBER(peek(1)))
            {
                BINARY_OP(NUMBER_VAL, +);
            }
            else
            {
                runtimeError("Operands must be two numbers or two strings.");
                return INTERPRET_RUNTIME_ERROR;
            }
            break;
        }
        case OP_SUBTRACT:
            BINARY_OP(NUMBER_VAL, -);
            break;
        case OP_MULTIPLY:
            BINARY_OP(NUMBER_VAL, *);
            break;
        case OP_DIVIDE:
            BINARY_OP(NUMBER_VAL, /);
            break;
        case OP_NOT:
        {
            Value value = pop();
            if (!IS_BOOL(value))
            {
                runtimeError("Operator must be applied to a bool.");
                return INTERPRET_RUNTIME_ERROR;
            }
            push(BOOL_VAL(!AS_BOOL(value)));
            break;
        }
        case OP_PRINT:
        {
            printValue(pop());
            printf("\n");
            break;
        }
        case OP_RETURN:
            // Exit interpreter
            return INTERPRET_OK;
        }
    }

#undef BINRARY_OP
#undef READ_STRING
#undef READ_CONSTANT
}

InterpretResult interpret(const char *source)
{
    Chunk chunk;
    initChunk(&chunk);

    if (!compile(source, &chunk))
    {
        freeChunk(&chunk);
        return INTERPRET_COMPILE_ERROR;
    }

    vm.chunk = &chunk;
    vm.ip = chunk.code;

    InterpretResult result = run();

    freeChunk(&chunk);
    return result;
}
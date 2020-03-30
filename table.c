#include <stdlib.h>
#include <string.h>

#include "memory.h"
#include "table.h"

// Memory Management
void initTable(Table *table)
{
    table->len = 0;
    table->cap = 0;
    table->entries = NULL;
}

void freeTable(Table *table)
{
    reallocate(table->entries, sizeof(Entry) * table->cap, 0);
    initTable(table);
}

// Table Operations
static Entry *findEntry(Entry *entries, int capacity, ObjString *key)
{
    uint32_t index = key->hash % capacity;
    Entry *tombstone = NULL;

    for (;;)
    {
        Entry *entry = &entries[index];

        if (entry->key == NULL)
        {
            if (IS_NONE(entry->value))
            {
                // Empty entry.
                return tombstone != NULL ? tombstone : entry;
            }
            else
            {
                // We found a tombstone.
                if (tombstone == NULL)
                    tombstone = entry;
            }
        }
        else if (entry->key == key)
        {
            // We found the key.
            return entry;
        }

        index = (index + 1) % capacity;
    }
}

static void adjustCapacity(Table *table, int capacity)
{
    Entry *entries = (Entry *)reallocate(NULL, 0, sizeof(Entry) * capacity);
    for (int i = 0; i < capacity; i++)
    {
        entries[i].key = NULL;
        entries[i].value = NONE_VAL;
    }

    table->len = 0;
    for (int i = 0; i < table->cap; i++)
    {
        Entry *entry = &table->entries[i];
        if (entry->key == NULL)
            continue;

        Entry *dest = findEntry(entries, capacity, entry->key);
        dest->key = entry->key;
        dest->value = entry->value;
        table->len++;
    }

    reallocate(table->entries, table->cap, sizeof(Entry) * table->cap);
    table->entries = entries;
    table->cap = capacity;
}

bool tableGet(Table *table, ObjString *key, Value *value)
{
    if (table->len == 0)
        return false;

    Entry *entry = findEntry(table->entries, table->cap, key);
    if (entry->key == NULL)
        return false;

    *value = entry->value;
    return true;
}

bool tableSet(Table *table, ObjString *key, Value value)
{
    if (table->len + 1 > table->cap * TABLE_MAX_LOAD)
    {
        int capacity = ((capacity < 8) ? 8 : (capacity)*2);
        adjustCapacity(table, capacity);
    }

    Entry *entry = findEntry(table->entries, table->cap, key);

    bool isNewKey = entry->key == NULL;
    if (isNewKey && IS_NONE(entry->value))
        table->len++;
    if (isNewKey)
        table->len++;

    entry->key = key;
    entry->value = value;
    return isNewKey;
}

bool tableDelete(Table *table, ObjString *key)
{
    if (table->len == 0)
        return false;

    // Find the entry.
    Entry *entry = findEntry(table->entries, table->cap, key);
    if (entry->key == NULL)
        return false;

    // Place a tombstone in the entry.
    entry->key = NULL;
    entry->value = BOOL_VAL(true);

    return true;
}

void tableAddAll(Table *from, Table *to)
{
    for (int i = 0; i < from->cap; i++)
    {
        Entry *entry = &from->entries[i];
        if (entry->key != NULL)
        {
            tableSet(to, entry->key, entry->value);
        }
    }
}

// To use for String Iterning
ObjString *tableFindString(Table *table, const char *chars, int length, uint32_t hash)
{
    if (table->len == 0)
        return NULL;

    uint32_t index = hash % table->cap;

    for (;;)
    {
        Entry *entry = &table->entries[index];

        if (entry->key == NULL)
        {
            // Stop if we find an empty non-tombstone entry.
            if (IS_NONE(entry->value))
                return NULL;
        }
        else if (entry->key->length == length &&
                 entry->key->hash == hash &&
                 memcmp(entry->key->chars, chars, length) == 0)
        {
            // We found it.
            return entry->key;
        }

        index = (index + 1) % table->cap;
    }
}
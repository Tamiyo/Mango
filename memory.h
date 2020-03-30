#pragma once

#include "object.h"

void *reallocate(void *previous, size_t oldSize, size_t newSize);

void freeObjects();
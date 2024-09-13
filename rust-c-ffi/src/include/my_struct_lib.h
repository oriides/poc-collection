#pragma once

#include <stdlib.h>

typedef struct MyCStruct
{
    int x;
} MyCStruct;

struct MyCStruct *new(int x);

void drop(struct MyCStruct *s);

void multiply(MyCStruct *s, int factor);

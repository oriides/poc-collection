#include "my_struct_lib.h"

struct MyCStruct *new(int x)
{
    struct MyCStruct *s = malloc(sizeof(*s));
    if (s != NULL)
        s->x = x;
    return s;
}

void drop(struct MyCStruct *s)
{
    free(s);
}

void multiply(MyCStruct *s, int factor)
{
    s->x *= factor;
};
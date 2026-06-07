#ifndef IVO_COMMON_H
#define IVO_COMMON_H

typedef struct {
    size_t start, end;
} span;

typedef enum {
    OP_PLUS, OP_MINUS, OP_STAR, OP_SLASH, OP_MODULO
} op;

#endif

#ifndef IVO_RESULT_H
#define IVO_RESULT_H

#include <stdbool.h>

#define SIMPLE_ERR(msg) return (result_t){\
    .is_ok = false,\
    .payload = { .err = strdup(msg) }\
}

typedef struct {
    bool is_ok;
    union {
        void* ok;
        char* err;
    } payload;
} result_t;

#endif

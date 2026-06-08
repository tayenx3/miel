#ifndef IVO_COMMON_H
#define IVO_COMMON_H

#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#ifdef _WIN32
    char* strndup(const char* s, size_t n);
    #define strdup _strdup
#endif

#define STREQ(a, b) !strcmp(a, b)

#define CASE_TO_STR(a) case a: return #a

#define DECL_VEC(item_type, name) \
typedef struct {\
    item_type* data;\
    size_t len;\
    size_t cap;\
} name##_t;\
name##_t create_##name(void);\
result_t push_to_##name(name##_t* vec, item_type item);\
void destroy_##name(name##_t* vec);

#ifdef ENOMEM
    #define IMPL_VEC(item_type, name, item_destroy) \
    name##_t create_##name(void) {\
        return (name##_t){0};\
    }\
    result_t push_to_##name(name##_t* vec, item_type item) {\
        if (vec->len >= vec->cap) {\
            vec->cap = vec->cap ? (vec->cap * 2) : 64;\
            item_type* new_data = realloc(vec->data, vec->cap * sizeof(item_type));\
            if (!new_data) {\
                if (errno == ENOMEM) SIMPLE_ERR("Out of memory");\
                else SIMPLE_ERR(strerror(errno));\
            }\
            vec->data = new_data;\
        }\
        vec->data[vec->len++] = item;\
        return (result_t){\
            .is_ok = true,\
            .payload = { .ok = NULL }\
        };\
    }\
    void destroy_##name(name##_t* vec) {\
        for (size_t i = 0; i < vec->len; i++)\
            (item_destroy)(vec->data + i);\
        free(vec->data);\
        vec->data = NULL;\
        vec->len = vec->cap = 0;\
    }
#else
#define IMPL_VEC(item_type, name, item_destroy) \
name##_t create_##name(void) {\
    return (name##_t){0};\
}\
result_t push_to_##name(name##_t* vec, item_type item) {\
    if (vec->len >= vec->cap) {\
        vec->cap = vec->cap ? (vec->cap * 2) : 64;\
        item_type* new_data = realloc(vec->data, vec->cap * sizeof(item_type));\
        if (!new_data) {\
            SIMPLE_ERR(strerror(errno));\
        }\
        vec->data = new_data;\
    }\
    vec->data[vec->len++] = item;\
    return (result_t){\
        .is_ok = true,\
        .payload = { .ok = NULL }\
    };\
}\
void destroy_##name(name##_t* vec) {\
    for (size_t i = 0; i < vec->len; i++)\
        (item_destroy)(vec->data + i);\
    free(vec->data);\
    vec->data = NULL;\
    vec->len = vec->cap = 0;\
}
#endif

typedef struct span {
    size_t start;
    size_t end;
} span_t;

typedef enum op {
    OP_PLUS, OP_MINUS, OP_STAR, OP_SLASH, OP_MODULO
} operator_t;

typedef struct result {
    bool is_ok;
    union {
        void* ok;
        char* err;
    } payload;
} result_t;

#define SIMPLE_ERR(msg) return (result_t){\
    .is_ok = false,\
    .payload = { .err = strdup(msg) }\
}

#define NONE (result_t){\
    .is_ok = false,\
    .payload = { .err = NULL }\
}

char* op_to_str(operator_t* op);
void empty_destroy(void* _);

#endif

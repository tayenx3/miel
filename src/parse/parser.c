#include "parser.h"
#include "../lex/token.h"
#include "../common.h"

parser_t create_parser(tok_stream_t* toks) {
    return (parser_t){
        .toks = toks,
        .pos = 0
    };
}

void destroy_parser(parser_t* parser) {
    destroy_tok_stream(parser->toks);
    free(parser->toks);
    parser->pos = 0;
}

static inline void advance(parser_t* parser) {
    parser->pos++;
}

static inline result_t peek(parser_t* parser) {
    if (parser->pos < parser->toks->len) {
        return (result_t){
            .is_ok = true,
            .payload = {
                .ok = parser->toks->data + parser->pos
            }
        };
    } else {
        return NONE;
    }
}

result_t parse(parser_t* parser) {
    return (result_t){
        .is_ok = true,
    };
}

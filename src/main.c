#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#include <errno.h>
#include "result.h"

#define STREQ(a, b) !strcmp(a, b)

result_t read_file(const char* path) {
    FILE* f = fopen(path, "rb");
    if (!f)
        switch (errno) {
            case 2:  SIMPLE_ERR("Cannot find file");
            case 13: SIMPLE_ERR("Cannot read file (permission denied)");
            case 24: SIMPLE_ERR("Too many open files");
            case 28: SIMPLE_ERR("No space on device");
            case 21: SIMPLE_ERR("Path is a directory");
            default: SIMPLE_ERR(strerror(errno));
        }

    if (fseek(f, 0, SEEK_END)) {
        fclose(f);
        SIMPLE_ERR(strerror(errno));
    }
    long fsize = ftell(f);
    if (fsize == -1L) {
        fclose(f);
        SIMPLE_ERR(strerror(errno));
    }
    if (fseek(f, 0, SEEK_SET)) {
        fclose(f);
        SIMPLE_ERR(strerror(errno));
    }

    char* buf = malloc((size_t)fsize + 1);
    if (!buf) {
        fclose(f);
        if (errno == ENOMEM) SIMPLE_ERR("Out of memory");
        else SIMPLE_ERR(strerror(errno));
    }

    size_t rsize = fread(buf, sizeof(char), fsize, f);
    if (rsize != (size_t)fsize) {
        fclose(f);
        free(buf);
        SIMPLE_ERR("File read failed");
    }

    buf[fsize] = '\0';
    fclose(f);
    return (result_t){
        .is_ok = true,
        .payload = { .ok = buf }
    };
}

typedef struct {
    char* input, output;
} command_t;

void destroy_command(command_t* cmd) {
    free(cmd->input);
    free(cmd->output);
    cmd->input = cmd->output = NULL;
}

result_t parse_command(int argc, char** argv) {
    command_t* result = calloc(1, sizeof(command_t));
    if (!result)
        if (errno == ENOMEM) SIMPLE_ERR("Out of memory");
        else SIMPLE_ERR(strerror(errno));

    int curr_arg = 0;
    while (curr_arg < argc) {
        char* arg = argv[curr_arg++];
        if (STREQ(arg, "--output") || STREQ(arg, "-o")) {
            if (curr_arg < argc) {
                if (result->output) {
                    destroy_command(result);
                    free(result);
                    SIMPLE_ERR("Output path already specified");
                }
                result->output = strdup(argv[curr_arg++]);
            } else {
                destroy_command(result);
                free(result);
                size_t needed = strlen(arg) + strlen("Expected output path after `` argument") + 1;
                char* buf = malloc(needed);
                if (buf) snprintf(buf, needed, "Expected output path after `%s` argument", arg);
                return (result_t){
                    .is_ok = false,
                    .payload = { .err = buf }
                };
            }
        } else {
            if (result->input) {
                destroy_command(result);
                free(result);
                size_t needed = strlen(arg) + strlen("Invalid argument ``") + 1;
                char* buf = malloc(needed);
                if (buf) snprintf(buf, needed, "Invalid argument `%s`", arg);
                return (result_t){
                    .is_ok = false,
                    .payload = { .err = buf }
                };
            }
            result->input = strdup(arg);
        }
    }

    return (result_t){
        .is_ok = true,
        .payload = { .ok = result },
    };
}

int main(int argc, char** argv) {
    result_t parse_result = parse_command(argc - 1, argv + 1);
    
    if (!parse_result.is_ok) {
        char* msg = parse_result.payload.err;
        if (msg) fprintf(stderr, "\x1b[91;1merror:\x1b[0m %s\n", msg);
        else fprintf(stderr, "\x1b[91;1merror:\x1b[0m Unknown error\n");
        free(msg);
        return 1;
    }

    command_t* cmd = parse_result.payload.ok;

    if (cmd->input) printf("input: %s\n", cmd->input);
    else {
        fprintf(stderr, "\x1b[91;1merror:\x1b[0m Expected input path\n");
        return 1;
    }
    if (cmd->output) printf("output: %s\n", cmd->output);

    result_t fread_result = read_file(cmd->input);
    if (!fread_result.is_ok) {
        fprintf(stderr, "\x1b[91;1merror:\x1b[0m %s\n", fread_result.payload.err);
        free(fread_result.payload.err);
        return 1;
    }

    char* contents = fread_result.payload.ok;
    printf("contents:\n%s\n", contents);
    free(contents);

    destroy_command(cmd);
    free(cmd);

    return 0;
}

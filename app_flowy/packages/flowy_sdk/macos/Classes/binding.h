#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>


int64_t init_sdk(char *path);

void async_command(int64_t port, const uint8_t *input, uintptr_t len);

void link_me_please(void);
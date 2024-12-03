#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 * The ewts_src should be a valid pointer to the string
 */
const char *ewts_to_unicode(const char *ewts_src);

/**
 * # Safety
 * The ptr should be a string returned from convert function
 */
void free_string(const char *ptr);

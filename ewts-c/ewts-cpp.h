#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

///
/// Creates `EwtsConverter` and returns its pointer
///
uintptr_t create_ewts_converter();

///
/// # Safety
/// Frees `EwtsConverter`.
/// Gets pointer returned fron `create_ewts_converter()` fn
///
void free_ewts_converter(uintptr_t ewts_converter_ptr);

///
/// # Panics
///
/// Panics if arguments are wrong
///
/// # Safety
/// The `ewts_converter_ptr` should be pointer returned from `create_ewts_converter()` fn.
/// And `ewts_src` should be a valid pointer to the string
///
const char *ewts_to_unicode(uintptr_t ewts_converter_ptr, const char *ewts_src);

/// # Safety
/// The ptr should be a pointer to the string returned from convert function
void free_ewts_string(const char *ptr);

}  // extern "C"

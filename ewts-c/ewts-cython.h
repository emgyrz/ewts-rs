from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  #
  # Creates `EwtsConverter` and returns its pointer
  #
  uintptr_t create_ewts_converter();

  #
  # Frees `EwtsConverter`.
  # Gets pointer to EwtsConverter instance
  #
  # # Safety
  # The `ewts_converter_ptr` should be pointer returned from `create_ewts_converter()` fn.
  #
  void free_ewts_converter(uintptr_t ewts_converter_ptr);

  #
  # Converts EWTS-string to Tibetan unicode string.
  #
  # # Example
  # ```cpp
  # // some C++ file
  # uintptr_t converter_ptr = create_ewts_converter();
  # const char * converted_str = ewts_to_unicode(converter_ptr, "rgyu ");
  # // "རྒྱུ་"
  # ```
  #
  # # Safety
  # The `ewts_converter_ptr` should be pointer returned from `create_ewts_converter()` fn.
  # And `ewts_src` should be a valid pointer to the string
  #
  const char *ewts_to_unicode(uintptr_t ewts_converter_ptr, const char *ewts_src);

  # As a precaution
  #
  # # Safety
  # The ptr should be a pointer to the string returned from convert function
  void free_ewts_string(const char *ptr);

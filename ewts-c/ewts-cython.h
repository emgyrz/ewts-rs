from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  # # Safety
  # The ewts_src should be a valid pointer to the string
  const char *ewts_to_unicode(const char *ewts_src);

  # # Safety
  # The ptr should be a string returned from convert function
  void free_string(const char *ptr);

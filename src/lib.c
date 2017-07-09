#include <stdio.h>
#include <errno.h>

// C function that calls `vsnprintf` with a POINTER to a va_list.
//
// We have to write this in C because there is no way to know
// the size of a va_list in Rust and so we couldn't pass it
// by-value as required by vsnprintf.
int vsnprintf_wrapper(char *buffer,
                      size_t size,
                      const char *format,
                      va_list *arg) {
  // C does not require vsprintf to set errno, but POSIX does.
  // Here we clear the errno and so we know that if this function
  // fails AND there is an error set, then it must have been triggered
  // by the sprintf.
  errno = 0;
  return vsnprintf(buffer, size, format, *arg);
}


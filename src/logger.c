#include <libinput.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

typedef void (*LogCallback)(enum libinput_log_priority, const char *);

static LogCallback global_callback = NULL;

// This is our main handler function that libinput will call.
// It receives the variable arguments (va_list) from libinput and converts them
// into a simple string that we can pass to Rust.
void colpetto_inner_log_handler(struct libinput *libinput,
                                enum libinput_log_priority priority,
                                const char *format, va_list args) {

  // If no callback is set, we can't do anything
  if (!global_callback)
    return;

  // vsnprintf with NULL buffer returns the required size.
  va_list args_copy;
  va_copy(args_copy, args);
  int size = vsnprintf(NULL, 0, format, args_copy);
  va_end(args_copy);

  // Handle error case from vsnprintf
  if (size < 0)
    return;

  char *buffer = malloc(size + 1);
  if (!buffer)
    return;

  // Actually format the string into our buffer
  vsnprintf(buffer, size + 1, format, args);

  global_callback(priority, buffer);

  free(buffer);
}

extern void colpetto_inner_set_log_callback(LogCallback callback) {
  global_callback = callback;
}

extern libinput_log_handler colpetto_inner_get_log_handler() {
  return colpetto_inner_log_handler;
}
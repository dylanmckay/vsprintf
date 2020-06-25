//! Convert C format strings to Rust.

extern crate libc;

use libc::size_t;
use std::iter::repeat;
use std::io::Error;
use std::os::raw::*;

const INITIAL_BUFFER_SIZE: usize = 512;

/// The result of a vsprintf call.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Prints a format string into a Rust string.
pub unsafe fn vsprintf<V>(format: *const c_char,
                          va_list: *mut V) -> Result<String> {
    vsprintf_raw(format, va_list).map(|bytes| {
        String::from_utf8(bytes).expect("vsprintf result is not valid utf-8")
    })
}

/// Prints a format string into a list of raw bytes that form
/// a null-terminated C string.
pub unsafe fn vsprintf_raw<V>(format: *const c_char,
                              va_list: *mut V) -> Result<Vec<u8>> {
    let list_ptr = va_list  as *mut c_void;

    let mut buffer = Vec::new();
    buffer.extend([0u8; INITIAL_BUFFER_SIZE].iter().cloned());

    loop {
        let character_count = vsnprintf_wrapper(
            buffer.as_mut_ptr(), buffer.len(), format, list_ptr
        );

        // Check for errors.
        if character_count == -1 {
            // C does not require vsprintf to set errno, but POSIX does.
            //
            // Default handling will just generate an 'unknown' IO error
            // if no errno is set.
            return Err(Error::last_os_error());
        } else {
            assert!(character_count >= 0);
            let character_count = character_count as usize;

            let current_max = buffer.len() - 1;

            // Check if we had enough room in the buffer to fit everything.
            if character_count > current_max {
                let extra_space_required = character_count - current_max;

                // Reserve enough space and try again.
                buffer.extend(repeat(0).take(extra_space_required as usize));
                continue;
            } else { // We fit everything into the buffer.
                // Truncate the buffer up until the null terminator.
                buffer = buffer.into_iter()
                               .take_while(|&b| b != 0)
                               .collect();
                break;
            }
        }
    }

    Ok(buffer)
}

extern {
    fn vsnprintf_wrapper(buffer: *mut u8,
                         size: size_t,
                         format: *const c_char,
                         va_list: *mut c_void) -> libc::c_int;
}

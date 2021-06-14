//! Convert C format strings to Rust.

extern crate libc;

use libc::size_t;
use std::convert::TryFrom;
use std::io::Error;
use std::os::raw::*;

/// The result of a vsprintf call.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Prints a format string into a Rust string.
pub unsafe fn vsprintf<V>(format: *const c_char, va_list: *mut V) -> Result<String> {
    vsprintf_raw(format, va_list)
        .map(|bytes| String::from_utf8(bytes).expect("vsprintf result is not valid utf-8"))
}

/// Return a buffer with the given length, filled with arbitrary data.
fn uninitd_buffer_of_len(len: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(len);
    // SAFETY: Any bit pattern is a valid u8.
    unsafe { buffer.set_len(len) };
    buffer
}

/// Prints a format string into a list of raw bytes that form
/// a null-terminated C string.
pub unsafe fn vsprintf_raw<V>(format: *const c_char, va_list: *mut V) -> Result<Vec<u8>> {
    let list_ptr = va_list as *mut c_void;

    let character_count = vsnprintf_wrapper(std::ptr::null_mut(), 0, format, list_ptr);

    // Check for errors.
    if character_count == -1 {
        // C does not require vsprintf to set errno, but POSIX does.
        //
        // Default handling will just generate an 'unknown' IO error
        // if no errno is set.
        return Err(Error::last_os_error());
    }

    // Include space for NULL byte.
    let mut buffer = uninitd_buffer_of_len(usize::try_from(character_count).unwrap() + 1);

    let final_character_count =
        vsnprintf_wrapper(buffer.as_mut_ptr(), buffer.len(), format, list_ptr);

    assert_eq!(final_character_count, character_count);

    // Drop null byte
    assert_eq!(buffer.pop(), Some(0u8));

    Ok(buffer)
}

extern "C" {
    fn vsnprintf_wrapper(
        buffer: *mut u8,
        size: size_t,
        format: *const c_char,
        va_list: *mut c_void,
    ) -> libc::c_int;
}

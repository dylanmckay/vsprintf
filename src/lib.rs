//! Convert C format strings to Rust.

extern crate libc;

use libc::size_t;
use std::io::Error;
use std::os::raw::*;

/// The result of a vsprintf call.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Prints a format string into a Rust string.
pub unsafe fn vsprintf<V>(format: *const c_char, va_list: *mut V) -> Result<String> {
    vsprintf_raw(format, va_list)
        .map(|bytes| String::from_utf8(bytes).expect("vsprintf result is not valid utf-8"))
}

/// Prints a format string into a list of raw bytes that form
/// a null-terminated C string.
pub unsafe fn vsprintf_raw<V>(format: *const c_char, va_list: *mut V) -> Result<Vec<u8>> {
    let list_ptr = va_list as *mut c_void;

    let mut buffer: Vec<u8> = Vec::new();
    loop {
        let rv = vsnprintf_wrapper(buffer.as_mut_ptr(), buffer.len(), format, list_ptr);

        // Check for errors.
        let character_count = if rv < 0 {
            // C does not require vsprintf to set errno, but POSIX does.
            //
            // Default handling will just generate an 'unknown' IO error
            // if no errno is set.
            return Err(Error::last_os_error());
        } else {
            rv as usize
        };

        if character_count >= buffer.len() {
            let new_len = character_count + 1;
            buffer.reserve_exact(new_len - buffer.len());
            // SAFETY: Any bit pattern is a valid u8, and we reserved the space.
            buffer.set_len(new_len);
            continue;
        }

        // Drop NULL byte and any excess capacity.
        buffer.truncate(character_count);
        break;
    }

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

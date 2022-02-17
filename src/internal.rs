use std::ffi::CString;
use std::os::raw::c_char;
use std::panic::{self, PanicInfo};
use std::ptr;

use crate::debugln;

/// Copies up to 256 bytes (including null termination) to
/// the provided destination. If the provided source string is too long, it will be
/// truncated.
pub unsafe fn copy_to_c_buffer(mut src: String, dest: *mut c_char) {
    // Truncate to 255 bytes (256 including the null terminator)
    src.truncate(255);
    let src_c = CString::new(src).unwrap_or_else(|_| CString::new("<invalid>").unwrap());
    let src_c_length = src_c.to_bytes_with_nul().len();
    debug_assert!(src_c_length <= 256);
    ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length);
}

/// Performs initialization required for the XPLM crate to work correctly
pub fn xplm_init() {
    panic::set_hook(Box::new(panic_hook));
    super::paths::path_init();
}

fn panic_hook(panic_info: &PanicInfo) {
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        debugln!("panic occurred: {:?}", s);
    } else {
        debugln!("panic occurred");
    }

    if let Some(location) = panic_info.location() {
        debugln!(
            "panic occurred in file '{}' at line {}",
            location.file(),
            location.line(),
        );
    } else {
        debugln!("panic occurred but can't get location information...");
    }
}

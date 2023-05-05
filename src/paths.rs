
use ffi::StringBuffer;
use xplm_sys::XPLMGetSystemPath;
//use std::ffi::{CString, NulError};
//use std::string::FromUtf8Error;

use super::feature;

/// Enables native paths
pub fn path_init() {
    // Feature specified to exist in SDK 2.1
    let native_path_feature =
        feature::find_feature("XPLM_USE_NATIVE_PATHS").expect("No native paths feature");
    native_path_feature.set_enabled(true);
}


pub fn xplane_folder() -> String{
    const BUFF_LEN: usize = 1024;
    let mut buffer = StringBuffer::new(BUFF_LEN);

    unsafe{ XPLMGetSystemPath(buffer.as_mut_ptr()); }

    let value_string = buffer.into_string().unwrap();

    value_string
}


pub fn plugins_folder() -> String{
    xplane_folder() + "Resources/plugins/"
}
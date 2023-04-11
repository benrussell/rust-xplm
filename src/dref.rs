
use std::ffi::{CString};
//use std::marker::PhantomData;
//use std::os::raw::c_void;
//use std::ptr;
use xplm_sys::*;

use super::data::borrowed::{FindError};



pub struct DrefHandle{
    raw_handle: XPLMDataRef,
}

impl DrefHandle{

    pub fn get_float( &self ) -> f32{
        unsafe { XPLMGetDataf(self.raw_handle) }
    }

    pub fn get_int( &self ) -> i32{
        unsafe { XPLMGetDatai(self.raw_handle) }
    }

}



pub fn find( name: &str ) -> Result<DrefHandle, FindError>{
    let name_c = CString::new(name)?; //FIXME: error handling
        
    let dataref = unsafe { XPLMFindDataRef(name_c.as_ptr()) };
    if dataref.is_null() {
        return Err(FindError::NotFound);
    }

    let dh = DrefHandle{
                       raw_handle: dataref,
                        };

    Ok( dh )
    
}
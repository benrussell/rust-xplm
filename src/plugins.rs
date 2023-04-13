//use crate::debugln;

use xplm_sys;

use ffi::StringBuffer;
use std::ffi::{CString, NulError};
//use std::string::FromUtf8Error;


/// Module to allow querying other plugins loaded in X-Plane session.

pub fn get_my_id() -> xplm_sys::XPLMPluginID{
    unsafe { xplm_sys::XPLMGetMyID() }
}

pub fn count() -> i32{
    unsafe { xplm_sys::XPLMCountPlugins() }
}

pub fn get_nth( index: i32 ) -> Option<xplm_sys::XPLMPluginID>{
    let ret: xplm_sys::XPLMPluginID = unsafe { xplm_sys::XPLMGetNthPlugin(index) };

    if ret == xplm_sys::XPLM_NO_PLUGIN_ID{
        None
    }else{
        Some( ret )
    }
}

pub fn find_by_path( path: &str ) -> Option<xplm_sys::XPLMPluginID>{
    let path_c = std::ffi::CString::new(path).unwrap(); //FIXME: unwrap!
    let ret: xplm_sys::XPLMPluginID = unsafe { xplm_sys::XPLMFindPluginByPath(path_c.as_ptr()) };

    if ret == xplm_sys::XPLM_NO_PLUGIN_ID{
        None
    }else{
        Some( ret )
    }
}

pub fn find_by_signature( signature: &str ) -> Option<xplm_sys::XPLMPluginID>{
    let signature_c = std::ffi::CString::new(signature).unwrap(); //FIXME: unwrap!
    let ret: xplm_sys::XPLMPluginID = unsafe { xplm_sys::XPLMFindPluginBySignature(signature_c.as_ptr()) };

    if ret == xplm_sys::XPLM_NO_PLUGIN_ID{
        None
    }else{
        Some( ret )
    }
}



//#[allow(unused_variables)]
pub struct Info{
    plugin_id: xplm_sys::XPLMPluginID,
    name: String,
    path: String,
    signature: String,
    description: String,
}


impl Info{    
    pub fn plugin_id(&self) -> &xplm_sys::XPLMPluginID{
        &self.plugin_id
    }

    pub fn name(&self) -> &str{
        &self.name
    }

    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn signature(&self) -> &str{
        &self.signature
    }

    pub fn description(&self) -> &str{
        &self.description
    }
}


pub fn get_info( plugin_id: xplm_sys::XPLMPluginID ) -> Info{
    
    const STORAGE: usize = 1024;    
    let mut name_buff = StringBuffer::new(STORAGE);
    let mut path_buff = StringBuffer::new(STORAGE);
    let mut sig_buff = StringBuffer::new(STORAGE);
    let mut desc_buff = StringBuffer::new(STORAGE);
    
    unsafe { 
        xplm_sys::XPLMGetPluginInfo(
            plugin_id, 
            name_buff.as_mut_ptr(),      //out name
            path_buff.as_mut_ptr(),      //out file path
            sig_buff.as_mut_ptr(),       //out signature
            desc_buff.as_mut_ptr()      //out description
        ) 
    }

    //FIXME: Improve this error handling..
    let name: String = name_buff.into_string().unwrap_or("UTF-8 error.".to_string());
    let path: String = path_buff.into_string().unwrap_or("UTF-8 error.".to_string());
    let sig: String = sig_buff.into_string().unwrap_or("UTF-8 error.".to_string());
    let desc: String = desc_buff.into_string().unwrap_or("UTF-8 error.".to_string());

    Info { 
        plugin_id: plugin_id,
        name: name, 
        path: path, 
        signature: sig, 
        description: desc, 
    }

}



pub fn is_enabled( plugin_id: xplm_sys::XPLMPluginID ) -> bool{
    unsafe { xplm_sys::XPLMIsPluginEnabled(plugin_id) != 0 }
}

pub fn enable( plugin_id: xplm_sys::XPLMPluginID ) -> bool{
    unsafe { xplm_sys::XPLMEnablePlugin(plugin_id) != 0 }    
}

pub fn disable( plugin_id: xplm_sys::XPLMPluginID ){
    unsafe { xplm_sys::XPLMDisablePlugin(plugin_id) }
}

pub fn reload(){
    unsafe { xplm_sys::XPLMReloadPlugins() }
}


pub fn send_message( plugin_id: xplm_sys::XPLMPluginID, message: i32, param: *mut std::os::raw::c_void ){
    unsafe{
        xplm_sys::XPLMSendMessageToPlugin( plugin_id, message, param );
    }
}


pub fn send_string( plugin_id: xplm_sys::XPLMPluginID, message: i32, value: &str ) -> Result<(), NulError> {
    let value_c = CString::new(value)?;

    unsafe{
        xplm_sys::XPLMSendMessageToPlugin( plugin_id, message, value_c.as_bytes_with_nul().as_ptr() as *mut std::os::raw::c_void );
    }

    Ok(())
}
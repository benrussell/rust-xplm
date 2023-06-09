
use ffi::StringBuffer;
use xplm_sys::{XPLMGetSystemPath, XPLMGetNthAircraftModel};
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



pub fn player_aircraft_folder() -> String{
    let acf_path = AircraftPath::from_id(0);
    // crate::debugln!("rust-xplm: player_aircraft_folder: {:?}", acf_path);

    acf_path.folder()
}



#[derive(Debug)]
pub struct AircraftPath{
    filename: String,
    folder: String,
}

impl AircraftPath{
    pub fn from_id( id: i32 ) -> Self{

        // https://developer.x-plane.com/sdk/XPLMGetNthAircraftModel/

        let mut buffer_filename = StringBuffer::new(256);
        let mut buffer_folder = StringBuffer::new(512);

        unsafe{
            XPLMGetNthAircraftModel(
                id,
                buffer_filename.as_mut_ptr(),
                buffer_folder.as_mut_ptr(),
            );
        }

        // X-Plane includes the filename in the folder data returned so we need
        // to remove it.

        let filename = buffer_filename.into_string().unwrap();
        let folder = buffer_folder.into_string().unwrap();
        let folder = folder.replace(&filename, "");

        AircraftPath { 
            filename: filename, 
            folder: folder, 
        }
    }


    pub fn filename(&self) -> String{
        self.filename.clone()
    }


    pub fn folder(&self) -> String{
        self.folder.clone()
    }


}
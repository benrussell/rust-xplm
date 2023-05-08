//use crate::debugln;

use crate::data;
use crate::data::borrowed; //::{DataRef, FindError};
use crate::data::{DataRead, StringRead};
//use crate::data::{ArrayRead, DataRead, ReadOnly, ReadWrite, StringRead};

enum XPlaneMessage{
    PlaneCrashed,
    PlaneLoaded,
    AirportLoaded,
    SceneryLoaded,
    AirplaneCountChanged,
    PlaneUnloaded,
    WillWritePrefs,
    LiveryLoaded,
    EnteredVR,
    ExitingVR,
    ReleasePlanes,
    FmodBankLoaded,
    FmodBankUnloading,
    DatarefsAdded,    
}

impl XPlaneMessage{
    fn from_xplm( xplm_msg: i32 ) -> Option<Self>{
        match xplm_msg as u32{ //FIXME: SDK bindgen is giving us u32 but function sig for RX messages gives us i32
            xplm_sys::XPLM_MSG_PLANE_CRASHED => Some(XPlaneMessage::PlaneCrashed),
            xplm_sys::XPLM_MSG_PLANE_LOADED => Some(XPlaneMessage::PlaneLoaded),
            xplm_sys::XPLM_MSG_AIRPORT_LOADED => Some(XPlaneMessage::AirportLoaded),
            xplm_sys::XPLM_MSG_SCENERY_LOADED => Some(XPlaneMessage::SceneryLoaded),
            xplm_sys::XPLM_MSG_AIRPLANE_COUNT_CHANGED => Some(XPlaneMessage::AirplaneCountChanged),
            xplm_sys::XPLM_MSG_PLANE_UNLOADED => Some(XPlaneMessage::PlaneUnloaded),
            xplm_sys::XPLM_MSG_WILL_WRITE_PREFS => Some(XPlaneMessage::WillWritePrefs),
            xplm_sys::XPLM_MSG_LIVERY_LOADED => Some(XPlaneMessage::LiveryLoaded),
            xplm_sys::XPLM_MSG_ENTERED_VR => Some(XPlaneMessage::EnteredVR),
            xplm_sys::XPLM_MSG_EXITING_VR => Some(XPlaneMessage::ExitingVR),
            xplm_sys::XPLM_MSG_RELEASE_PLANES => Some(XPlaneMessage::ReleasePlanes),
            xplm_sys::XPLM_MSG_FMOD_BANK_LOADED => Some(XPlaneMessage::FmodBankLoaded),
            xplm_sys::XPLM_MSG_FMOD_BANK_UNLOADING => Some(XPlaneMessage::FmodBankUnloading),
            xplm_sys::XPLM_MSG_DATAREFS_ADDED => Some(XPlaneMessage::DatarefsAdded),
            _ => None,
        }
    }
}


pub trait XPlaneMessageFilter: crate::plugin::Plugin {

    #[allow(unused_variables)]        
    fn rx_message(&mut self,
        from: xplm_sys::XPLMPluginID, //always from X-Plane
        message: i32, // used to filted into sub functions
        param: *mut ::std::os::raw::c_void){

        //debugln!("trait MessageCallback rx xplane message");

        match XPlaneMessage::from_xplm(message){
            Some(XPlaneMessage::PlaneCrashed) => self.msg_plane_crashed(),
            Some(XPlaneMessage::PlaneLoaded) => self.msg_plane_loaded(param as u32),
            Some(XPlaneMessage::AirportLoaded) => self.msg_airport_loaded(),
            Some(XPlaneMessage::SceneryLoaded) => self.msg_scenery_loaded(),
            Some(XPlaneMessage::AirplaneCountChanged) => self.msg_airplane_count_changed(),
            Some(XPlaneMessage::PlaneUnloaded) => self.msg_plane_unloaded(param as u32),
            Some(XPlaneMessage::WillWritePrefs) => self.msg_will_write_prefs(),
            Some(XPlaneMessage::LiveryLoaded) => self.prepare_msg_livery_loaded(param as u32),
            Some(XPlaneMessage::EnteredVR) => self.msg_entered_vr(),
            Some(XPlaneMessage::ExitingVR) => self.msg_exiting_vr(),
            Some(XPlaneMessage::ReleasePlanes) => self.msg_release_planes(),
            Some(XPlaneMessage::FmodBankLoaded) => self.msg_fmod_bank_loaded(param as u32),
            Some(XPlaneMessage::FmodBankUnloading) => self.msg_fmod_bank_unloading(param as u32),
            Some(XPlaneMessage::DatarefsAdded) => self.msg_datarefs_added(param as u32),
            None => (),
        }

    }


    // https://developer.x-plane.com/sdk/XPLMPlugin/#XPLM_MSG_PLANE_CRASHED
    
    fn msg_plane_crashed(&mut self); // param ignored
    

    #[allow(unused_variables)]
    fn msg_plane_loaded(&mut self, param: u32){
        // param is index of plane being loaded
    }

    fn msg_airport_loaded(&mut self); // param ignored

    // Use datarefs to determine the new scenery files that were loaded.
    fn msg_scenery_loaded(&mut self); // param ignored

    // This message is sent whenever the user adjusts the number of X-Plane aircraft models. 
    // You must use XPLMCountPlanes to find out how many planes are now available. 
    // This message will only be sent in XP7 and higher because in XP6 the number of aircraft is not user-adjustable.         
    fn msg_airplane_count_changed(&mut self); // param ignored

    // The parameter contains the index number of the plane being unloaded; 0 indicates the user’s plane. 
    // The parameter is of type int, passed as the value of the pointer. 
    // (That is: the parameter is an int, not a pointer to an int.)
    #[allow(unused_variables)]
    fn msg_plane_unloaded(&mut self, param: u32 ){
        // param is int value        
    }

    fn msg_will_write_prefs(&mut self); // param ignored

    // This function retrieves the livery data as strings from the X-Plane datarefs.
    #[allow(unused_variables)]
    fn prepare_msg_livery_loaded(&mut self, param: u32){
        // This message is sent to your plugin right after a livery is loaded for an airplane. 
        // You can use this to check the new livery (via datarefs) and react accordingly. 
        // The parameter contains the index number of the aircraft whose livery is changing.

            //sim/aircraft/view/acf_livery_index int
            //sim/aircraft/view/acf_livery_path byte[1024]

            let dref_livery_index: borrowed::DataRef<u32, data::ReadOnly> = borrowed::DataRef::find("sim/aircraft/view/acf_livery_index").expect("bad dref: livery index");            
            let dref_livery_path: borrowed::DataRef<[u8], data::ReadOnly> = borrowed::DataRef::find("sim/aircraft/view/acf_livery_path").expect("bad dref: livery path");

            let livery_index: u32 = dref_livery_index.get();
            let livery_path: String = dref_livery_path.get_as_string().unwrap_or(String::from("Rust Error: bad livery data."));

        self.msg_livery_loaded( param, livery_index, &livery_path );
    }

    // This is called by prepare_msg_livery_loaded(...)
    fn msg_livery_loaded(&mut self, param: u32, livery_index: u32, livery_path: &str);

    fn msg_entered_vr(&mut self); // param ignored

    fn msg_exiting_vr(&mut self); // param ignored

    fn msg_release_planes(&mut self); // param ignored
        //multiplayer interop function..
        
    #[allow(unused_variables)]
    fn msg_fmod_bank_loaded(&mut self, param: u32){
        // The parameter is the XPLMBankID enum in XPLMSound.h, 0 for the master bank and 1 for the radio bank.
    }

    #[allow(unused_variables)]
    fn msg_fmod_bank_unloading(&mut self, param: u32){
        // The parameter is the XPLMBankID enum in XPLMSound.h, 0 for the master bank and 1 for the radio bank.
    }

    #[allow(unused_variables)]
    fn msg_datarefs_added(&mut self, param: u32){
        /*
        Sent to your plugin per-frame (at-most) when/if datarefs are added. It will include the new data ref total count so that your plugin can keep a local cache of the total, see what’s changed and know which ones to inquire about if it cares.
        This message is only sent to plugins that enable the XPLM_WANTS_DATAREF_NOTIFICATIONS feature.
        */
    }


}




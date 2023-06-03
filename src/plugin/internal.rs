use std::os::raw::{c_char, c_int};
use std::panic;
use std::panic::AssertUnwindSafe;
use std::ptr; //::{self, addr_of};

use super::super::debugln;
use super::super::internal::copy_to_c_buffer;

use super::Plugin;

/// Information on a plugin
pub struct PluginData<P> {
    /// A pointer to the plugin, allocated in a Box
    pub plugin: *mut P,
    /// If the plugin has panicked in any XPLM callback
    ///
    /// Plugins that have panicked will not receive any further
    /// XPLM callbacks.
    pub panicked: bool,
}

/// Implements the XPluginStart callback
///
/// This reduces the amount of code in the xplane_plugin! macro.
///
/// data is a reference to a PluginData object where the created plugin will be stored.
/// The other parameters are the same as for XPluginStart.
///
/// This function tries to create and allocate a plugin. On success, it stores a pointer to the
/// plugin in data.plugin and returns 1. If the plugin fails to start, it stores a null pointer
/// in data.plugin and returns 0.
///
/// This function never unwinds. It catches any unwind that may occur.
pub unsafe fn xplugin_start<P>(
    data: &mut PluginData<P>,
    name: *mut c_char,
    signature: *mut c_char,
    description: *mut c_char,
) -> c_int
where
    P: Plugin,
{
    let unwind = panic::catch_unwind(AssertUnwindSafe(|| {
        super::super::internal::xplm_init();

        let info = P::info();

        let fancy_name = format!("{} v{}", info.name, info.version);

        debugln!("{}", fancy_name);
        debugln!("{}", info.description);
        //debugln!("{}", info.signature);
        
        copy_to_c_buffer(fancy_name, name);
        copy_to_c_buffer(info.description.clone(), description);
        copy_to_c_buffer(info.signature.clone(), signature);

        let retval = match P::start( info ) {
            Ok(plugin) => {
                let plugin_box = Box::new(plugin);
                data.plugin = Box::into_raw(plugin_box);
                1
            }
            Err(e) => {
                debugln!("Plugin failed to start: {}", e);
                data.plugin = ptr::null_mut();
                0
            }
        };

        // Pass the raw plugin pointer into itself so we can interop with C APIs and callbacks??
        (*data.plugin).register_ptr( data.plugin as usize );


        return retval;

    }));
    unwind.unwrap_or_else(|_| {
        debugln!("Panic in XPluginStart");
        data.panicked = true;
        data.plugin = ptr::null_mut();
        0
    })
}

/// Implements the XPluginStop callback
///
/// This function never unwinds. It catches any unwind that may occur.
pub unsafe fn xplugin_stop<P>(data: &mut PluginData<P>)
where
    P: Plugin,
{
    if !data.panicked {
        let unwind = panic::catch_unwind(AssertUnwindSafe(|| {
            (*data.plugin).stop();
            let plugin = Box::from_raw(data.plugin);
            data.plugin = ptr::null_mut();
            drop(plugin);
        }));
        if unwind.is_err() {
            eprintln!("Panic in XPluginStop");
            data.panicked = true;
        }
    } else {
        debugln!("Warning: A plugin that panicked cannot be stopped. It may leak resources.");
    }
}

/// Implements the XPluginEnable callback
///
/// This function never unwinds. It catches any unwind that may occur.
pub unsafe fn xplugin_enable<P>(data: &mut PluginData<P>) -> c_int
where
    P: Plugin,
{
    if !data.panicked {
        let unwind = panic::catch_unwind(AssertUnwindSafe(|| match (*data.plugin).enable() {
            Ok(_) => 1,
            Err(e) => {
                debugln!("Plugin failed to enable: {}", e);
                0
            }
        }));
        unwind.unwrap_or_else(|_| {
            eprintln!("Panic in XPluginEnable");
            data.panicked = true;
            0
        })
    } else {
        // Can't enable a plugin that has panicked
        0
    }
}

/// Implements the XPluginDisable callback
///
/// This function never unwinds. It catches any unwind that may occur.
pub unsafe fn xplugin_disable<P>(data: &mut PluginData<P>)
where
    P: Plugin,
{
    if !data.panicked {
        let unwind = panic::catch_unwind(AssertUnwindSafe(|| {
            (*data.plugin).disable();
        }));
        if unwind.is_err() {
            eprintln!("Panic in XPluginDisable");
            data.panicked = true;
        }
    }
}

/// Implements the XPluginReceiveMessage callback
///
/// This function never unwinds. It catches any unwind that may occur.
pub unsafe fn xplugin_receive_message<P>(
    data: &mut PluginData<P>, 
    from: ::std::os::raw::c_int,
    message: ::std::os::raw::c_int,
    param: *mut ::std::os::raw::c_void
    )
where
    P: Plugin,
{
    if !data.panicked {
        let unwind = panic::catch_unwind(AssertUnwindSafe(|| {
            
            // let plugin = &mut (*data.plugin);
            
            // let from: xplm_sys::XPLMPluginID = from as xplm_sys::XPLMPluginID;
            // let message: i32 = message as i32;
            
            // plugin.rx_message(from, message, param);



            // debugln!("rust-xplm: rx_msg: data.plugin: {:?}", (*data).plugin);

            (*data.plugin).rx_message(from, message, param);

        }));
        if unwind.is_err() {
            eprintln!("Panic in XPluginReceiveMessage");
            data.panicked = true;
        }
    }
}

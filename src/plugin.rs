/// Accessing and communicating with other plugins
pub mod management;

/// Items used by the xplane_plugin! macro, which must be public
#[doc(hidden)]
pub mod internal;

/// Information about a plugin
pub struct PluginInfo {
    /// The plugin name
    pub name: String,
    /// The plugin's signature, in reverse DNS format
    pub signature: String,
    /// A description of the plugin
    pub description: String,
}

/// The trait that all plugins should implement
pub trait Plugin: Sized {
    /// The error type that a plugin may encounter when starting up or enabling
    type Error: std::fmt::Display;

    /// Called when X-Plane loads this plugin
    ///
    /// On success, returns a plugin object
    fn start() -> Result<Self, Self::Error>;

    /// Returns information on this plugin
    fn info(&self) -> PluginInfo;

    /// Called when the plugin is enabled
    ///
    /// If this function returns an Err, the plugin will remain disabled.
    ///
    /// The default implementation returns Ok(()).
    fn enable(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the plugin is disabled
    ///
    /// The default implementation does nothing.
    fn disable(&mut self) {}

    /// Called when the plugin is disabled
    ///
    /// The default implementation does nothing.
    fn stop(&mut self){}

    /// Called for messages from other plugins.
    ///
    /// The default implementation does nothing.
    #[allow(unused_variables)]        
    fn receive_message(&mut self,
        from: u32,
        message: u32,
        param: *mut ::std::os::raw::c_void){}
        
    /// Called for messages from X-Plane.
    ///
    /// The default implementation does nothing.
    #[allow(unused_variables)]        
    fn receive_xplane_message(&mut self,
        message: u32,
        param: *mut ::std::os::raw::c_void){} 


}

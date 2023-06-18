use std::any::Any;

use crate::high::{
    northstar::{EngineLoadType, PluginData, ScriptVmType},
    squirrel::CSquirrelVMHandle,
};

///! the plugin system will look for a exported function to pass stuff to
///!
///! this exported function and others are created by the `entry` macro
///!
///! it takes your plugin struct and calls specific function for each event

/// Trait for defining the callbacks and entry point of the plugin
///
/// also provides a thread to run code on (the main function)
///
/// it is unsafe to run any titanfall engine functions on it
pub trait Plugin: Any + Sync {
    type SaveType;

    fn new() -> Self;

    fn initialize(&mut self, plugin_data: &PluginData);

    fn main(&self);

    fn on_engine_load(&self, _engine: &EngineLoadType, _dll_ptr: *const std::ffi::c_void) {}

    fn on_sqvm_created(&self, _sqvm_handle: &CSquirrelVMHandle<Self::SaveType>) {}

    fn on_sqvm_destroyed(&self, _context: ScriptVmType) {}

    fn runframe(&self) {}
}

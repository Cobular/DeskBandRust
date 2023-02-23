use std::ffi::c_void;

use once_cell::sync::OnceCell;
use windows::{Win32::{Foundation::{HINSTANCE, BOOL}, System::{SystemServices::DLL_PROCESS_ATTACH, LibraryLoader::DisableThreadLibraryCalls}}, core::HRESULT};
use log::{debug, error, log_enabled, info, Level};

use crate::log_config::configure_logging;

mod log_config;
mod deskband;

static G_HISNTANCE: OnceCell<HINSTANCE> = OnceCell::new();

#[no_mangle]
extern "system" fn DllMain(module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    configure_logging().unwrap();


    if reason == DLL_PROCESS_ATTACH {
        info!("Process Attached");

        G_HISNTANCE.set(module).unwrap();
        unsafe { DisableThreadLibraryCalls(module)};
        return true.into();
    }

    return false.into();
}

#[no_mangle]
extern "system" fn DllRegisterServer()
{
    info!("Registering");
}

#[no_mangle]
extern "system" fn DllUnregisterServer()
{
    println!("Unregistering");
}
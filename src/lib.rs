use std::ffi::c_void;

use log::info;
use once_cell::sync::OnceCell;
use windows::{
    core::GUID,
    Win32::{
        Foundation::{BOOL, HINSTANCE},
        System::{LibraryLoader::DisableThreadLibraryCalls, SystemServices::DLL_PROCESS_ATTACH},
    },
};

use crate::log_config::configure_logging;

mod deskband;
mod log_config;
mod classfactory;

const THING_GUID: GUID = GUID::from_u128(15872140175531212679355407191611801757);
static G_HISNTANCE: OnceCell<HINSTANCE> = OnceCell::new();

#[no_mangle]
extern "system" fn DllMain(module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    configure_logging().unwrap();

    if reason == DLL_PROCESS_ATTACH {
        info!("Process Attached");

        G_HISNTANCE.set(module).unwrap();
        unsafe { DisableThreadLibraryCalls(module) };

        let deskband = deskband::MyDeskBand::new();
        return true.into();
    }

    false.into()
}

#[no_mangle]
extern "system" fn DllGetClassObject(
    rclsid: GUID,
    riid: *const GUID,
    ppv: *mut c_void,
) -> BOOL {
    unsafe {
        if THING_GUID != rclsid {
            return false.into();
        }
    }

    let deskband = deskband::MyDeskBand::new();

    info!(
        "GetClassObject\n rclsid {:?}\n riid {:?}\n ppv {:?}",
        rclsid, riid, ppv
    );
    true.into()
}

#[no_mangle]
extern "system" fn DllCanUnloadNow() -> BOOL {
    info!("CanUnloadNow");
    true.into()
}

#[no_mangle]
extern "system" fn DllRegisterServer() {
    info!("Registering");
}

#[no_mangle]
extern "system" fn DllUnregisterServer() {
    println!("Unregistering");
}

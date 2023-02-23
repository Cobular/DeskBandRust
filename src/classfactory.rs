use log::info;
use windows::{
    core::GUID,
    Win32::{
        Foundation::{BOOL, E_NOTIMPL},
        System::Com::{IClassFactory, IClassFactory_Impl},
    },
};
use windows_implement::implement;

#[implement(IClassFactory)]
struct MyClassFactory {}

impl IClassFactory_Impl for MyClassFactory {
    fn CreateInstance(
        &self,
        punkouter: &core::option::Option<windows::core::IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows::core::Result<()> {
        info!(
            "CreateInstance\n punkouter {:?}\n riid {:?}\n ppvobject {:?}",
            punkouter, riid, ppvobject
        );
        Err(E_NOTIMPL.into())
    }

    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        info!("LockServer\n flock {:?}", flock);
        Err(E_NOTIMPL.into())
    }
}

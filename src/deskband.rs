use core::option::Option;

use log::info;
use windows::core::{IUnknown, Result, GUID};
use windows::Win32::Foundation::{HWND, BOOL, RECT};
use windows::Win32::System::Com::{IPersistStream_Impl, IPersist_Impl};
use windows::Win32::System::Ole::{IObjectWithSite_Impl, IOleClientSite, IOleWindow_Impl};
use windows::Win32::UI::Shell::{
    IDeskBand, IDeskBand2_Impl, IDeskBand_Impl, IDockingWindow_Impl, IInputObject, IInputObjectSite, DESKBANDINFO, IInputObject_Impl,
};

struct MyDeskBand {
    // The window handle of the deskband.
    hwnd: Option<HWND>,
    // The window handle of the parent window.
    parent_hwnd: Option<HWND>,
    // The ID of the deskband
    id: Option<u32>,
    // If glass is enabled
    composition_enabled: bool,
    // If the deskband setting has been changed
    dirty: bool,
    // If the deskband currently has focus
    has_focus: bool,
    // Parent site that contains the deskband
    parent_site: Option<IInputObjectSite>,
    // IObjectWithSite site
    site: Option<IUnknown>,
}

impl MyDeskBand {
    fn new(this_hwnd: HWND) -> Self {
        info!("Creating deskband");
        Self {
            hwnd: None,
            parent_hwnd: None,
            id: None,
            composition_enabled: false,
            dirty: false,
            has_focus: false,
            parent_site: None,
            site: None,
        }
    }

    
}

impl Drop for MyDeskBand {
    fn drop(&mut self) {
        info!("Dropping deskband");
    }
}

impl IDeskBand2_Impl for MyDeskBand {
    extern "C" fn CanRenderComposited(
        &self,
    ) -> Result<BOOL> {
        todo!()
    }

    extern "C" fn SetCompositionState(
        &self,
        fcompositionenabled: BOOL,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn GetCompositionState(
        &self,
    ) -> Result<BOOL> {
        todo!()
    }
}

impl IDeskBand_Impl for MyDeskBand {
    extern "C" fn GetBandInfo(
        &self,
        dwbandid: u32,
        dwviewmode: u32,
        pdbi: *mut DESKBANDINFO,
    ) -> Result<()> {
        todo!()
    }
}

impl IDockingWindow_Impl for MyDeskBand {
    extern "C" fn ShowDW(
        &self,
        fshow: BOOL,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn CloseDW(&self, dwreserved: u32) -> Result<()> {
        todo!()
    }

    extern "C" fn ResizeBorderDW(
        &self,
        prcborder: *const RECT,
        punktoolbarsite: &Option<IUnknown>,
        freserved: BOOL,
    ) -> Result<()> {
        todo!()
    }
}

impl IOleWindow_Impl for MyDeskBand {
    extern "C" fn GetWindow(
        &self,
        phwnd: *mut HWND,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn ContextSensitiveHelp(
        &self,
        fentermode: BOOL,
    ) -> Result<()> {
        todo!()
    }
}

impl IPersist_Impl for MyDeskBand {
    extern "C" fn GetClassID(
        &self,
        pclassid: *mut GUID,
    ) -> Result<()> {
        todo!()
    }
}

impl IPersistStream_Impl for MyDeskBand {
    extern "C" fn IsDirty(
        &self,
    ) -> Result<BOOL> {
        todo!()
    }

    extern "C" fn Load(
        &self,
        pstm: &Option<IUnknown>,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn Save(
        &self,
        pstm: &Option<IUnknown>,
        fcleardirty: BOOL,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn GetSizeMax(&self, pcb: *mut u64) -> Result<()> {
        todo!()
    }
}

impl IObjectWithSite_Impl for MyDeskBand {
    extern "C" fn SetSite(
        &self,
        punksite: &Option<IUnknown>,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn GetSite(
        &self,
        riid: *const GUID,
        ppvsite: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        todo!()
    }
}

impl IInputObject_Impl for MyDeskBand {
    extern "C" fn UIActivateIO(
        &self,
        factivate: BOOL,
        pmsg: &Option<IUnknown>,
    ) -> Result<()> {
        todo!()
    }

    extern "C" fn HasFocusIO(&self) -> Result<BOOL> {
        todo!()
    }

    extern "C" fn TranslateAcceleratorIO(
        &self,
        pmsg: &Option<IUnknown>,
    ) -> Result<()> {
        todo!()
    }
}
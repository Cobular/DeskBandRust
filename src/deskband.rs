use log::info;
use windows::core::{IUnknown, HRESULT};
use windows::Win32::Foundation::{BOOL, HWND, RECT, E_NOTIMPL};
use windows::Win32::System::Com::{IPersistStream, IPersistStream_Impl, IPersist_Impl};
use windows::Win32::System::Ole::{
    IObjectWithSite, IObjectWithSite_Impl, IOleWindow_Impl,
};
use windows::Win32::UI::Shell::{
    IDeskBand2, IDeskBand2_Impl, IDeskBand_Impl, IDockingWindow_Impl, IInputObject,
    IInputObjectSite, IInputObject_Impl, DESKBANDINFO,
};
use windows_implement::implement;

#[implement(IDeskBand2, IPersistStream, IObjectWithSite, IInputObject)]
pub struct MyDeskBand {
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
    pub fn new() -> Self {
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
        self.Release();
        info!("Dropping deskband");
    }
}

impl IPersistStream_Impl for MyDeskBand {
    fn IsDirty(&self) -> HRESULT {
        info!("IsDirty");
        E_NOTIMPL
    }

    fn Load(
        &self,
        pstm: &Option<windows::Win32::System::Com::IStream>,
    ) -> windows::core::Result<()> {
        info!("Load\n pstm {:?}", pstm);
        Err(E_NOTIMPL.into())
    }

    fn Save(
        &self,
        pstm: &::core::option::Option<windows::Win32::System::Com::IStream>,
        fcleardirty: BOOL,
    ) -> windows::core::Result<()> {
        info!("Save\n pstm {:?}\n fcleardirty {:?}", pstm, fcleardirty);
        Err(E_NOTIMPL.into())
    }

    fn GetSizeMax(&self) -> windows::core::Result<u64> {
        info!("GetSizeMax");
        Err(E_NOTIMPL.into())
    }
}

impl IPersist_Impl for MyDeskBand {
    fn GetClassID(&self) -> windows::core::Result<windows::core::GUID> {
        info!("GetClassID");
        Err(E_NOTIMPL.into())
    }
}

impl IInputObject_Impl for MyDeskBand {
    fn UIActivateIO(
        &self,
        factivate: BOOL,
        pmsg: *const windows::Win32::UI::WindowsAndMessaging::MSG,
    ) -> windows::core::Result<()> {
        info!("UIActivateIO\n factivate {:?}\n pmsg {:?}", factivate, pmsg);
        Err(E_NOTIMPL.into())
    }

    fn HasFocusIO(&self) -> windows::core::Result<()> {
        info!("HasFocusIO");
        Err(E_NOTIMPL.into())
    }

    fn TranslateAcceleratorIO(
        &self,
        pmsg: *const windows::Win32::UI::WindowsAndMessaging::MSG,
    ) -> windows::core::Result<()> {
        info!("TranslateAcceleratorIO\n pmsg {:?}", pmsg);
        Err(E_NOTIMPL.into())
    }
}

impl IObjectWithSite_Impl for MyDeskBand {
    fn SetSite(&self, punksite: &Option<windows::core::IUnknown>) -> windows::core::Result<()> {
        info!("SetSite\n punksite {:?}", punksite);
        Err(E_NOTIMPL.into())
    }

    fn GetSite(
        &self,
        riid: *const windows::core::GUID,
        ppvsite: *mut *mut ::std::ffi::c_void,
    ) -> windows::core::Result<()> {
        info!("GetSite\n riid {:?}\n ppvsite {:?}", riid, ppvsite);
        Err(E_NOTIMPL.into())
    }
}

impl IDeskBand2_Impl for MyDeskBand {
    fn CanRenderComposited(&self) -> windows::core::Result<BOOL> {
        info!("CanRenderComposited");
        Err(E_NOTIMPL.into())
    }

    fn SetCompositionState(&self, fcompositionenabled: BOOL) -> windows::core::Result<()> {
        info!("SetCompositionState\n fcompositionenabled {:?}", fcompositionenabled);
        Err(E_NOTIMPL.into())
    }

    fn GetCompositionState(&self) -> windows::core::Result<BOOL> {
        info!("GetCompositionState");
        Err(E_NOTIMPL.into())
    }
}

impl IDeskBand_Impl for MyDeskBand {
    fn GetBandInfo(
        &self,
        dwbandid: u32,
        dwviewmode: u32,
        pdbi: *mut DESKBANDINFO,
    ) -> windows::core::Result<()> {
        info!("GetBandInfo\n dwbandid {:?}\n dwviewmode {:?}\n pdbi {:?}", dwbandid, dwviewmode, pdbi);
        Err(E_NOTIMPL.into())
    }
}

impl IDockingWindow_Impl for MyDeskBand {
    fn ShowDW(&self, fshow: BOOL) -> windows::core::Result<()> {
        info!("ShowDW\n fshow {:?}", fshow);
        Err(E_NOTIMPL.into())
    }

    fn CloseDW(&self, dwreserved: u32) -> windows::core::Result<()> {
        info!("CloseDW\n dwreserved {:?}", dwreserved);
        Err(E_NOTIMPL.into())
    }

    fn ResizeBorderDW(
        &self,
        prcborder: *const RECT,
        punktoolbarsite: &core::option::Option<windows::core::IUnknown>,
        freserved: BOOL,
    ) -> windows::core::Result<()> {
        info!("ResizeBorderDW\n prcborder {:?}\n punktoolbarsite {:?}\n freserved {:?}", prcborder, punktoolbarsite, freserved);
        Err(E_NOTIMPL.into())
    }
}

impl IOleWindow_Impl for MyDeskBand {
    fn GetWindow(&self) -> windows::core::Result<HWND> {
        info!("GetWindow");
        todo!()
    }

    fn ContextSensitiveHelp(&self, fentermode: BOOL) -> windows::core::Result<()> {
        info!("ContextSensitiveHelp\n fentermode {:?}", fentermode);
        Err(E_NOTIMPL.into())
    }
}

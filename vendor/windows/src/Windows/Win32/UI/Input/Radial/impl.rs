#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerConfigurationInterop_Impl: Sized {
    fn GetForWindow(&mut self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerConfigurationInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerConfigurationInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>() -> IRadialControllerConfigurationInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerConfigurationInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerIndependentInputSourceInterop_Impl: Sized {
    fn CreateForWindow(&mut self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSourceInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>() -> IRadialControllerIndependentInputSourceInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerIndependentInputSourceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSourceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRadialControllerInterop_Impl: Sized {
    fn CreateForWindow(&mut self, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRadialControllerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IRadialControllerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerInterop_Impl, const OFFSET: isize>() -> IRadialControllerInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerInterop as ::windows::core::Interface>::IID
    }
}

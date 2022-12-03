#[cfg(feature = "Win32_Foundation")]
pub trait ICloneViewHelper_Impl: Sized {
    fn GetConnectedIDs(&mut self, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveTopology(&mut self, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::Result<()>;
    fn SetActiveTopology(&mut self, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::Result<()>;
    fn Commit(&mut self, ffinalcall: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICloneViewHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelper_Impl, const OFFSET: isize>() -> ICloneViewHelper_Vtbl {
        unsafe extern "system" fn GetConnectedIDs<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectedIDs(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pulid), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetActiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetActiveTopology(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn SetActiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveTopology(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: ICloneViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffinalcall: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&ffinalcall)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, Impl, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, Impl, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICloneViewHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IViewHelper_Impl: Sized {
    fn GetConnectedIDs(&mut self, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::Result<()>;
    fn GetActiveTopology(&mut self, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::Result<()>;
    fn SetActiveTopology(&mut self, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn SetConfiguration(&mut self, pistream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<u32>;
    fn GetProceedOnNewConfiguration(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IViewHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>() -> IViewHelper_Vtbl {
        unsafe extern "system" fn GetConnectedIDs<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectedIDs(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pulid), ::core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn GetActiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetActiveTopology(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&pulcount), ::core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn SetActiveTopology<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActiveTopology(::core::mem::transmute_copy(&wszadaptorname), ::core::mem::transmute_copy(&ulsourceid), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pultargetid)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistream: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetConfiguration(::core::mem::transmute(&pistream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProceedOnNewConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IViewHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProceedOnNewConfiguration().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectedIDs: GetConnectedIDs::<Identity, Impl, OFFSET>,
            GetActiveTopology: GetActiveTopology::<Identity, Impl, OFFSET>,
            SetActiveTopology: SetActiveTopology::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            GetProceedOnNewConfiguration: GetProceedOnNewConfiguration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewHelper as ::windows::core::Interface>::IID
    }
}

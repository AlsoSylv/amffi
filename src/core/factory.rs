use std::{
    ffi::CStr,
    path::{Path, PathBuf},
    ptr::null_mut,
};

use widestring::{WideCStr, WideChar};

use crate::{
    core::{
        compute::AMFPrograms,
        context::{AMFContext, AMFContextVtbl},
        debug::AMFDebug,
        interface::Interface,
        result::{AMFError, AMFResult},
        trace::AMFTrace,
    },
    stdcall,
};

use crate::components::component::AMFComponent;

pub const AMF_INIT_FUNCTION_NAME: &CStr = c"AMFInit";
pub const AMF_QUERY_VERSION_FUNCTION_NAME: &CStr = c"AMFQueryVersion";

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub const AMF_DLL_NAME: &str = "amfrt64.dll";
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub const AMF_LITE_DLL_NAME: &str = "amfrtlt64.dll";

#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub const AMF_DLL_NAME: &str = "amfrt32.dll";
#[cfg(all(target_arch = "x86", target_os = "windows"))]
pub const AMF_LITE_DLL_NAME: &str = "amfrtlt32.dll";

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    target_os = "linux"
))]
pub const AMF_DLL_NAME: &str = "libamfrt64.so.1";
#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    target_os = "linux"
))]
pub const AMF_LITE_DLL_NAME: &str = "libamfrt64.so.1";

#[cfg(all(target_arch = "x86", target_os = "linux"))]
pub const AMF_DLL_NAME: &str = "libamfrt32.so.1";
#[cfg(all(target_arch = "x86", target_os = "linux"))]
pub const AMF_LITE_DLL_NAME: &str = "libamfrt32.so.1";

#[cfg(target_os = "macos")]
/// MacOS does not have amfrtlt
pub const AMF_DLL_NAME: &str = "libamfrt.framework/libamfrt";

#[repr(C)]
pub struct AMFFactoryVtbl {
    create_context: stdcall!(fn(this: *mut *const Self, context: *mut AMFContext) -> AMFResult),
    create_component: stdcall!(
        fn(
            this: *mut *const Self,
            context: *mut *const AMFContextVtbl,
            name: *const WideChar,
            component: *mut AMFComponent,
        ) -> AMFResult
    ),
    set_cache_folder: stdcall!(fn(this: *mut *const Self, path: *const WideChar) -> AMFResult),
    get_cache_folder: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    get_debug: stdcall!(fn(this: *mut *const Self, debug: *mut AMFDebug) -> AMFResult),
    get_trace: stdcall!(fn(this: *mut *const Self, trace: *mut AMFTrace) -> AMFResult),
    get_programs: stdcall!(fn(this: *mut *const Self, programs: *mut AMFPrograms) -> AMFResult),
}

#[repr(C)]
pub struct AMFFactory(*mut *const AMFFactoryVtbl);

impl AMFFactory {
    unsafe fn vtable(&self) -> &AMFFactoryVtbl {
        unsafe { &**self.0 }
    }

    unsafe fn as_raw(&self) -> *mut *const AMFFactoryVtbl {
        self.0
    }

    pub fn create_context(&self) -> Result<AMFContext, AMFError> {
        let mut context = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().create_context)(self.as_raw(), context.as_mut_ptr()) }
            .into_error()?;
        Ok(unsafe { context.assume_init() })
    }

    pub fn create_component(
        &self,
        context: &AMFContext,
        name: &WideCStr,
    ) -> Result<AMFComponent, AMFError> {
        let mut component = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().create_component)(
                self.as_raw(),
                context.as_raw(),
                name.as_ptr(),
                component.as_mut_ptr(),
            )
        }
        .into_error()?;
        Ok(unsafe { component.assume_init() })
    }

    pub fn get_cache_folder(&self) -> PathBuf {
        let cstr = unsafe { (self.vtable().get_cache_folder)(self.as_raw()) };
        let cstr = unsafe { widestring::WideCStr::from_ptr_str(cstr) };
        cstr.to_string().unwrap().into()
    }

    pub fn set_cache_folder(&self, path: &Path) -> Result<(), AMFError> {
        let path = widestring::WideCString::from_os_str(path.as_os_str()).unwrap();
        unsafe { (self.vtable().set_cache_folder)(self.as_raw(), path.as_ptr()) }.into_error()
    }

    pub fn get_debug(&self) -> Result<AMFDebug, AMFError> {
        let mut debug = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().get_debug)(self.as_raw(), debug.as_mut_ptr()).into_error()? };
        Ok(unsafe { debug.assume_init() })
    }

    pub fn get_trace(&self) -> Result<AMFTrace, AMFError> {
        let mut trace = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().get_trace)(self.as_raw(), trace.as_mut_ptr()).into_error()? };
        Ok(unsafe { trace.assume_init() })
    }

    pub fn get_programs(&self) -> Result<AMFPrograms, AMFError> {
        let mut programs = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().get_programs)(self.as_raw(), programs.as_mut_ptr()) }
            .into_error()?;
        Ok(unsafe { programs.assume_init() })
    }
}

impl Default for AMFFactory {
    fn default() -> Self {
        Self(null_mut())
    }
}

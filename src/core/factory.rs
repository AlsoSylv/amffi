use std::ptr::null_mut;

use widestring::{WideCStr, WideChar};

use crate::{
    core::{
        context::{AMFContext, AMFContextVtbl},
        debug::AMFDebug,
        interface::Interface,
        result::{AMFError, AMFResult},
    },
    stdcall,
};

use crate::components::component::AMFComponent;

pub const AMF_INIT_FUNCTION_NAME: &str = "AMFInit";
pub const AMF_QUERY_VERSION_FUNCTION_NAME: &str = "AMFQueryVersion";

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
    // TODO: Expose in wrapper function
    set_cache_folder: stdcall!(
        fn(
            this: *mut *const Self,
            context: *mut *const AMFContextVtbl,
            name: *const WideChar,
        ) -> AMFResult
    ),
    // TODO: Expose in wrapper function
    get_cache_folder: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    // TODO: Expose in wrapper function
    get_debug: stdcall!(fn(this: *mut *const Self, debug: *mut AMFDebug) -> AMFResult),
    // TODO: Add trace support
    get_trace: *mut (),
    // TODO: Add programs support
    get_programs: *mut (),
}

#[repr(C)]
pub struct AMFFactory(*mut *const AMFFactoryVtbl);

impl AMFFactory {
    unsafe fn vtable(&self) -> &AMFFactoryVtbl {
        unsafe { &**self.0 }
    }

    pub fn create_context(&self) -> Result<AMFContext, AMFError> {
        let mut context = AMFContext::default();
        unsafe { (self.vtable().create_context)(self.0, &raw mut context) }.into_error()?;
        Ok(context)
    }

    pub fn create_component(
        &self,
        context: &AMFContext,
        name: &WideCStr,
    ) -> Result<AMFComponent, AMFError> {
        let mut component = AMFComponent::default();
        unsafe {
            (self.vtable().create_component)(
                self.0,
                context.as_raw(),
                name.as_ptr(),
                &raw mut component,
            )
        }
        .into_error()?;
        Ok(component)
    }
}

impl Default for AMFFactory {
    fn default() -> Self {
        Self(null_mut())
    }
}

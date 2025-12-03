use std::sync::OnceLock;

use libloading::Library;

use crate::core::{
    factory::{AMF_DLL_NAME, AMF_INIT_FUNCTION_NAME, AMF_QUERY_VERSION_FUNCTION_NAME, AMFFactory},
    result::AMFResult,
};

pub mod components;
pub mod core;

static LIBRARY: OnceLock<AMFLibrary> = OnceLock::new();

pub struct AMFLibrary {
    #[allow(unused)]
    // The lib needs to stay alive AS LONG AMF IS IN USE
    lib: Library,
    init: cdecl!(fn(version: u64, factory: *mut AMFFactory) -> AMFResult),
    query_version: cdecl!(fn(version: *mut u64) -> AMFResult),
}

impl AMFLibrary {
    pub fn query_version(&self) -> Result<u64, AMFResult> {
        let mut version = 0;
        let result = unsafe { (self.query_version)(&raw mut version) };
        if result != AMFResult::Ok {
            Err(result)
        } else {
            Ok(version)
        }
    }

    pub fn init_factory(&self, version_number: u64) -> Result<AMFFactory, AMFResult> {
        let mut factory = AMFFactory::default();
        let result = unsafe { (self.init)(version_number, &raw mut factory) };
        if result != AMFResult::Ok {
            Err(result)
        } else {
            Ok(factory)
        }
    }
}

pub fn amf_init() -> Result<&'static AMFLibrary, libloading::Error> {
    if let Some(lib) = LIBRARY.get() {
        Ok(lib)
    } else {
        let lib = unsafe { Library::new(AMF_DLL_NAME)? };
        let init = unsafe { *lib.get(AMF_INIT_FUNCTION_NAME)? };
        let query_version = unsafe { *lib.get(AMF_QUERY_VERSION_FUNCTION_NAME)? };

        let lib = AMFLibrary {
            lib,
            init,
            query_version,
        };

        Ok(LIBRARY.get_or_init(|| lib))
    }
}

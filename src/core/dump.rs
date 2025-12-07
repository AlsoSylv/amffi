use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use widestring::{WideCStr, WideCString, WideChar};

use crate::{
    core::{
        interface::{AMFInterface, AMFInterfaceVtbl, Interface},
        platform::Guid,
        result::{AMFError, AMFResult},
    },
    stdcall,
};

#[repr(transparent)]
pub struct AMFDump(<Self as Deref>::Target);

#[repr(C)]
pub struct AMFDumpVtbl {
    base: AMFInterfaceVtbl,
    get_dump_base_path: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    set_dump_base_path: stdcall!(fn(this: *mut *const Self, path: *const WideChar) -> AMFResult),
    is_input_dump_enabled: stdcall!(fn(this: *mut *const Self) -> bool),
    enable_input_dump: stdcall!(fn(this: *mut *const Self, enabled: bool) -> AMFResult),
    get_input_dump_full_name: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    is_output_dump_enabled: stdcall!(fn(this: *mut *const Self) -> bool),
    enable_output_dump: stdcall!(fn(this: *mut *const Self, enabled: bool) -> AMFResult),
    get_output_dump_full_name: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    is_per_session_dump_enabled: stdcall!(fn(this: *mut *const Self) -> bool),
    enable_per_session_dump: stdcall!(fn(this: *mut *const Self, enabled: bool)),
}

impl AMFDump {
    pub fn get_dump_base_path(&self) -> PathBuf {
        let wide =
            unsafe { WideCStr::from_ptr_str((self.vtable().get_dump_base_path)(self.as_raw())) };
        PathBuf::from(wide.to_os_string())
    }

    pub fn set_dump_base_path(&self, path: &Path) -> Result<(), AMFError> {
        let wide = WideCString::from_os_str(path.as_os_str()).expect("Path cannot contain null");
        unsafe { (self.vtable().set_dump_base_path)(self.as_raw(), wide.as_ptr()).into_error() }
    }

    pub fn is_input_dump_enabled(&self) -> bool {
        unsafe { (self.vtable().is_input_dump_enabled)(self.as_raw()) }
    }

    pub fn enable_input_dump(&self, enabled: bool) -> Result<(), AMFError> {
        unsafe { (self.vtable().enable_input_dump)(self.as_raw(), enabled).into_error() }
    }

    pub fn get_input_dump_full_name(&self) -> PathBuf {
        let wide = unsafe {
            WideCStr::from_ptr_str((self.vtable().get_input_dump_full_name)(self.as_raw()))
        };
        PathBuf::from(wide.to_os_string())
    }

    pub fn is_output_dump_enabled(&self) -> bool {
        unsafe { (self.vtable().is_output_dump_enabled)(self.as_raw()) }
    }

    pub fn enable_output_dump(&self, enabled: bool) -> Result<(), AMFError> {
        unsafe { (self.vtable().enable_output_dump)(self.as_raw(), enabled).into_error() }
    }

    pub fn get_output_dump_full_name(&self) -> PathBuf {
        let wide = unsafe {
            WideCStr::from_ptr_str((self.vtable().get_output_dump_full_name)(self.as_raw()))
        };
        PathBuf::from(wide.to_os_string())
    }

    pub fn is_per_session_dump_enabled(&self) -> bool {
        unsafe { (self.vtable().is_per_session_dump_enabled)(self.as_raw()) }
    }

    pub fn enable_per_session_dump(&self, enabled: bool) {
        unsafe { (self.vtable().enable_per_session_dump)(self.as_raw(), enabled) }
    }
}

impl Interface for AMFDump {
    type Vtbl = AMFDumpVtbl;

    const GUID: Guid = Guid::from_values(
        0x75366ad4,
        0x504c,
        0x430b,
        [0xbb, 0xe2, 0xad, 0x21, 0x82, 0x8, 0xf, 0x72],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl Deref for AMFDump {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

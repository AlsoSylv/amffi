use std::ffi::c_void;

use crate::{
    core::interface::{AMFInterface, AMFInterfaceVtbl, Guid, Interface},
    stdcall,
};

#[repr(C)]
pub enum AMFPlaneType {
    Unknown = 0,
    Packed = 1,
    Y = 2,
    UV = 3,
    U = 4,
    V = 5,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFPlane(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFPlaneVtbl {
    base: AMFInterfaceVtbl,
    get_type: stdcall!(fn(this: *mut *const Self) -> AMFPlaneType),
    get_native: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_pixel_size_in_bytes: stdcall!(fn(this: *mut *const Self) -> i32),
    get_offset_x: stdcall!(fn(this: *mut *const Self) -> i32),
    get_offset_y: stdcall!(fn(this: *mut *const Self) -> i32),
    get_width: stdcall!(fn(this: *mut *const Self) -> i32),
    get_height: stdcall!(fn(this: *mut *const Self) -> i32),
    get_h_pitch: stdcall!(fn(this: *mut *const Self) -> i32),
    get_v_pitch: stdcall!(fn(this: *mut *const Self) -> i32),
    is_tiled: stdcall!(fn(this: *mut *const Self) -> bool),
}

impl AMFPlane {
    pub fn get_type(&self) -> AMFPlaneType {
        unsafe { (self.vtable().get_type)(self.as_raw()) }
    }
    pub fn get_native(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native)(self.as_raw()) }
    }
    pub fn get_pixel_size_in_bytes(&self) -> i32 {
        unsafe { (self.vtable().get_pixel_size_in_bytes)(self.as_raw()) }
    }
    pub fn get_offset_x(&self) -> i32 {
        unsafe { (self.vtable().get_offset_x)(self.as_raw()) }
    }
    pub fn get_offset_y(&self) -> i32 {
        unsafe { (self.vtable().get_offset_y)(self.as_raw()) }
    }
    pub fn get_width(&self) -> i32 {
        unsafe { (self.vtable().get_width)(self.as_raw()) }
    }
    pub fn get_height(&self) -> i32 {
        unsafe { (self.vtable().get_height)(self.as_raw()) }
    }
    pub fn get_h_pitch(&self) -> i32 {
        unsafe { (self.vtable().get_h_pitch)(self.as_raw()) }
    }
    pub fn get_v_pitch(&self) -> i32 {
        unsafe { (self.vtable().get_v_pitch)(self.as_raw()) }
    }
    pub fn is_tiled(&self) -> bool {
        unsafe { (self.vtable().is_tiled)(self.as_raw()) }
    }
}

impl Interface for AMFPlane {
    type Vtbl = AMFPlaneVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0xbede1aa6,
        0xd8fa,
        0x4625,
        [0x94, 0x65, 0x6c, 0x82, 0xc4, 0x37, 0x71, 0x2e],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFPlane {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

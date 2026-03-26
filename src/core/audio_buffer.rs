use std::os::raw::c_void;

use crate::{
    core::{
        data::{AMFData, AMFDataVtbl},
        interface::Interface,
    },
    stdcall,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub enum AMFAudioFormat {
    Unknown = -1,
    U8 = 0,
    S16 = 1,
    S32 = 2,
    FLT = 3,
    DBL = 4,
    U8P = Self::U8 as isize + 5,
    S16P = Self::S16 as isize + 5,
    S32P = Self::S32 as isize + 5,
    FLTP = Self::FLT as isize + 5,
    DBLP = Self::DBL as isize + 5,
}

#[repr(C)]
/// Bitflags
pub enum AMFAudioChannelLayout {
    FrontLeft = 0x1,
    FrontRight = 0x2,
    FrontCenter = 0x4,
    LowFrequency = 0x8,
    BackLeft = 0x10,
    BackRight = 0x20,
    FrontLeftOfCenter = 0x40,
    FrontRightOfCenter = 0x80,
    BackCenter = 0x100,
    SideLeft = 0x200,
    SideRight = 0x400,
    TopCenter = 0x800,
    TopFrontLeft = 0x1000,
    TopFrontCenter = 0x2000,
    TopFrontRight = 0x4000,
    TopBackLeft = 0x8000,
    TopBackCenter = 0x10000,
    TopBackRight = 0x20000,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFAudioBuffer(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFAudioBufferVtbl {
    base: AMFDataVtbl,
    get_sample_count: stdcall!(fn(this: *mut *const Self) -> i32),
    get_sample_rate: stdcall!(fn(this: *mut *const Self) -> i32),
    get_channel_count: stdcall!(fn(this: *mut *const Self) -> i32),
    get_sample_format: stdcall!(fn(this: *mut *const Self) -> AMFAudioFormat),
    get_sample_size: stdcall!(fn(this: *mut *const Self) -> i32),
    get_channel_layout: stdcall!(fn(this: *mut *const Self) -> u32),
    get_native: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_size: stdcall!(fn(this: *mut *const Self) -> isize),
}

impl AMFAudioBuffer {
    pub fn get_sample_count(&self) -> i32 {
        unsafe { (self.vtable().get_sample_count)(self.as_raw()) }
    }

    pub fn get_sample_rate(&self) -> i32 {
        unsafe { (self.vtable().get_sample_rate)(self.as_raw()) }
    }

    pub fn get_channel_count(&self) -> i32 {
        unsafe { (self.vtable().get_channel_count)(self.as_raw()) }
    }

    pub fn get_sample_format(&self) -> AMFAudioFormat {
        unsafe { (self.vtable().get_sample_format)(self.as_raw()) }
    }

    pub fn get_sample_size(&self) -> i32 {
        unsafe { (self.vtable().get_sample_size)(self.as_raw()) }
    }

    pub fn get_channel_layout(&self) -> u32 {
        unsafe { (self.vtable().get_channel_layout)(self.as_raw()) }
    }

    pub fn get_native(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native)(self.as_raw()) }
    }

    pub fn get_size(&self) -> isize {
        unsafe { (self.vtable().get_size)(self.as_raw()) }
    }
}

impl super::interface::sealed::Sealed for AMFAudioBuffer {}

impl Interface for AMFAudioBuffer {
    type Vtbl = AMFAudioBufferVtbl;

    const GUID: super::platform::Guid = super::platform::Guid::from_values(
        0x2212ff8,
        0x6107,
        0x430b,
        [0xb6, 0x3c, 0xc7, 0xe5, 0x40, 0xe5, 0xf8, 0xeb],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFAudioBuffer {
    type Target = AMFData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

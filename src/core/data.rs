use crate::{
    core::{
        interface::{Guid, Interface},
        property_storage::{AMFPropertyStorage, AMFPropertyStorageVtbl},
        result::{AMFError, AMFResult},
    },
    stdcall,
};

#[repr(C)]
pub enum AMFDataType {
    Buffer = 0,
    Surface = 1,
    AudioBuffer = 2,
    User = 1000,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum AMFMemoryType {
    Unknown = 0,
    Host = 1,
    DX9 = 2,
    DX11 = 3,
    OpenCL = 4,
    OpenGL = 5,
    XV = 6,
    Gralloc = 7,
    /// deprecated, the same as AMF_MEMORY_OPENCL
    ComputeForDX9 = 8,
    /// deprecated, the same as AMF_MEMORY_OPENCL
    ComputeForDX11 = 9,
    Vulkan = 10,
    DX12 = 11,
    Last,
}

#[repr(C)]
pub enum AMFDXVersion {
    DX9 = 90,
    DX9Ex = 91,
    DX11_0 = 110,
    DX11_1 = 111,
    DX12 = 120,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct AMFData(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFDataVtbl {
    base: AMFPropertyStorageVtbl,
    get_memory_type: stdcall!(fn(this: *mut *const Self) -> AMFMemoryType),
    duplicate:
        stdcall!(fn(this: *mut *const Self, ty: AMFMemoryType, data: *mut AMFData) -> AMFResult),
    convert: stdcall!(fn(this: *mut *const Self, ty: AMFMemoryType) -> AMFResult),
    interop: stdcall!(fn(this: *mut *const Self, ty: AMFMemoryType) -> AMFResult),
    get_data_type: stdcall!(fn(this: *mut *const Self) -> AMFDataType),
    is_reusable: stdcall!(fn(this: *mut *const Self) -> bool),
    set_pts: stdcall!(fn(this: *mut *const Self, pts: i64)),
    get_pts: stdcall!(fn(this: *mut *const Self) -> i64),
    set_duration: stdcall!(fn(this: *mut *const Self, duration: i64)),
    get_duration: stdcall!(fn(this: *mut *const Self) -> i64),
}

impl AMFData {
    pub fn get_memory_type(&self) -> AMFMemoryType {
        unsafe { (self.vtable().get_memory_type)(self.as_raw()) }
    }

    pub fn duplicate(&self) -> Result<AMFData, AMFError> {
        let mut data = std::mem::MaybeUninit::uninit();
        let ty = self.get_memory_type();
        unsafe { (self.vtable().duplicate)(self.as_raw(), ty, data.as_mut_ptr()).into_error()? };
        Ok(unsafe { data.assume_init() })
    }

    pub fn convert(&self, ty: AMFMemoryType) -> Result<(), AMFError> {
        unsafe { (self.vtable().convert)(self.as_raw(), ty) }.into_error()
    }

    pub fn interop(&self, ty: AMFMemoryType) -> Result<(), AMFError> {
        unsafe { (self.vtable().interop)(self.as_raw(), ty) }.into_error()
    }

    pub fn get_data_type(&self) -> AMFDataType {
        unsafe { (self.vtable().get_data_type)(self.as_raw()) }
    }

    pub fn is_reusable(&self) -> bool {
        unsafe { (self.vtable().is_reusable)(self.as_raw()) }
    }

    pub fn set_pts(&self, pts: i64) {
        unsafe { (self.vtable().set_pts)(self.as_raw(), pts) }
    }

    pub fn get_pts(&self) -> i64 {
        unsafe { (self.vtable().get_pts)(self.as_raw()) }
    }

    pub fn set_duration(&self, duration: i64) {
        unsafe { (self.vtable().set_duration)(self.as_raw(), duration) }
    }

    pub fn get_duration(&self) -> i64 {
        unsafe { (self.vtable().get_duration)(self.as_raw()) }
    }
}

impl super::interface::sealed::Sealed for AMFData {}

impl Interface for AMFData {
    type Vtbl = AMFDataVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0xa1159bf6,
        0x9104,
        0x4107,
        [0x8e, 0xaa, 0xc5, 0x3d, 0x5d, 0xba, 0xc5, 0x11],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFData {
    type Target = AMFPropertyStorage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

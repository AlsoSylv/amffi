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

#[derive(Default, Clone)]
#[repr(transparent)]
pub struct AMFData(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFDataVtbl {
    base: AMFPropertyStorageVtbl,
    get_memory_type: stdcall!(fn(this: *mut *const AMFDataVtbl) -> AMFMemoryType),
    duplicate: stdcall!(
        fn(this: *mut *const AMFDataVtbl, ty: AMFMemoryType, data: *mut AMFData) -> AMFResult
    ),
    convert: stdcall!(fn(this: *mut *const AMFDataVtbl, ty: AMFMemoryType) -> AMFResult),
    interop: stdcall!(fn(this: *mut *const AMFDataVtbl, ty: AMFMemoryType) -> AMFResult),
    get_data_type: stdcall!(fn(this: *mut *const AMFDataVtbl) -> AMFDataType),
    is_reusable: stdcall!(fn(this: *mut *const AMFDataVtbl) -> bool),
    set_pts: stdcall!(fn(this: *mut *const AMFDataVtbl, pts: i64)),
    get_pts: stdcall!(fn(this: *mut *const AMFDataVtbl) -> i64),
    set_duration: stdcall!(fn(this: *mut *const AMFDataVtbl, duration: i64)),
    get_duration: stdcall!(fn(this: *mut *const AMFDataVtbl) -> i64),
}

impl AMFData {
    pub fn get_memory_type(&self) -> AMFMemoryType {
        unsafe { (self.vtable().get_memory_type)(self.as_raw()) }
    }

    pub fn duplicate(&self) -> Result<AMFData, AMFError> {
        let mut data = AMFData::default();
        let ty = self.get_memory_type();
        unsafe { (self.vtable().duplicate)(self.as_raw(), ty, &raw mut data) };
        Ok(data)
    }

    pub fn convert(&self, ty: AMFMemoryType) -> Result<(), AMFError> {
        unsafe { (self.vtable().convert)(self.as_raw(), ty) }.into_error()
    }
}

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

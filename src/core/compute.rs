use std::{ffi::c_void, ops::Deref};

use crate::{
    core::{
        data::AMFMemoryType,
        interface::{AMFInterface, AMFInterfaceVtbl, Interface},
        plane::AMFPlane,
        platform::Guid,
        result::{AMFError, AMFResult},
    },
    stdcall,
};

#[repr(transparent)]
#[derive(Default)]
pub struct AMFCompute(<Self as Deref>::Target);

#[repr(C)]
pub struct AMFComputeVtbl {
    base: AMFInterfaceVtbl,
    get_memory_type: stdcall!(fn(this: *mut *const Self) -> AMFMemoryType),
    get_native_context: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_compute_device_id: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_native_compute_queue: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_kernel: stdcall!(fn(this: *mut *const Self)),
    put_sync_point: stdcall!(fn(this: *mut *const Self)),
    finish_queue: stdcall!(fn(this: *mut *const Self)),
    flush_queue: stdcall!(fn(this: *mut *const Self)),
    fill_plane: stdcall!(fn(this: *mut *const Self)),
    fill_buffer: stdcall!(fn(this: *mut *const Self)),
    convert_plane_to_buffer: stdcall!(fn(this: *mut *const Self)),
    copy_buffer: stdcall!(fn(this: *mut *const Self)),
    copy_plane: stdcall!(
        fn(
            this: *mut *const Self,
            src_plane: AMFPlane,
            src_origin: *const isize,
            region: *const isize,
            dst_plane: AMFPlane,
            dst_origin: *const isize,
        ) -> AMFResult
    ),
    copy_buffer_to_host: stdcall!(fn(this: *mut *const Self)),
    copy_buffer_from_host: stdcall!(fn(this: *mut *const Self)),
    copy_plane_to_host: stdcall!(fn(this: *mut *const Self)),
    copy_plane_from_host: stdcall!(fn(this: *mut *const Self)),
    convert_plane_to_plane: stdcall!(fn(this: *mut *const Self)),
}

impl AMFCompute {
    pub fn copy_plane(
        &self,
        src_plane: AMFPlane,
        src_origin: [isize; 3],
        region: [isize; 3],
        dst_plane: AMFPlane,
        dst_origin: [isize; 3],
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_plane)(
                self.as_raw(),
                src_plane,
                src_origin.as_ptr(),
                region.as_ptr(),
                dst_plane,
                dst_origin.as_ptr(),
            )
            .into_error()
        }
    }
}

impl Interface for AMFCompute {
    type Vtbl = AMFComputeVtbl;

    const GUID: Guid = Guid::from_values(
        0x3846233a,
        0x3f43,
        0x443f,
        [0x8a, 0x45, 0x75, 0x22, 0x11, 0xa9, 0xfb, 0xd5],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl Deref for AMFCompute {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::{
    ffi::{CStr, c_char, c_void},
    ops::Deref,
    path::Path,
};

use widestring::{WideCStr, WideChar};

use crate::{
    core::{
        buffer::AMFBuffer,
        compute::sealed::Color,
        data::AMFMemoryType,
        interface::{AMFInterface, AMFInterfaceVtbl, Interface},
        plane::AMFPlane,
        platform::Guid,
        result::{AMFError, AMFResult},
    },
    stdcall,
};

#[repr(C)]
pub enum AMFChannelOrder {
    Invalid = 0,
    R = 1,
    RG = 2,
    BGRA = 3,
    RGBA = 4,
    ARGB = 5,
    YUV2 = 6,
    ABGR = 7,
}

#[repr(C)]
pub enum AMFChannelType {
    Invalid = 0,
    UnsignedI8 = 1,
    UnsignedI32 = 2,
    UNormI8 = 3,
    UNormI16 = 4,
    SNormI16 = 5,
    Float = 6,
    Float16 = 7,
    U16 = 8,
    I101010 = 9,
}

#[repr(C)]
pub enum AMFArgumentAccess {
    Read = 0,
    Write = 1,
    ReadWrite = 2,
    ReadWriteMask = 0xFFFF,
    SamplerLineaer = 0x10000000,
    SamplerNormCoord = 0x20000000,
    SamplerPoint = 0x40000000,
    SamplerMask = 0xFFFF0000,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFComputeKernel(<Self as Deref>::Target);

#[repr(C)]
pub struct AMFComputeKernelVtbl {
    base: AMFInterfaceVtbl,
    get_native: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_id_name: stdcall!(fn(this: *mut *const Self) -> *const WideChar),
    set_arg_plane_native: stdcall!(
        fn(
            this: *mut *const Self,
            idx: isize,
            plane: *mut c_void,
            access: AMFArgumentAccess,
        ) -> AMFResult
    ),
    set_arg_buffer_native: stdcall!(
        fn(
            this: *mut *const Self,
            idx: isize,
            buffer: *mut c_void,
            access: AMFArgumentAccess,
        ) -> AMFResult
    ),
    set_arg_plane: stdcall!(
        fn(
            this: *mut *const Self,
            idx: isize,
            plane: AMFPlane,
            access: AMFArgumentAccess,
        ) -> AMFResult
    ),
    set_arg_buffer: stdcall!(
        fn(
            this: *mut *const Self,
            idx: isize,
            buffer: AMFBuffer,
            access: AMFArgumentAccess,
        ) -> AMFResult
    ),
    set_arg_i32: stdcall!(fn(this: *mut *const Self, idx: isize, data: i32) -> AMFResult),
    set_arg_i64: stdcall!(fn(this: *mut *const Self, idx: isize, data: i64) -> AMFResult),
    set_arg_float: stdcall!(fn(this: *mut *const Self, idx: isize, data: f32) -> AMFResult),
    set_arg_blob: stdcall!(
        fn(this: *mut *const Self, idx: isize, data_size: isize, data: *const c_void) -> AMFResult
    ),
    get_compile_workgroup_size:
        stdcall!(fn(this: *mut *const Self, worksize_group: *mut isize) -> AMFResult),
    enqueue: stdcall!(
        fn(
            this: *mut *const Self,
            dimension: isize,
            global_offset: *const isize,
            global_size: *const isize,
            local_size: *const isize,
        ) -> AMFResult
    ),
}

impl AMFComputeKernel {
    pub fn get_native(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native)(self.as_raw()) }
    }

    pub fn get_id_name(&self) -> &WideCStr {
        unsafe { WideCStr::from_ptr_str((self.vtable().get_id_name)(self.as_raw())) }
    }

    pub fn set_arg_plane_native(
        &self,
        idx: isize,
        plane: *mut c_void,
        access: AMFArgumentAccess,
    ) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_plane_native)(self.as_raw(), idx, plane, access) }
            .into_error()
    }

    pub fn set_arg_buffer_native(
        &self,
        idx: isize,
        buffer: *mut c_void,
        access: AMFArgumentAccess,
    ) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_buffer_native)(self.as_raw(), idx, buffer, access) }
            .into_error()
    }

    pub fn set_arg_plane(
        &self,
        idx: isize,
        plane: AMFPlane,
        access: AMFArgumentAccess,
    ) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_plane)(self.as_raw(), idx, plane, access) }.into_error()
    }

    pub fn set_arg_buffer(
        &self,
        idx: isize,
        buffer: AMFBuffer,
        access: AMFArgumentAccess,
    ) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_buffer)(self.as_raw(), idx, buffer, access) }.into_error()
    }

    pub fn set_arg_i32(&self, idx: isize, val: i32) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_i32)(self.as_raw(), idx, val) }.into_error()
    }

    pub fn set_arg_i64(&self, idx: isize, val: i64) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_i64)(self.as_raw(), idx, val) }.into_error()
    }

    pub fn set_arg_float(&self, idx: isize, val: f32) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_arg_float)(self.as_raw(), idx, val) }.into_error()
    }

    pub fn set_arg_blob(&self, idx: isize, data: &[u8]) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().set_arg_blob)(
                self.as_raw(),
                idx,
                data.len() as isize,
                data.as_ptr() as _,
            )
        }
        .into_error()
    }

    pub fn get_compile_workgroup_size(&self) -> Result<[isize; 3], AMFError> {
        let mut workgroup_size = [0; 3];
        unsafe {
            (self.vtable().get_compile_workgroup_size)(self.as_raw(), workgroup_size.as_mut_ptr())
        }
        .into_error()?;
        Ok(workgroup_size)
    }

    pub fn enqueue(
        &self,
        dimension: isize,
        global_offset: [isize; 3],
        global_size: [isize; 3],
        local_size: [isize; 3],
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().enqueue)(
                self.as_raw(),
                dimension,
                global_offset.as_ptr(),
                global_size.as_ptr(),
                local_size.as_ptr(),
            )
        }
        .into_error()
    }
}

impl super::interface::sealed::Sealed for AMFComputeKernel {}

impl Interface for AMFComputeKernel {
    type Vtbl = AMFComputeKernelVtbl;

    const GUID: Guid = Guid::from_values(
        0x94815701,
        0x6c84,
        0x4ba6,
        [0xa9, 0xfe, 0xe9, 0xad, 0x40, 0xf8, 0x8, 0x8],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl Deref for AMFComputeKernel {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFComputeSyncPoint(<Self as Deref>::Target);

#[repr(C)]
pub struct AMFComputeSyncPointVtbl {
    base: AMFInterfaceVtbl,
    is_complete: stdcall!(fn(this: *mut *const Self) -> bool),
    wait: stdcall!(fn(this: *mut *const Self)),
}

impl AMFComputeSyncPoint {
    pub fn is_complete(&self) -> bool {
        unsafe { (self.vtable().is_complete)(self.as_raw()) }
    }

    pub fn wait(&self) {
        unsafe { (self.vtable().wait)(self.as_raw()) }
    }
}

impl super::interface::sealed::Sealed for AMFComputeSyncPoint {}

impl Interface for AMFComputeSyncPoint {
    type Vtbl = AMFComputeSyncPointVtbl;

    const GUID: Guid = Guid::from_values(
        0x66f33fe6,
        0xaae,
        0x4e65,
        [0xba, 0x3, 0xea, 0x8b, 0xa3, 0x60, 0x11, 0x2],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl Deref for AMFComputeSyncPoint {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFCompute(<Self as Deref>::Target);

#[repr(C)]
pub struct AMFComputeVtbl {
    base: AMFInterfaceVtbl,
    get_memory_type: stdcall!(fn(this: *mut *const Self) -> AMFMemoryType),
    get_native_context: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_compute_device_id: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_native_compute_queue: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_kernel: stdcall!(
        fn(this: *mut *const Self, kernel_id: u64, kernel: *mut AMFComputeKernel) -> AMFResult
    ),
    put_sync_point:
        stdcall!(fn(this: *mut *const Self, sync_point: *mut AMFComputeSyncPoint) -> AMFResult),
    finish_queue: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    flush_queue: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    fill_plane: stdcall!(
        fn(
            this: *mut *const Self,
            plane: AMFPlane,
            origin: *const isize,
            region: *const isize,
            color: *const c_void,
        ) -> AMFResult
    ),
    fill_buffer: stdcall!(
        fn(
            this: *mut *const Self,
            buffer: AMFBuffer,
            dst_offset: isize,
            dst_size: isize,
            source_pattern: *const c_void,
            pattern_size: isize,
        ) -> AMFResult
    ),
    convert_plane_to_buffer:
        stdcall!(fn(this: *mut *const Self, src_plane: AMFPlane, *mut AMFBuffer) -> AMFResult),
    copy_buffer: stdcall!(
        fn(
            this: *mut *const Self,
            src_buffer: AMFBuffer,
            src_offset: isize,
            size: isize,
            dst_buffer: AMFBuffer,
            dst_offset: isize,
        ) -> AMFResult
    ),
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
    copy_buffer_to_host: stdcall!(
        fn(
            this: *mut *const Self,
            src_buffer: AMFBuffer,
            src_offset: isize,
            size: isize,
            dst: *mut c_void,
            blocking: bool,
        ) -> AMFResult
    ),
    copy_buffer_from_host: stdcall!(
        fn(
            this: *mut *const Self,
            source: *const c_void,
            size: isize,
            dst_buffer: AMFBuffer,
            dst_offset_in_bytes: isize,
            blocking: bool,
        ) -> AMFResult
    ),
    copy_plane_to_host: stdcall!(
        fn(
            this: *mut *const Self,
            src_plane: AMFPlane,
            origin: *const isize,
            region: *const isize,
            dst: *mut c_void,
            dst_pitch: isize,
            blocking: bool,
        ) -> AMFResult
    ),
    copy_plane_from_host: stdcall!(
        fn(
            this: *mut *const Self,
            source: *const c_void,
            origin: *const isize,
            region: *const isize,
            src_pitch: isize,
            dst_plane: AMFPlane,
            blocking: bool,
        ) -> AMFResult
    ),
    convert_plane_to_plane: stdcall!(
        fn(
            this: *mut *const Self,
            src_plane: AMFPlane,
            dst_plane: *mut AMFPlane,
            order: AMFChannelOrder,
            ty: AMFChannelType,
        ) -> AMFResult
    ),
}

impl AMFCompute {
    pub fn get_memory_type(&self) -> AMFMemoryType {
        unsafe { (self.vtable().get_memory_type)(self.as_raw()) }
    }

    pub fn get_native_context(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native_context)(self.as_raw()) }
    }

    #[cfg(feature = "opencl3")]
    pub fn ocl3_get_native_context(&self) -> opencl3::context::Context {
        let device_id = self.get_compute_device_id();
        opencl3::context::Context::new(self.get_native_context(), &[device_id])
    }

    pub fn get_compute_device_id(&self) -> *mut c_void {
        unsafe { (self.vtable().get_compute_device_id)(self.as_raw()) }
    }

    #[cfg(feature = "opencl3")]
    pub fn ocl3_get_compute_device(&self) -> opencl3::device::Device {
        opencl3::device::Device::new(self.get_compute_device_id())
    }

    pub fn get_native_compute_queue(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native_compute_queue)(self.as_raw()) }
    }

    #[cfg(feature = "opencl3")]
    pub fn ocl3_get_native_compute_queue(
        &self,
    ) -> Result<opencl3::command_queue::CommandQueue, opencl3::error_codes::ClError> {
        let max_working_dimensions = self.ocl3_get_compute_device().max_work_item_dimensions()?;
        Ok(opencl3::command_queue::CommandQueue::new(
            self.get_native_compute_queue(),
            max_working_dimensions,
        ))
    }

    pub fn get_kernel(&self, kernel_id: u64) -> Result<AMFComputeKernel, AMFError> {
        let mut kernel = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().get_kernel)(self.as_raw(), kernel_id, kernel.as_mut_ptr()) }
            .into_error()?;
        Ok(unsafe { kernel.assume_init() })
    }

    pub fn put_sync_point(&self) -> Result<AMFComputeSyncPoint, AMFError> {
        let mut sync_point = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().put_sync_point)(self.as_raw(), sync_point.as_mut_ptr()) }
            .into_error()?;
        Ok(unsafe { sync_point.assume_init() })
    }

    /// `color` is [f32; 4] for RGBA_16, and [u32; 4] otherwise
    pub fn fill_plane(
        &self,
        plane: AMFPlane,
        origin: [isize; 3],
        region: [isize; 3],
        color: impl Color,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().fill_plane)(
                self.as_raw(),
                plane,
                origin.as_ptr(),
                region.as_ptr(),
                color._as_ptr(),
            )
            .into_error()
        }
    }

    pub fn fill_buffer(
        &self,
        buffer: AMFBuffer,
        dst_offset: isize,
        dst_size: isize,
        source_pattern: &[u8],
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().fill_buffer)(
                self.as_raw(),
                buffer,
                dst_offset,
                dst_size,
                source_pattern.as_ptr() as _,
                source_pattern.len() as isize,
            )
            .into_error()
        }
    }

    pub fn convert_plane_to_buffer(&self, plane: AMFPlane) -> Result<AMFBuffer, AMFError> {
        let mut buffer = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().convert_plane_to_buffer)(self.as_raw(), plane, buffer.as_mut_ptr())
        }
        .into_error()?;
        Ok(unsafe { buffer.assume_init() })
    }

    pub fn copy_buffer(
        &self,
        src_buffer: AMFBuffer,
        src_offset: isize,
        size: isize,
        dst_buffer: AMFBuffer,
        dst_offset: isize,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_buffer)(
                self.as_raw(),
                src_buffer,
                src_offset,
                size,
                dst_buffer,
                dst_offset,
            )
            .into_error()
        }
    }

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

    pub fn copy_buffer_to_host(
        &self,
        src_buffer: AMFBuffer,
        src_offset: isize,
        size: isize,
        dst: *mut c_void,
        blocking: bool,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_buffer_to_host)(
                self.as_raw(),
                src_buffer,
                src_offset,
                size,
                dst,
                blocking,
            )
        }
        .into_error()
    }
    pub fn copy_buffer_from_host(
        &self,
        src: *const c_void,
        size: isize,
        dst_buffer: AMFBuffer,
        dst_offset_in_bytes: isize,
        blocking: bool,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_buffer_from_host)(
                self.as_raw(),
                src,
                size,
                dst_buffer,
                dst_offset_in_bytes,
                blocking,
            )
        }
        .into_error()
    }
    pub fn copy_plane_to_host(
        &self,
        src_plane: AMFPlane,
        origin: [isize; 3],
        region: [isize; 3],
        dst: *mut c_void,
        dst_pitch: isize,
        blocking: bool,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_plane_to_host)(
                self.as_raw(),
                src_plane,
                origin.as_ptr(),
                region.as_ptr(),
                dst,
                dst_pitch,
                blocking,
            )
        }
        .into_error()
    }
    pub fn copy_plane_from_host(
        &self,
        src: *const c_void,
        origin: [isize; 3],
        region: [isize; 3],
        src_pitch: isize,
        dst_plane: AMFPlane,
        blocking: bool,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_plane_from_host)(
                self.as_raw(),
                src,
                origin.as_ptr(),
                region.as_ptr(),
                src_pitch,
                dst_plane,
                blocking,
            )
        }
        .into_error()
    }

    pub fn convert_plane_to_plane(
        &self,
        src_plane: AMFPlane,
        order: AMFChannelOrder,
        ty: AMFChannelType,
    ) -> Result<AMFPlane, AMFError> {
        let mut plane = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().convert_plane_to_plane)(
                self.as_raw(),
                src_plane,
                plane.as_mut_ptr(),
                order,
                ty,
            )
        }
        .into_error()?;
        Ok(unsafe { plane.assume_init() })
    }

    pub fn finish_queue(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().finish_queue)(self.as_raw()).into_error() }
    }
    
    pub fn flush_queue(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().flush_queue)(self.as_raw()).into_error() }
    }
}

mod sealed {
    use std::ffi::c_void;

    pub trait Color {
        fn _as_ptr(&self) -> *const c_void;
    }

    impl Color for [u32; 4] {
        fn _as_ptr(&self) -> *const c_void {
            self.as_ptr() as _
        }
    }

    impl Color for [f32; 4] {
        fn _as_ptr(&self) -> *const c_void {
            self.as_ptr() as _
        }
    }
}

impl super::interface::sealed::Sealed for AMFCompute {}

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

#[repr(transparent)]
pub struct AMFPrograms(*mut *const AMFProgramsVtbl);

#[repr(C)]
pub struct AMFProgramsVtbl {
    register_kernel_source_file: stdcall!(
        fn(
            this: *mut *const Self,
            kernel_id: *mut u64,
            kernel_id_name: *const WideChar,
            kernel_name: *const c_char,
            file_path: *const WideChar,
            options: *const c_char,
        ) -> AMFResult
    ),
    register_kernel_source: stdcall!(
        fn(
            this: *mut *const Self,
            kernel_id: *mut u64,
            kernel_id_name: *const WideChar,
            kernel_name: *const c_char,
            data_size: isize,
            data: *const u8,
            options: *const c_char,
        ) -> AMFResult
    ),
    register_kernel_binary: stdcall!(
        fn(
            this: *mut *const Self,
            kernel_id: *mut u64,
            kernel_id_name: *const WideChar,
            kernel_name: *const c_char,
            data_size: isize,
            data: *const u8,
            options: *const c_char,
        ) -> AMFResult
    ),
    register_kernel_source_1: stdcall!(
        fn(
            this: *mut *const Self,
            memory_type: AMFMemoryType,
            kernel_id: *mut u64,
            kernel_id_name: *const WideChar,
            kernel_name: *const c_char,
            data_size: isize,
            data: *const u8,
            options: *const c_char,
        ) -> AMFResult
    ),
    register_kernel_binary_1: stdcall!(
        fn(
            this: *mut *const Self,
            memory_type: AMFMemoryType,
            kernel_id: *mut u64,
            kernel_id_name: *const WideChar,
            kernel_name: *const c_char,
            data_size: isize,
            data: *const u8,
            options: *const c_char,
        ) -> AMFResult
    ),
}

impl AMFPrograms {
    fn vtable(&self) -> &AMFProgramsVtbl {
        unsafe { &**self.0 }
    }

    fn as_raw(&self) -> *mut *const AMFProgramsVtbl {
        self.0
    }

    pub fn register_kernel_source_file(
        &self,
        kernel_id_name: &WideCStr,
        kernel_name: &CStr,
        file_path: &Path,
        options: &CStr,
    ) -> Result<u64, AMFError> {
        let path = widestring::WideCString::from_os_str(file_path.as_os_str()).unwrap();
        let mut id = 0;
        unsafe {
            (self.vtable().register_kernel_source_file)(
                self.as_raw(),
                &raw mut id,
                kernel_id_name.as_ptr(),
                kernel_name.as_ptr(),
                path.as_ptr(),
                options.as_ptr(),
            )
        }
        .into_error()?;
        Ok(id)
    }
    pub fn register_kernel_source(
        &self,
        kernel_id_name: &WideCStr,
        kernel_name: &CStr,
        data: &[u8],
        options: &CStr,
    ) -> Result<u64, AMFError> {
        let mut id = 0;
        unsafe {
            (self.vtable().register_kernel_source)(
                self.as_raw(),
                &raw mut id,
                kernel_id_name.as_ptr(),
                kernel_name.as_ptr(),
                data.len() as isize,
                data.as_ptr(),
                options.as_ptr(),
            )
        }
        .into_error()?;
        Ok(id)
    }
    pub fn register_kernel_binary(
        &self,
        kernel_id_name: &WideCStr,
        kernel_name: &CStr,
        data: &[u8],
        options: &CStr,
    ) -> Result<u64, AMFError> {
        let mut id = 0;
        unsafe {
            (self.vtable().register_kernel_binary)(
                self.as_raw(),
                &raw mut id,
                kernel_id_name.as_ptr(),
                kernel_name.as_ptr(),
                data.len() as isize,
                data.as_ptr(),
                options.as_ptr(),
            )
        }
        .into_error()?;
        Ok(id)
    }
    pub fn register_kernel_source_1(
        &self,
        memory_type: AMFMemoryType,
        kernel_id_name: &WideCStr,
        kernel_name: &CStr,
        data: &[u8],
        options: &CStr,
    ) -> Result<u64, AMFError> {
        let mut id = 0;
        unsafe {
            (self.vtable().register_kernel_source_1)(
                self.as_raw(),
                memory_type,
                &raw mut id,
                kernel_id_name.as_ptr(),
                kernel_name.as_ptr(),
                data.len() as isize,
                data.as_ptr(),
                options.as_ptr(),
            )
        }
        .into_error()?;
        Ok(id)
    }
    pub fn register_kernel_binary_1(
        &self,
        memory_type: AMFMemoryType,
        kernel_id_name: &WideCStr,
        kernel_name: &CStr,
        data: &[u8],
        options: &CStr,
    ) -> Result<u64, AMFError> {
        let mut id = 0;
        unsafe {
            (self.vtable().register_kernel_binary_1)(
                self.as_raw(),
                memory_type,
                &raw mut id,
                kernel_id_name.as_ptr(),
                kernel_name.as_ptr(),
                data.len() as isize,
                data.as_ptr(),
                options.as_ptr(),
            )
        }
        .into_error()?;
        Ok(id)
    }
}

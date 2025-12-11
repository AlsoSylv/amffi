use std::{
    ffi::{CStr, CString, c_char, c_void},
    ptr::null_mut,
};

#[cfg(windows)]
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D};

use crate::{
    core::{
        buffer::{AMFBuffer, AMFBufferObserver},
        compute::AMFCompute,
        data::{AMFDXVersion, AMFMemoryType},
        interface::{AMFInterface, Guid, Interface},
        property_storage::AMFPropertyStorageVtbl,
        result::{AMFError, AMFResult},
        surface::{AMFSurface, AMFSurfaceFormat, AMFSurfaceObserver},
        vulkan_amf::{AMFVulkanBuffer, AMFVulkanDevice, AMFVulkanSurface},
    },
    stdcall,
};

#[repr(C)]
#[derive(Default, Clone)]
pub struct AMFContext(<Self as std::ops::Deref>::Target);

unsafe impl Send for AMFContext {}

#[repr(C)]
pub struct AMFContextVtbl {
    base: AMFPropertyStorageVtbl,
    terminate: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_dx9: stdcall!(fn(this: *mut *const AMFContextVtbl, dx9device: *mut c_void) -> AMFResult),
    get_dx9_device:
        stdcall!(fn(this: *mut *const AMFContextVtbl, version: AMFDXVersion) -> *mut c_void),
    lock_dx9: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_dx9: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_dx11: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            dx11_device: *mut c_void,
            version: AMFDXVersion,
        ) -> AMFResult
    ),
    get_dx11_device:
        stdcall!(fn(this: *mut *const AMFContextVtbl, version: AMFDXVersion) -> *mut c_void),
    lock_dx11: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_dx11: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_opencl:
        stdcall!(fn(this: *mut *const AMFContextVtbl, command_queue: *mut c_void) -> AMFResult),
    get_opencl_context: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    get_opencl_command_queue: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    get_opencl_device_id: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    get_opencl_compute_factory:
        stdcall!(fn(this: *mut *const AMFContextVtbl, pp_factory: *mut *mut ()) -> AMFResult),
    init_opencl_ex: stdcall!(fn(this: *mut *const AMFContextVtbl, device: *mut ()) -> AMFResult),
    lock_opencl: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_opencl: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_opengl: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            opengl_context: *mut c_void,
            windows: *mut c_void,
            dc: *mut c_void,
        ) -> AMFResult
    ),
    get_opengl_context: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    get_opengl_drawable: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    lock_opengl: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_opengl: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_xv: stdcall!(fn(this: *mut *const AMFContextVtbl, xv_device: *mut c_void) -> AMFResult),
    get_xv_device: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    lock_xv: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_xv: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    init_gralloc:
        stdcall!(fn(this: *mut *const AMFContextVtbl, gralloc_device: *mut c_void) -> AMFResult),
    get_gralloc_device: stdcall!(fn(this: *mut *const AMFContextVtbl) -> *mut c_void),
    lock_gralloc: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    unlock_gralloc: stdcall!(fn(this: *mut *const AMFContextVtbl) -> AMFResult),
    alloc_buffer: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            ty: AMFMemoryType,
            size: isize,
            buffer: *mut AMFBuffer,
        ) -> AMFResult
    ),
    alloc_surface: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            ty: AMFMemoryType,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
            surface: *mut AMFSurface,
        ) -> AMFResult
    ),
    alloc_audio_buffer: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            ty: AMFMemoryType,
            format: isize,
            samples: i32,
            sample_rate: i32,
            channels: i32,
            audio_buffer: *mut *mut c_void,
        ) -> AMFResult
    ),
    create_buffer_from_host_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            host_buffer: *mut c_void,
            size: isize,
            buffer: *mut *mut AMFBuffer,
            observer: *mut AMFBufferObserver,
        ) -> AMFResult
    ),
    create_surface_from_host_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
            h_pitch: i32,
            v_pitch: i32,
            data: *mut c_void,
            surface: *mut *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_surface_from_dx9_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            dx9_surface: *mut c_void,
            surface: *mut *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_surface_from_dx11_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            dx11_surface: *mut c_void,
            surface: *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_surface_from_opengl_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            format: AMFSurfaceFormat,
            gl_texture_id: *mut c_void,
            surface: *mut *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_surface_from_gralloc_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            gralloc_surface: *mut c_void,
            surface: *mut *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_surface_from_opencl_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
            cl_places: *mut *mut c_void,
            surface: *mut *mut AMFSurface,
            observer: *mut AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_buffer_from_opencl_native: stdcall!(
        fn(
            this: *mut *const AMFContextVtbl,
            cl_buffer: *mut c_void,
            size: isize,
            buffer: *mut *mut AMFBuffer,
        ) -> AMFResult
    ),
    get_compute: stdcall!(
        fn(this: *mut *const Self, mem_type: AMFMemoryType, compute: *mut AMFCompute) -> AMFResult
    ),
}

impl AMFContext {
    pub fn terminate(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().terminate)(self.as_raw()) }.into_error()
    }

    #[cfg(windows)]
    pub fn init_dx11(
        &self,
        device: ID3D11Device,
        dx_version: AMFDXVersion,
    ) -> Result<(), AMFError> {
        use windows::core::Interface;

        unsafe {
            (self.vtable().init_dx11)(self.as_raw(), device.as_raw(), dx_version).into_error()
        }
    }

    /// # Safety
    /// `device` is not null
    /// `device` is ID3D11Device
    pub unsafe fn init_dx11_raw(
        &self,
        device: *mut c_void,
        dx_version: AMFDXVersion,
    ) -> Result<(), AMFError> {
        unsafe { (self.vtable().init_dx11)(self.as_raw(), device, dx_version).into_error() }
    }

    pub fn get_dx11_device(&self, dx_version: AMFDXVersion) -> *mut c_void {
        unsafe { (self.vtable().get_dx11_device)(self.as_raw(), dx_version) }
    }

    pub fn lock_dx11(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().lock_dx11)(self.as_raw()) }.into_error()
    }

    pub fn unlock_dx11(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().unlock_dx11)(self.as_raw()) }.into_error()
    }

    pub fn alloc_surface(
        &self,
        ty: AMFMemoryType,
        format: AMFSurfaceFormat,
        width: i32,
        height: i32,
    ) -> Result<AMFSurface, AMFError> {
        let mut surface = AMFSurface::default();
        unsafe {
            (self.vtable().alloc_surface)(
                self.as_raw(),
                ty,
                format,
                width,
                height,
                &raw mut surface,
            )
        }
        .into_error()?;
        Ok(surface)
    }

    #[cfg(windows)]
    pub fn create_surface_from_dx11_native(
        &self,
        buffer: &ID3D11Texture2D,
    ) -> Result<AMFSurface, AMFError> {
        use windows::core::Interface;
        let mut surface = AMFSurface::default();
        unsafe {
            (self.vtable().create_surface_from_dx11_native)(
                self.as_raw(),
                buffer.as_raw(),
                &raw mut surface,
                std::ptr::null_mut(),
            )
        }
        .into_error()?;
        Ok(surface)
    }

    pub fn get_compute(&self, memory_type: AMFMemoryType) -> Result<AMFCompute, AMFError> {
        let mut compute = AMFCompute::default();
        unsafe { (self.vtable().get_compute)(self.as_raw(), memory_type, &raw mut compute) }
            .into_error()?;
        Ok(compute)
    }
}

impl Interface for AMFContext {
    type Vtbl = AMFContextVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0xa76a13f0,
        0xd80e,
        0x4fcc,
        [0xb5, 0x8, 0x65, 0xd0, 0xb5, 0x2e, 0xd9, 0xee],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFContext {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct AMFContext1(AMFContext);

impl Interface for AMFContext1 {
    type Vtbl = AMFContext1Vtbl;

    const GUID: Guid = Guid::from_values(
        0xa76a13f0,
        0xd80e,
        0x4fcc,
        [0xb5, 0x8, 0x65, 0xd0, 0xb5, 0x2e, 0xd9, 0xee],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

#[repr(C)]
pub struct AMFContext1Vtbl {
    base: AMFContextVtbl,
    create_buffer_from_dx11_native: stdcall!(
        fn(
            this: *mut *const Self,
            host_buffer: *mut c_void,
            buffer: *mut AMFBuffer,
            observer: AMFBufferObserver,
        ) -> AMFResult
    ),
    alloc_buffer_ex: stdcall!(
        fn(
            this: *mut *const Self,
            ty: AMFMemoryType,
            size: isize,
            usage: u32,
            cpu_access: u32,
            buffer: *mut AMFBuffer,
        ) -> AMFResult
    ),
    alloc_surface_ex: stdcall!(
        fn(
            this: *mut *const Self,
            ty: AMFMemoryType,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
            usage: u32,
            cpu_access: u32,
            surface: *mut AMFSurface,
        ) -> AMFResult
    ),
    init_vulkan:
        stdcall!(fn(this: *mut *const Self, vulkan_device: *mut AMFVulkanDevice) -> AMFResult),
    get_vulkan_device: stdcall!(fn(this: *mut *const Self) -> *mut AMFVulkanDevice),
    lock_vulkan: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    unlock_vulkan: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    create_surface_from_vulkan_native: stdcall!(
        fn(
            this: *mut *const Self,
            vulkan_image: *mut c_void,
            surface: *mut AMFSurface,
            observer: AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_buffer_from_vulkan_native: stdcall!(
        fn(
            this: *mut *const Self,
            vulkan_buffer: *mut c_void,
            surface: *mut AMFBuffer,
            observer: AMFSurfaceObserver,
        ) -> AMFResult
    ),
    get_vulkan_device_extensions: stdcall!(
        fn(this: *mut *const Self, count: *mut isize, extensions: *mut *const c_char) -> AMFResult
    ),
}

impl AMFContext1 {    
    pub fn init_vulkan(&self, device: Option<&mut AMFVulkanDevice>) -> Result<(), AMFError> {
        let device = device.map(|d| d as *mut _).unwrap_or(null_mut());
        unsafe { (self.vtable().init_vulkan)(self.as_raw(), device) }.into_error()
    }

    pub fn get_vulkan_device(&self) -> AMFVulkanDevice {
        unsafe {
            (*((self.vtable().get_vulkan_device)(self.as_raw()) as *mut AMFVulkanDevice)).clone()
        }
    }

    pub fn lock_vulkan(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().lock_vulkan)(self.as_raw()) }.into_error()
    }

    pub fn unlock_vulkan(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().unlock_vulkan)(self.as_raw()) }.into_error()
    }

    pub fn create_surface_from_vulkan_native(
        &self,
        vulkan_surface: &mut AMFVulkanSurface,
    ) -> Result<AMFSurface, AMFError> {
        let mut surface = AMFSurface::default();
        unsafe {
            (self.vtable().create_surface_from_vulkan_native)(
                self.as_raw(),
                vulkan_surface as *mut _ as _,
                &raw mut surface,
                std::ptr::null_mut(),
            )
        }
        .into_error()?;
        Ok(surface)
    }

    pub fn create_buffer_from_vulkan_native(
        &self,
        vulkan_buffer: &mut AMFVulkanBuffer,
    ) -> Result<AMFBuffer, AMFError> {
        let mut surface = AMFBuffer::default();
        unsafe {
            (self.vtable().create_buffer_from_vulkan_native)(
                self.as_raw(),
                vulkan_buffer as *mut _ as _,
                &raw mut surface,
                std::ptr::null_mut(),
            )
        }
        .into_error()?;
        Ok(surface)
    }

    pub fn get_vulkan_device_extensions(&self) -> Result<Box<[CString]>, AMFError> {
        let mut count = 0;
        unsafe {
            (self.vtable().get_vulkan_device_extensions)(
                self.as_raw(),
                &raw mut count,
                std::ptr::null_mut(),
            )
        }
        .into_error()?;
        let mut vec = vec![std::ptr::null(); count as usize];
        unsafe {
            (self.vtable().get_vulkan_device_extensions)(
                self.as_raw(),
                &raw mut count,
                vec.as_mut_ptr(),
            )
        }
        .into_error()?;
        let vec = vec
            .into_iter()
            .map(|ptr| unsafe { CStr::from_ptr(ptr).to_owned() });
        Ok(vec.collect())
    }
}

impl std::ops::Deref for AMFContext1 {
    type Target = AMFContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
pub struct AMFContext2(AMFContext1);

impl Interface for AMFContext2 {
    type Vtbl = AMFContext2Vtbl;

    const GUID: Guid = Guid::from_values(
        0xd9e9f868,
        0x6220,
        0x44c6,
        [0xa2, 0x2f, 0x7c, 0xd6, 0xda, 0xc6, 0x86, 0x46],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

#[repr(C)]
pub struct AMFContext2Vtbl {
    base: AMFContext1Vtbl,
    init_dx12: stdcall!(
        fn(this: *mut *const Self, dx12_device: *mut c_void, dx_version: AMFDXVersion) -> AMFResult
    ),
    get_dx12_device: stdcall!(fn(this: *mut *const Self, dx_version: AMFDXVersion) -> *mut c_void),
    lock_dx12: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    unlock_dx12: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    create_surface_from_dx12_native: stdcall!(
        fn(
            this: *mut *const Self,
            resource: *mut c_void,
            surface: *mut AMFSurface,
            observer: AMFSurfaceObserver,
        ) -> AMFResult
    ),
    create_buffer_from_dx12_native: stdcall!(
        fn(
            this: *mut *const Self,
            resource: *mut c_void,
            surface: *mut AMFBuffer,
            observer: AMFBufferObserver,
        ) -> AMFResult
    ),
}

impl std::ops::Deref for AMFContext2 {
    type Target = AMFContext1;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

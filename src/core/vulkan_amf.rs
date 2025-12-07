use std::{ffi::c_void, ptr::null_mut};

use widestring::{WideCStr, widecstr};

pub const AMF_CONTEXT_VULKAN_COMPUTE_QUEUE: &WideCStr = widecstr!("VulkanComputeQueue");

#[repr(C)]
#[derive(Clone)]
pub struct AMFVulkanDevice {
    size_of: isize,
    pub next: *mut c_void,
    instance: *mut c_void,
    physical_device: *mut c_void,
    device: *mut c_void,
}

impl AMFVulkanDevice {
    #[cfg(feature = "ash")]
    pub fn from_ash(
        instance: ash::vk::Instance,
        physical_device: ash::vk::PhysicalDevice,
        device: ash::vk::Device,
    ) -> Self {
        // Safety: The ash types are all `repr(transparent)` and should be transmuted to avoid provence issues
        unsafe {
            Self::from_raw(
                std::mem::transmute(instance),
                std::mem::transmute(physical_device),
                std::mem::transmute(device),
            )
        }
    }

    pub unsafe fn from_raw(
        instance: *mut c_void,
        physical_device: *mut c_void,
        device: *mut c_void,
    ) -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            instance,
            physical_device,
            device,
        }
    }
}

#[repr(C)]
pub struct AMFVulkanSync {
    size_of: isize,
    next: *mut c_void,
    semaphore: u64,
    submitted: bool,
    fence: u64,
}

impl AMFVulkanSync {
    #[cfg(feature = "ash")]
    pub fn from_ash(semaphore: ash::vk::Semaphore, submitted: bool, fence: ash::vk::Fence) -> Self {
        use ash::vk::Handle;

        unsafe { Self::from_raw(semaphore.as_raw(), submitted, fence.as_raw()) }
    }

    /// # SAFETY
    /// `semaphore` and `fence` needs to be valid VKHandles
    pub unsafe fn from_raw(semaphore: u64, submitted: bool, fence: u64) -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            semaphore,
            submitted,
            fence,
        }
    }
}

#[repr(C)]
pub struct AMFVulkanBuffer {
    pub size_of: isize,
    pub next: *mut (),
    pub buffer: *mut (),
    pub memory: *mut (),
    pub size: i64,
    pub allocated_size: i64,
    pub access_flags: u32,
    pub usage: u32,
    pub access: u32,
    pub sync: AMFVulkanSync,
}

#[repr(C)]
pub struct AMFVulkanSurface {
    pub size_of: isize,
    pub next: *mut (),
    pub buffer: u64,
    pub memory: u64,
    pub size: i64,
    pub format: u32,
    pub width: i32,
    pub height: i32,
    pub current_layout: u32,
    pub usage: u32,
    pub access: u32,
    pub sync: AMFVulkanSync,
}

pub struct AMFVulkanSurfaceBuilder {
    inner: AMFVulkanSurface,
}

impl AMFVulkanSurfaceBuilder {
    #[cfg(feature = "ash")]
    pub fn ash_buffer(mut self, buffer: ash::vk::Buffer) -> Self {
        use ash::vk::Handle;

        self.inner.buffer = buffer.as_raw();
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_memory(mut self, memory: ash::vk::DeviceMemory) -> Self {
        use ash::vk::Handle;

        self.inner.memory = memory.as_raw();
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.inner.size = size;
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_format(mut self, format: ash::vk::Format) -> Self {
        self.inner.format = format.as_raw() as u32;
        self
    }

    pub fn width(mut self, width: i32) -> Self {
        self.inner.width = width;
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.inner.height = height;
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_current_layout(mut self, current_layout: ash::vk::ImageLayout) -> Self {
        self.inner.current_layout = current_layout.as_raw() as u32;
        self
    }

    pub fn usage(mut self, usage: u32) -> Self {
        self.inner.usage = usage;
        self
    }

    pub fn cpu_memory_access(mut self, access: u32) -> Self {
        self.inner.access = access;
        self
    }

    pub fn sync(mut self, amf_sync: AMFVulkanSync) -> Self {
        self.inner.sync = amf_sync;
        self
    }

    pub fn build(mut self) -> AMFVulkanSurface {
        self.inner.size_of = size_of::<AMFVulkanSurface>() as isize;
        self.inner
    }
}

#[repr(C)]
pub struct AMFVulkanSurface1 {
    pub size_of: isize,
    pub next: *mut (),
    pub tiling: u32,
}

#[repr(C)]
pub struct AMFVulkanView {
    pub size_of: isize,
    pub next: *mut (),
    pub surface: *mut AMFVulkanSurface,
    pub view: *mut (),
    pub plane_width: i32,
    pub plane_height: i32,
    pub plane_width_pitch: i32,
    pub plane_height_pitch: i32,
}

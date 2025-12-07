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
    pub buffer: *mut (),
    pub memory: *mut (),
    pub size: i64,
    pub format: u32,
    pub width: i32,
    pub height: i32,
    pub current_layout: u32,
    pub usage: u32,
    pub access: u32,
    pub sync: AMFVulkanSync,
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

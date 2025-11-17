use widestring::{WideCStr, widecstr};

pub const AMF_CONTEXT_VULKAN_COMPUTE_QUEUE: &WideCStr = widecstr!("VulkanComputeQueue");

#[repr(C)]
pub struct AMFVulkanDevice {
    pub size_of: isize,
    pub next: *mut (),
    pub instance: *mut (),
    pub physical_device: *mut (),
    pub device: *mut (),
}

#[repr(C)]
pub struct AMFVulkanSync {
    pub size_of: isize,
    pub next: *mut (),
    pub semaphore: *mut (),
    pub submitted: bool,
    pub fence: *mut (),
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

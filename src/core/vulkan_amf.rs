use std::{ffi::c_void, ptr::null_mut};

use widestring::{WideCStr, widecstr};

pub const AMF_CONTEXT_VULKAN_COMPUTE_QUEUE: &WideCStr = widecstr!("VulkanComputeQueue");
pub const AMF_CONTEXT_VULKAN_USE_TIMELINE_SEMAPHORES: &WideCStr =
    widecstr!("VulkanTimelineSemaphores");

#[repr(C)]
#[derive(Clone)]
pub struct AMFVulkanDevice {
    size_of: isize,
    next: *mut c_void,
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
    fn empty() -> Self {
        Self {
            size_of: size_of::<AMFVulkanSync>() as isize,
            next: null_mut(),
            semaphore: 0,
            submitted: false,
            fence: 0,
        }
    }

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
    size_of: isize,
    next: *mut (),
    buffer: u64,
    memory: u64,
    size: i64,
    allocated_size: i64,
    access_flags: u32,
    usage: u32,
    access: u32,
    sync: AMFVulkanSync,
}

impl AMFVulkanBuffer {
    pub fn builder() -> AMFVulkanBufferBuilder {
        AMFVulkanBufferBuilder::new()
    }

    fn empty() -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            buffer: 0,
            memory: 0,
            size: 0,
            allocated_size: 0,
            access_flags: 0,
            usage: 0,
            access: 0,
            sync: AMFVulkanSync::empty(),
        }
    }
}

pub struct AMFVulkanBufferBuilder {
    inner: AMFVulkanBuffer,
}

impl AMFVulkanBufferBuilder {
    pub fn new() -> Self {
        Self {
            inner: AMFVulkanBuffer::empty(),
        }
    }

    #[cfg(feature = "ash")]
    pub fn ash_buffer(mut self, buffer: ash::vk::Buffer) -> Self {
        use ash::vk::Handle;

        self.inner.buffer = buffer.as_raw();
        self
    }

    pub fn raw_buffer(mut self, buffer: u64) -> Self {
        self.inner.buffer = buffer;
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_memory(mut self, memory: ash::vk::DeviceMemory) -> Self {
        use ash::vk::Handle;

        self.inner.memory = memory.as_raw();
        self
    }

    pub fn raw_memory(mut self, memory: u64) -> Self {
        self.inner.memory = memory;
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.inner.size = size;
        self
    }

    pub fn allocated_size(mut self, size: i64) -> Self {
        self.inner.allocated_size = size;
        self
    }

    pub fn access_flags(mut self, access: u32) -> Self {
        self.inner.access_flags = access;
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

    pub fn build(self) -> AMFVulkanBuffer {
        self.inner
    }
}

#[repr(C)]
pub struct AMFVulkanSurface {
    size_of: isize,
    next: *mut (),
    buffer: u64,
    memory: u64,
    size: i64,
    format: u32,
    width: i32,
    height: i32,
    current_layout: u32,
    usage: u32,
    access: u32,
    sync: AMFVulkanSync,
}

impl AMFVulkanSurface {
    pub fn builder() -> AMFVulkanSurfaceBuilder {
        AMFVulkanSurfaceBuilder::new()
    }

    fn empty() -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            buffer: 0,
            memory: 0,
            size: 0,
            format: 0,
            width: 0,
            height: 0,
            current_layout: 0,
            usage: 0,
            access: 0,
            sync: AMFVulkanSync::empty(),
        }
    }
}

pub struct AMFVulkanSurfaceBuilder {
    inner: AMFVulkanSurface,
}

impl AMFVulkanSurfaceBuilder {
    pub fn new() -> Self {
        Self {
            inner: AMFVulkanSurface::empty(),
        }
    }

    #[cfg(feature = "ash")]
    pub fn ash_buffer(mut self, buffer: ash::vk::Buffer) -> Self {
        use ash::vk::Handle;

        self.inner.buffer = buffer.as_raw();
        self
    }

    pub fn raw_buffer(mut self, buffer: u64) -> Self {
        self.inner.buffer = buffer;
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_memory(mut self, memory: ash::vk::DeviceMemory) -> Self {
        use ash::vk::Handle;

        self.inner.memory = memory.as_raw();
        self
    }

    pub fn raw_memory(mut self, memory: u64) -> Self {
        self.inner.memory = memory;
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

    pub fn raw_format(mut self, format: u32) -> Self {
        self.inner.format = format;
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

    pub fn raw_current_layout(mut self, current_layout: u32) -> Self {
        self.inner.current_layout = current_layout;
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

    pub fn next(mut self, next: &mut AMFVulkanSurface1) -> Self {
        self.inner.next = next as *mut _ as _;
        self
    }

    pub fn build(self) -> AMFVulkanSurface {
        self.inner
    }
}

#[repr(C)]
pub struct AMFVulkanSurface1 {
    size_of: isize,
    next: *mut (),
    tiling: u32,
}

impl AMFVulkanSurface1 {
    #[cfg(feature = "ash")]
    pub fn ash_new(tiling: ash::vk::ImageTiling) -> Self {
        Self::raw_new(tiling.as_raw() as u32)
    }

    pub fn raw_new(tiling: u32) -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            tiling,
        }
    }
}

#[repr(C)]
pub struct AMFVulkanView {
    size_of: isize,
    next: *mut (),
    surface: *mut AMFVulkanSurface,
    view: u64,
    plane_width: i32,
    plane_height: i32,
    plane_width_pitch: i32,
    plane_height_pitch: i32,
}

impl AMFVulkanView {
    pub fn builder() -> AMFVulkanViewBuilder {
        AMFVulkanViewBuilder::new()
    }

    fn empty() -> Self {
        Self {
            size_of: size_of::<Self>() as isize,
            next: null_mut(),
            surface: null_mut(),
            view: 0,
            plane_width: 0,
            plane_height: 0,
            plane_width_pitch: 0,
            plane_height_pitch: 0,
        }
    }
}

pub struct AMFVulkanViewBuilder {
    inner: AMFVulkanView,
}

impl AMFVulkanViewBuilder {
    pub fn new() -> Self {
        Self {
            inner: AMFVulkanView::empty(),
        }
    }

    pub fn surface(mut self, surface: &mut AMFVulkanSurface) -> Self {
        self.inner.surface = surface;
        self
    }

    #[cfg(feature = "ash")]
    pub fn ash_view(mut self, view: ash::vk::ImageView) -> Self {
        use ash::vk::Handle;

        self.inner.view = view.as_raw();
        self
    }

    pub fn raw_view(mut self, view: u64) -> Self {
        self.inner.view = view;
        self
    }

    pub fn plane_width(mut self, plane_width: i32) -> Self {
        self.inner.plane_width = plane_width;
        self
    }

    pub fn plane_height(mut self, plane_height: i32) -> Self {
        self.inner.plane_height = plane_height;
        self
    }

    pub fn plane_width_pitch(mut self, plane_width_pitch: i32) -> Self {
        self.inner.plane_width_pitch = plane_width_pitch;
        self
    }

    pub fn plane_height_pitch(mut self, plane_height_pitch: i32) -> Self {
        self.inner.plane_height_pitch = plane_height_pitch;
        self
    }

    pub fn build(self) -> AMFVulkanView {
        self.inner
    }
}

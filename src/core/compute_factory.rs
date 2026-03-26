use std::ffi::c_void;

use widestring::{WideCStr, widecstr};

use crate::{
    core::{
        compute::AMFCompute,
        interface::{AMFInterface, AMFInterfaceVtbl, Interface},
        property_storage::{AMFPropertyStorage, AMFPropertyStorageVtbl},
        result::{AMFError, AMFResult},
    },
    stdcall,
};

pub const AMF_DEVICE_NAME: &WideCStr = widecstr!("DeviceName");
pub const AMF_DRIVER_VERSION_NAME: &WideCStr = widecstr!("DriverVersion");
pub const AMF_AUDIO_CONVOLUTION_MAX_STREAMS: &WideCStr = widecstr!("ConvolutionMaxStreams");
pub const AMF_AUDIO_CONVOLUTION_LENGTH: &WideCStr = widecstr!("ConvolutionLength");
pub const AMF_AUDIO_CONVOLUTION_BUFFER_SIZE: &WideCStr = widecstr!("ConvolutionBufferSize");
pub const AMF_AUDIO_CONVOLUTION_SAMPLE_RATE: &WideCStr = widecstr!("ConvolutionSampleRate");

#[derive(Clone)]
#[repr(transparent)]
pub struct AMFComputeDevice(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFComputeDeviceVtbl {
    base: AMFPropertyStorageVtbl,
    get_native_platform: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_native_device_id: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    get_native_context: stdcall!(fn(this: *mut *const Self) -> *mut c_void),
    create_compute: stdcall!(
        fn(this: *mut *const Self, rsvd: *mut c_void, compute: *mut AMFCompute) -> AMFResult
    ),
    create_compute_ex: stdcall!(
        fn(
            this: *mut *const Self,
            command_queue: *mut c_void,
            compute: *mut AMFCompute,
        ) -> AMFResult
    ),
}

impl AMFComputeDevice {
    pub fn get_native_platform(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native_platform)(self.as_raw()) }
    }

    pub fn get_native_device_id(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native_device_id)(self.as_raw()) }
    }

    pub fn get_native_context(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native_context)(self.as_raw()) }
    }

    pub fn create_compute(&self) -> Result<AMFCompute, AMFError> {
        let mut compute = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().create_compute)(
                self.as_raw(),
                std::ptr::null_mut(),
                compute.as_mut_ptr(),
            )
        }
        .into_error()?;
        Ok(unsafe { compute.assume_init() })
    }

    pub fn create_compute_ex(&self, command_queue: *mut c_void) -> Result<AMFCompute, AMFError> {
        let mut compute = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().create_compute_ex)(self.as_raw(), command_queue, compute.as_mut_ptr())
        }
        .into_error()?;
        Ok(unsafe { compute.assume_init() })
    }
}

impl super::interface::sealed::Sealed for AMFComputeDevice {}

impl Interface for AMFComputeDevice {
    type Vtbl = AMFComputeDeviceVtbl;

    const GUID: super::platform::Guid = super::platform::Guid::from_values(
        0xb79d7cf6,
        0x2c5c,
        0x4deb,
        [0xb8, 0x96, 0xa2, 0x9e, 0xbe, 0xa6, 0xe3, 0x97],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFComputeDevice {
    type Target = AMFPropertyStorage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFComputeFactory(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFComputeFactoryVtbl {
    base: AMFInterfaceVtbl,
    get_device_count: stdcall!(fn(this: *mut *const Self) -> i32),
    get_device_at:
        stdcall!(fn(this: *mut *const Self, idx: i32, device: *mut AMFComputeDevice) -> AMFResult),
}

impl AMFComputeFactory {
    pub fn get_device_count(&self) -> i32 {
        unsafe { (self.vtable().get_device_count)(self.as_raw()) }
    }

    pub fn get_device_at(&self, idx: i32) -> Result<AMFComputeDevice, AMFError> {
        let mut device = std::mem::MaybeUninit::uninit();
        unsafe { (self.vtable().get_device_at)(self.as_raw(), idx, device.as_mut_ptr()) }
            .into_error()?;
        Ok(unsafe { device.assume_init() })
    }
}

impl super::interface::sealed::Sealed for AMFComputeFactory {}

impl Interface for AMFComputeFactory {
    type Vtbl = AMFComputeFactoryVtbl;

    const GUID: super::platform::Guid = super::platform::Guid::from_values(
        0xe3c24bd7,
        0x2d83,
        0x416c,
        [0x8c, 0x4e, 0xfd, 0x13, 0xca, 0x86, 0xf4, 0xd0],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFComputeFactory {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

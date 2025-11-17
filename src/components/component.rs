use crate::{
    core::{
        buffer::AMFBuffer,
        context::AMFContext,
        data::{AMFData, AMFDataVtbl, AMFMemoryType},
        interface::{AMFInterface, AMFInterfaceVtbl, Guid, Interface},
        property_storage_ext::{AMFPropertyStorageEx, AMFPropertyStorageExVtbl},
        result::{AMFError, AMFResult},
        surface::{AMFSurface, AMFSurfaceFormat},
    },
    stdcall,
};

#[repr(C)]
pub struct AMFDataAllocatorCB(AMFInterface);

#[repr(C)]
pub struct AMFDataAllocatorCBVtbl {
    base: AMFInterfaceVtbl,
    alloc_buffer: stdcall!(
        fn(
            this: *mut AMFDataAllocatorCB,
            mem_ty: AMFMemoryType,
            size: isize,
            buffer: *mut *mut AMFBuffer,
        ) -> AMFResult
    ),
    alloc_surface: stdcall!(
        fn(
            this: *mut AMFDataAllocatorCB,
            mem_ty: AMFMemoryType,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
            pitch: i32,
            v_pitch: i32,
            surface: *mut *mut AMFSurface,
        ) -> AMFResult
    ),
}

impl Interface for AMFDataAllocatorCB {
    type Vtbl = AMFDataAllocatorCBVtbl;

    const GUID: Guid = Guid::from_values(
        0x4bf46198,
        0x8b7b,
        0x49d0,
        [0xaa, 0x72, 0x48, 0xd4, 0x7, 0xce, 0x24, 0xc5],
    );

    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

#[repr(C)]
pub struct AMFComponentOptimizationCallback {
    vtbl: *const AMFComponentOptimizationCallbackVtbl,
}

#[repr(C)]
pub struct AMFComponentOptimizationCallbackVtbl {
    on_component_optimization_progress:
        stdcall!(fn(this: *mut AMFComponentOptimizationCallback, percent: u32) -> AMFResult),
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct AMFComponent(AMFPropertyStorageEx);

unsafe impl Send for AMFComponent {}

#[repr(C)]
pub struct AMFComponentVtbl {
    base: AMFPropertyStorageExVtbl,
    init: stdcall!(
        fn(
            this: *mut *const AMFComponentVtbl,
            format: AMFSurfaceFormat,
            width: i32,
            height: i32,
        ) -> AMFResult
    ),
    re_init: stdcall!(fn(this: *mut *const AMFComponentVtbl, width: i32, height: i32) -> AMFResult),
    terminate: stdcall!(fn(this: *mut *const AMFComponentVtbl) -> AMFResult),
    drain: stdcall!(fn(this: *mut *const AMFComponentVtbl) -> AMFResult),
    flush: stdcall!(fn(this: *mut *const AMFComponentVtbl) -> AMFResult),
    submit_input: stdcall!(
        fn(this: *mut *const AMFComponentVtbl, data: *mut *const AMFDataVtbl) -> AMFResult
    ),
    query_output: stdcall!(fn(this: *mut *const AMFComponentVtbl, data: *mut AMFData) -> AMFResult),
    get_context: stdcall!(fn(this: *mut *const AMFComponentVtbl) -> AMFContext),
    // TODO: Figure out how to represent this in Rust
    set_output_data_allocator_cb: stdcall!(
        fn(this: *mut *const AMFComponentVtbl, callback: *mut AMFDataAllocatorCB) -> AMFResult
    ),
    get_caps: stdcall!(fn(this: *mut *const AMFComponentVtbl, *mut *mut AMFSurface) -> AMFResult),
    optimize: stdcall!(
        fn(
            this: *mut *const AMFComponentVtbl,
            callback: *mut AMFComponentOptimizationCallback,
        ) -> AMFResult
    ),
}

impl AMFComponent {
    pub fn init(&self, format: AMFSurfaceFormat, width: i32, height: i32) -> Result<(), AMFError> {
        unsafe { (self.vtable().init)(self.as_raw(), format, width, height) }.into_error()
    }

    pub fn re_init(&self, width: i32, height: i32) -> Result<(), AMFError> {
        unsafe { (self.vtable().re_init)(self.as_raw(), width, height) }.into_error()
    }

    pub fn terminate(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().terminate)(self.as_raw()) }.into_error()
    }

    pub fn drain(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().drain)(self.as_raw()) }.into_error()
    }

    pub fn flush(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().flush)(self.as_raw()) }.into_error()
    }

    pub fn submit_input(&self, data: &AMFData) -> Result<(), AMFError> {
        unsafe { (self.vtable().submit_input)(self.as_raw(), data.as_raw()) }.into_error()
    }

    pub fn query_output(&self) -> Result<AMFData, AMFError> {
        let mut data = AMFData::default();
        unsafe { (self.vtable().query_output)(self.as_raw(), &raw mut data) }.into_error()?;
        Ok(data)
    }

    pub fn get_context(&self) -> AMFContext {
        unsafe { (self.vtable().get_context)(self.as_raw()) }
    }
}

impl Interface for AMFComponent {
    type Vtbl = AMFComponentVtbl;

    const GUID: Guid = Guid::from_values(
        0x8b51e5e4,
        0x455d,
        0x4034,
        [0xa7, 0x46, 0xde, 0x1b, 0xed, 0xc3, 0xc4, 0x6],
    );

    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFComponent {
    type Target = AMFPropertyStorageEx;

    fn deref(&self) -> &AMFPropertyStorageEx {
        &self.0
    }
}

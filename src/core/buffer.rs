use std::ffi::c_void;

use crate::{
    core::{
        data::AMFDataVtbl,
        interface::{AMFInterface, Guid, Interface},
        result::{AMFError, AMFResult},
    },
    stdcall,
};

pub type AMFBufferObserver = *mut *const AMFBufferObserverVtbl;

#[repr(C)]
pub struct AMFBufferObserverVtbl {
    on_buffer_data_release: stdcall!(fn(this: AMFBufferObserver, buffer: AMFBuffer)),
}

pub trait BufferObserver {
    fn on_buffer_data_release(&mut self, buffer: AMFBuffer);
}

#[repr(C)]
pub(crate) struct InternalBufferObserver<T: BufferObserver> {
    vtbl: &'static AMFBufferObserverVtbl,
    this: T,
}

impl<T: BufferObserver> InternalBufferObserver<T> {
    pub(crate) fn new(observer: T) -> Self {
        Self {
            vtbl: &AMFBufferObserverVtbl {
                on_buffer_data_release: internal_observer::<T>,
            },
            this: observer,
        }
    }
}

stdcall! {
    fn internal_observer<T: BufferObserver>(this: *mut *const AMFBufferObserverVtbl, surface: AMFBuffer) {
        let this = unsafe { &mut *(this as *mut InternalBufferObserver<T>) };
        this.this.on_buffer_data_release(surface);
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct AMFBuffer(AMFInterface);

#[repr(C)]
pub struct AMFBufferVtbl {
    base: AMFDataVtbl,
    set_size: stdcall!(fn(this: *mut *const AMFBufferVtbl, new_size: isize) -> AMFResult),
    get_size: stdcall!(fn(this: *mut *const AMFBufferVtbl) -> isize),
    get_native: stdcall!(fn(this: *mut *const AMFBufferVtbl) -> *mut c_void),
    add_observer: stdcall!(fn(this: *mut *const AMFBufferVtbl, observer: *mut AMFBufferObserver)),
    remove_observer:
        stdcall!(fn(this: *mut *const AMFBufferVtbl, observer: *mut AMFBufferObserver)),
}

impl AMFBuffer {
    pub fn set_size(&self, new_size: isize) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_size)(self.as_raw(), new_size) }.into_error()
    }
    pub fn get_size(&self) -> isize {
        unsafe { (self.vtable().get_size)(self.as_raw()) }
    }
    pub fn get_native(&self) -> *mut c_void {
        unsafe { (self.vtable().get_native)(self.as_raw()) }
    }

    pub fn add_observer<T: BufferObserver>(&self, observer: T) -> BufferObserverHandle<T> {
        let internal_observer = Box::into_raw(Box::new(InternalBufferObserver::new(observer)));
        unsafe { (self.vtable().add_observer)(self.as_raw(), internal_observer as _) };
        BufferObserverHandle { ptr: internal_observer, buffer: self.clone() }
    }

    pub fn remove_observer<T: BufferObserver>(&self, handle: BufferObserverHandle<T>) {
        drop(handle)
    }
}

pub struct BufferObserverHandle<T: BufferObserver> {
    ptr: *mut InternalBufferObserver<T>,
    buffer: AMFBuffer,
}

impl<T: BufferObserver> Drop for BufferObserverHandle<T> {
    fn drop(&mut self) {
        unsafe { (self.buffer.vtable().remove_observer)(self.buffer.as_raw(), self.ptr as _) };
        unsafe {
            drop(Box::from_raw(self.ptr));
        };    
    }
}

impl Interface for AMFBuffer {
    type Vtbl = AMFBufferVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0xb04b7248,
        0xb6f0,
        0x4321,
        [0xb6, 0x91, 0xba, 0xa4, 0x74, 0xf, 0x9f, 0xcb],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFBuffer {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::{ffi::c_void, ptr::null_mut};

pub use crate::core::platform::Guid;

use crate::{
    core::result::{AMFError, AMFResult},
    stdcall,
};

#[repr(transparent)]
pub struct AMFInterface(*mut *const <Self as Interface>::Vtbl);

impl Default for AMFInterface {
    fn default() -> Self {
        Self(std::ptr::null_mut())    
    }
}

#[repr(C)]
pub struct AMFInterfaceVtbl {
    acquire: stdcall!(fn(this: *mut *const Self) -> u64),
    release: stdcall!(fn(this: *mut *const Self) -> u64),
    query_interface:
        stdcall!(fn(this: *mut *const Self, *const Guid, *mut *mut c_void) -> AMFResult),
}

impl AMFInterface {
    /// # Safety
    /// The underlying reference must be valid to drop at the end of the scope
    pub unsafe fn from_raw(raw: *mut *const <Self as Interface>::Vtbl) -> AMFInterface {
        AMFInterface(raw)
    }
}

impl Clone for AMFInterface {
    fn clone(&self) -> Self {
        unsafe { (self.vtable().acquire)(self.as_raw()) };
        Self(self.0)
    }
}

impl Drop for AMFInterface {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { (self.vtable().release)(self.as_raw()) };
        }
    }
}

pub trait Interface {
    type Vtbl;

    const GUID: Guid;

    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl;

    #[inline(always)]
    fn as_interface(&self) -> &AMFInterfaceVtbl {
        unsafe { &**self.as_raw_interface() }
    }

    #[inline(always)]
    fn cast<T>(&self) -> Result<T, AMFError>
    where
        T: Interface,
    {
        let mut new = null_mut();
        unsafe {
            (self.as_interface().query_interface)(self.as_raw_interface(), &T::GUID, &raw mut new)
        }
        .into_error()?;
        unsafe { Ok(std::mem::transmute_copy(&new)) }
    }

    #[inline(always)]
    fn as_raw(&self) -> *mut *const Self::Vtbl {
        self.as_raw_interface() as _
    }

    #[inline(always)]
    fn vtable(&self) -> &Self::Vtbl {
        unsafe { &**self.as_raw() }
    }
}

impl Interface for AMFInterface {
    type Vtbl = AMFInterfaceVtbl;

    const GUID: Guid = Guid::from_values(
        0x9d872f34,
        0x90dc,
        0x4b93,
        [0xb6, 0xb2, 0x6c, 0xa3, 0x7c, 0x85, 0x25, 0xdb],
    );

    #[inline(always)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl {
        self.0
    }
}

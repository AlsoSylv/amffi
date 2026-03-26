use std::{ffi::c_void, ptr::null_mut};

pub use crate::core::platform::Guid;

use crate::{
    core::{
        interface::sealed::Sealed,
        result::{AMFError, AMFResult},
    },
    stdcall,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// This structure contains a pointer to the AMFInterfaceVtbl
/// This is meant as a base struct or class in C++
/// This struct can be cast using `cast::<T>` which checks for validity
#[repr(transparent)]
pub struct AMFInterface(*mut *const <Self as Interface>::Vtbl);

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
        // If `self.0` is not null, the reference count can be decrimented
        if !self.0.is_null() {
            unsafe { (self.vtable().release)(self.as_raw()) };
        }
    }
}

pub trait Interface: Sealed + Clone {
    #[doc(hidden)]
    type Vtbl;

    #[doc(hidden)]
    const GUID: Guid;

    #[doc(hidden)]
    fn as_raw_interface(&self) -> *mut *const AMFInterfaceVtbl;

    #[inline(always)]
    #[doc(hidden)]
    fn as_raw(&self) -> *mut *const Self::Vtbl {
        self.as_raw_interface() as _
    }

    #[inline(always)]
    #[doc(hidden)]
    fn vtable(&self) -> &Self::Vtbl {
        unsafe { &**self.as_raw() }
    }

    #[inline(always)]
    /// Checked cast using `query_interface`, incriments the reference count
    fn cast<T>(&self) -> Result<T, AMFError>
    where
        T: Interface,
    {
        let mut new = null_mut();
        // Safety: The caller is a valid reference to `AMFInterface`, meaning it contains a NonNull pointer
        unsafe {
            let interface = &**self.as_raw_interface();
            (interface.query_interface)(self.as_raw_interface(), &T::GUID, &raw mut new)
        }
        .into_error()?;
        unsafe { Ok(std::mem::transmute_copy(&new)) }
    }
}

impl Sealed for AMFInterface {}

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

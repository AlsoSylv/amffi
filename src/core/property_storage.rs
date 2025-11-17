use widestring::{WideCStr, WideChar};

use crate::{
    core::{
        interface::{AMFInterface, AMFInterfaceVtbl, Guid, Interface},
        result::{AMFError, AMFResult},
        variant::{AMFVariant, AMFVariantStruct, AMFVariants},
    },
    stdcall,
};

#[repr(C)]
pub struct AMFPropertyStorageObserver {
    vtbl: *const AMFPropertyStorageObserverVtbl,
}

#[repr(C)]
pub struct AMFPropertyStorageObserverVtbl {
    on_property_changed: stdcall!(fn(this: *mut *const Self, name: *const u16)),
}

#[derive(Default, Clone)]
#[repr(transparent)]
pub struct AMFPropertyStorage(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFPropertyStorageVtbl {
    base: AMFInterfaceVtbl,
    set_property: stdcall!(
        fn(this: *mut *const Self, name: *const WideChar, value: AMFVariantStruct) -> AMFResult
    ),
    get_property: stdcall!(
        fn(
            this: *mut *const Self,
            name: *const WideChar,
            value: *mut AMFVariantStruct,
        ) -> AMFResult
    ),
    has_property: stdcall!(fn(this: *mut *const Self, name: *const WideChar) -> bool),
    get_property_count: stdcall!(fn(this: *mut *const Self) -> isize),
    get_property_at: stdcall!(
        fn(
            this: *mut *const Self,
            idx: isize,
            name: *mut WideChar,
            name_size: isize,
            value: *mut AMFVariantStruct,
        ) -> AMFResult
    ),
    clear: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    add_to: stdcall!(
        fn(this: *mut *const Self, dst: *mut *const Self, overwrite: bool, deep: bool) -> AMFResult
    ),
    copy_to: stdcall!(
        fn(this: *mut *const Self, dst: *mut *const Self, overwrite: bool, deep: bool) -> AMFResult
    ),
    add_observer: stdcall!(fn(this: *mut *const Self, observer: *mut AMFPropertyStorageObserver)),
    remove_observer:
        stdcall!(fn(this: *mut *const Self, observer: *mut AMFPropertyStorageObserver)),
}

impl AMFPropertyStorage {
    pub fn set_property(&self, name: &WideCStr, value: impl AMFVariant) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_property)(self.as_raw(), name.as_ptr(), value.to_variant()) }
            .into_error()
    }

    pub fn get_property(&self, name: &WideCStr) -> Result<AMFVariants<'_, AMFInterface>, AMFError> {
        let mut value = AMFVariantStruct::default();
        unsafe { (self.vtable().get_property)(self.as_raw(), name.as_ptr(), &raw mut value) }
            .into_error()?;
        Ok(From::from(value))
    }

    pub fn has_property(&self, name: &WideCStr) -> bool {
        unsafe { (self.vtable().has_property)(self.as_raw(), name.as_ptr()) }
    }

    pub fn get_property_count(&self) -> isize {
        unsafe { (self.vtable().get_property_count)(self.as_raw()) }
    }

    pub fn clear(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().clear)(self.as_raw()) }.into_error()
    }

    pub fn add_to(&self, dst: &Self, overwrite: bool, deep: bool) -> Result<(), AMFError> {
        unsafe { (self.vtable().add_to)(self.as_raw(), dst.as_raw(), overwrite, deep) }.into_error()
    }

    pub fn copy_to(&self, dst: &Self, overwrite: bool, deep: bool) -> Result<(), AMFError> {
        unsafe { (self.vtable().copy_to)(self.as_raw(), dst.as_raw(), overwrite, deep) }
            .into_error()
    }
}

impl Interface for AMFPropertyStorage {
    type Vtbl = AMFPropertyStorageVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0xc7cec05b,
        0xcfb9,
        0x48af,
        [0xac, 0xe3, 0xf6, 0x8d, 0xf8, 0x39, 0x5f, 0xe3],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFPropertyStorage {
    type Target = AMFInterface;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

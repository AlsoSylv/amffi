use widestring::{WideCStr, WideChar};

use crate::{
    core::{
        interface::{AMFInterface, Guid, Interface},
        property_storage::{AMFPropertyStorage, AMFPropertyStorageVtbl},
        result::{AMFError, AMFResult},
        variant::{AMFVariant, AMFVariantStruct, AMFVariantType, AMFVariants},
    },
    stdcall,
};

pub type AMFPropertyAccessType = isize;

pub const AMF_PROPERTY_ACCESS_PRIVATE: AMFPropertyAccessType = 0x0;
pub const AMF_PROPERTY_ACCESS_READ: AMFPropertyAccessType = 0x1;
pub const AMF_PROPERTY_ACCESS_WRITE: AMFPropertyAccessType = 0x2;
pub const AMF_PROPERTY_ACCESS_READ_WRITE: AMFPropertyAccessType = 0x3;
pub const AMF_PROPERTY_ACCESS_WRITE_RUNTIME: AMFPropertyAccessType = 0x4;
pub const AMF_PROPERTY_ACCESS_FULL: AMFPropertyAccessType = 0xFF;
pub const AMF_PROPERTY_ACCESS_NON_PERSISTENT: AMFPropertyAccessType = 0x4000;
pub const AMF_PROPERTY_ACCESS_NON_PERSISTENT_READ: AMFPropertyAccessType =
    AMF_PROPERTY_ACCESS_NON_PERSISTENT | AMF_PROPERTY_ACCESS_READ;
pub const AMF_PROPERTY_ACCESS_NON_PERSISTENT_READ_WRITE: AMFPropertyAccessType =
    AMF_PROPERTY_ACCESS_NON_PERSISTENT | AMF_PROPERTY_ACCESS_READ_WRITE;
pub const AMF_PROPERTY_ACCESS_NON_PERSISTENT_FULL: AMFPropertyAccessType =
    AMF_PROPERTY_ACCESS_NON_PERSISTENT | AMF_PROPERTY_ACCESS_FULL;
pub const AMF_PROPERTY_ACCESS_INVALID: AMFPropertyAccessType = 0x8000;

#[repr(C)]
pub struct AMFEnumDescriptionEntry {
    value: i32,
    name: *const u16,
}

#[repr(C)]
pub struct AMFPropertyInfo {
    name: *const u16,
    desc: *const u16,
    ty: AMFVariantType,
    content_type: u32,
    default: AMFVariantStruct,
    min: AMFVariantStruct,
    max: AMFVariantStruct,
    access_ty: AMFPropertyAccessType,
    enum_desc: AMFEnumDescriptionEntry,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFPropertyStorageEx(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFPropertyStorageExVtbl {
    base: AMFPropertyStorageVtbl,
    get_properties_info_count: stdcall!(fn(this: *mut *const Self) -> isize),
    get_property_info_at: stdcall!(
        fn(this: *mut *const Self, idx: isize, info: *mut *const AMFPropertyInfo) -> AMFResult
    ),
    get_property_info: stdcall!(
        fn(
            this: *mut *const Self,
            name: *const WideChar,
            info: *mut *const AMFPropertyInfo,
        ) -> AMFResult
    ),
    validate_property: stdcall!(
        fn(
            this: *mut *const Self,
            name: *const WideChar,
            value: AMFVariantStruct,
            out_validated: *mut AMFVariantStruct,
        ) -> AMFResult
    ),
}

impl AMFPropertyStorageEx {
    pub fn get_properties_info_count(&self) -> isize {
        unsafe { (self.vtable().get_properties_info_count)(self.as_raw()) }
    }

    pub fn get_property_info_at(&self, idx: isize) -> Result<&AMFPropertyInfo, AMFError> {
        let mut ptr = std::ptr::null();
        unsafe { (self.vtable().get_property_info_at)(self.as_raw(), idx, &raw mut ptr) }
            .into_error()?;

        Ok(unsafe { &*ptr })
    }

    pub fn get_property_info(&self, name: &WideCStr) -> Result<&AMFPropertyInfo, AMFError> {
        let mut ptr = std::ptr::null();
        unsafe { (self.vtable().get_property_info)(self.as_raw(), name.as_ptr(), &raw mut ptr) }
            .into_error()?;

        Ok(unsafe { &*ptr })
    }

    pub fn validate_property(
        &self,
        name: &WideCStr,
        value: impl AMFVariant,
    ) -> Result<AMFVariants<'_, AMFInterface>, AMFError> {
        let mut out = std::mem::MaybeUninit::uninit();
        unsafe {
            (self.vtable().validate_property)(
                self.as_raw(),
                name.as_ptr(),
                value.to_variant(),
                out.as_mut_ptr(),
            )
        }
        .into_error()?;

        Ok(unsafe { out.assume_init().into() })
    }
}

impl super::interface::sealed::Sealed for AMFPropertyStorageEx {}

impl Interface for AMFPropertyStorageEx {
    type Vtbl = AMFPropertyStorageExVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0x16b8958d,
        0xe943,
        0x4a33,
        [0xa3, 0x5a, 0x88, 0x5a, 0xd8, 0x28, 0xf2, 0x67],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFPropertyStorageEx {
    type Target = AMFPropertyStorage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

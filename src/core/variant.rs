//! TODO: Replace this with a better implementation entirely

use std::ffi::{CStr, c_char};

use widestring::{WideCStr, WideChar};

use crate::core::{
    interface::{AMFInterface, AMFInterfaceVtbl, Interface},
    platform::{
        AMFColor, AMFFloatPoint2D, AMFFloatPoint3D, AMFFloatSize, AMFFloatVector4D, AMFPoint,
        AMFRate, AMFRatio, AMFRect, AMFSize,
    },
};

#[repr(C)]
pub enum AMFVariantType {
    Empty = 0,
    Bool = 1,
    I64 = 2,
    Double = 3,
    Rect = 4,
    Size = 5,
    Point = 6,
    Rate = 7,
    Ratio = 8,
    Color = 9,
    /// `*mut c_char`
    String = 10,
    /// `*mut u16``
    WString = 11,
    Interface = 12,
    Float = 13,
    FloatSize = 14,
    FloatPoint2D = 15,
    FloatPoint3D = 16,
    FloatVector4D = 17,
}

#[repr(C)]
pub struct AMFVariantStruct {
    ty: AMFVariantType,
    value: AMFVariantValue,
}

impl Default for AMFVariantStruct {
    fn default() -> Self {
        Self {
            ty: AMFVariantType::Empty,
            value: AMFVariantValue { bool: false },
        }
    }
}

#[repr(C)]
pub union AMFVariantValue {
    bool: bool,
    i64: i64,
    double: f64,
    string: *const c_char,
    w_string: *const WideChar,
    interface: *mut *const AMFInterfaceVtbl,
    rect: AMFRect,
    size: AMFSize,
    point: AMFPoint,
    rate: AMFRate,
    ratio: AMFRatio,
    color: AMFColor,
    float: f32,
    float_size: AMFFloatSize,
    float_point_2d: AMFFloatPoint2D,
    float_point_3d: AMFFloatPoint3D,
    float_vector_4d: AMFFloatVector4D,
}

pub enum AMFVariants<'a, T: Interface> {
    Empty,
    Bool(bool),
    I64(i64),
    F64(f64),
    String(&'a CStr),
    WideString(&'a WideCStr),
    Interface(T),
    Rect(AMFRect),
    Size(AMFSize),
    Point(AMFPoint),
    Rate(AMFRate),
    Ratio(AMFRatio),
    Color(AMFColor),
    F32(f32),
    FloatSize(AMFFloatSize),
    FloatPoint(AMFFloatPoint2D),
    FLoatPoint3D(AMFFloatPoint3D),
    FloatVector4D(AMFFloatVector4D),
}

impl From<AMFVariantStruct> for AMFVariants<'_, AMFInterface> {
    fn from(value: AMFVariantStruct) -> Self {
        match value.ty {
            AMFVariantType::Empty => Self::Empty,
            AMFVariantType::Bool => Self::Bool(unsafe { value.value.bool }),
            AMFVariantType::I64 => Self::I64(unsafe { value.value.i64 }),
            AMFVariantType::Double => Self::F64(unsafe { value.value.double }),
            AMFVariantType::Rect => Self::Rect(unsafe { value.value.rect }),
            AMFVariantType::Size => Self::Size(unsafe { value.value.size }),
            AMFVariantType::Point => Self::Point(unsafe { value.value.point }),
            AMFVariantType::Rate => Self::Rate(unsafe { value.value.rate }),
            AMFVariantType::Ratio => Self::Ratio(unsafe { value.value.ratio }),
            AMFVariantType::Color => Self::Color(unsafe { value.value.color }),
            AMFVariantType::String => Self::String(unsafe { CStr::from_ptr(value.value.string) }),
            AMFVariantType::WString => {
                Self::WideString(unsafe { WideCStr::from_ptr_str(value.value.w_string) })
            }
            AMFVariantType::Interface => {
                Self::Interface(unsafe { AMFInterface::from_raw(value.value.interface) })
            }
            AMFVariantType::Float => AMFVariants::F32(unsafe { value.value.float }),
            AMFVariantType::FloatSize => AMFVariants::FloatSize(unsafe { value.value.float_size }),
            AMFVariantType::FloatPoint2D => {
                AMFVariants::FloatPoint(unsafe { value.value.float_point_2d })
            }
            AMFVariantType::FloatPoint3D => {
                AMFVariants::FLoatPoint3D(unsafe { value.value.float_point_3d })
            }
            AMFVariantType::FloatVector4D => {
                AMFVariants::FloatVector4D(unsafe { value.value.float_vector_4d })
            }
        }
    }
}

mod sealed {
    use std::ffi::CStr;

    use widestring::WideCStr;

    use crate::core::{
        interface::Interface,
        platform::{
            AMFColor, AMFFloatPoint2D, AMFFloatPoint3D, AMFFloatSize, AMFFloatVector4D, AMFPoint,
            AMFRate, AMFRatio, AMFRect, AMFSize,
        },
    };

    macro_rules! sealed {
        ($($type:ty),*) => {
            pub trait Sealed {}
            $(impl Sealed for $type {})*
        };
    }

    sealed! {
        bool, i64, f64, &CStr, &WideCStr, AMFRect, AMFSize, AMFPoint, AMFRate, AMFRatio, AMFColor, f32, AMFFloatSize, AMFFloatPoint2D, AMFFloatPoint3D, AMFFloatVector4D
    }

    impl<T: Interface> Sealed for &T {}
}

pub trait AMFVariant: sealed::Sealed {
    fn to_variant(self) -> AMFVariantStruct;
}

impl AMFVariant for bool {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Bool,
            value: AMFVariantValue { bool: self },
        }
    }
}

impl AMFVariant for i64 {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::I64,
            value: AMFVariantValue { i64: self },
        }
    }
}

impl AMFVariant for f64 {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Double,
            value: AMFVariantValue { double: self },
        }
    }
}

impl AMFVariant for &'_ CStr {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::String,
            value: AMFVariantValue {
                string: self.as_ptr(),
            },
        }
    }
}

impl AMFVariant for &'_ WideCStr {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::WString,
            value: AMFVariantValue {
                w_string: self.as_ptr(),
            },
        }
    }
}

impl<T: Interface> AMFVariant for &T {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Interface,
            value: AMFVariantValue {
                interface: self.as_raw_interface(),
            },
        }
    }
}

impl AMFVariant for AMFRect {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Rect,
            value: AMFVariantValue { rect: self },
        }
    }
}

impl AMFVariant for AMFSize {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Size,
            value: AMFVariantValue { size: self },
        }
    }
}

impl AMFVariant for AMFPoint {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Point,
            value: AMFVariantValue { point: self },
        }
    }
}

impl AMFVariant for AMFRate {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Rate,
            value: AMFVariantValue { rate: self },
        }
    }
}

impl AMFVariant for AMFRatio {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Ratio,
            value: AMFVariantValue { ratio: self },
        }
    }
}

impl AMFVariant for AMFColor {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Color,
            value: AMFVariantValue { color: self },
        }
    }
}

impl AMFVariant for f32 {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::Float,
            value: AMFVariantValue { float: self },
        }
    }
}

impl AMFVariant for AMFFloatSize {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::FloatSize,
            value: AMFVariantValue { float_size: self },
        }
    }
}

impl AMFVariant for AMFFloatPoint2D {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::FloatPoint2D,
            value: AMFVariantValue {
                float_point_2d: self,
            },
        }
    }
}

impl AMFVariant for AMFFloatPoint3D {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::FloatPoint3D,
            value: AMFVariantValue {
                float_point_3d: self,
            },
        }
    }
}

impl AMFVariant for AMFFloatVector4D {
    fn to_variant(self) -> AMFVariantStruct {
        AMFVariantStruct {
            ty: AMFVariantType::FloatVector4D,
            value: AMFVariantValue {
                float_vector_4d: self,
            },
        }
    }
}

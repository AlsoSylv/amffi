#[cfg(not(target_arch = "x86_64"))]
#[macro_export]
macro_rules! stdcall {
        (fn $args:tt $(-> $ret:tt)?) => { unsafe extern "stdcall" fn $args $(-> $ret)? };
        ($pub:vis $(unsafe $(@$is_unsafe:tt)?)? fn $($rest:tt)*) => {
            $pub $( unsafe $($is_unsafe)?)? extern "stdcall" fn $($rest)*
        };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! stdcall {
        (fn $args:tt $(-> $ret:ty)?) => { unsafe extern "C" fn $args $(-> $ret)? };
        ($pub:vis $(unsafe $(@$is_unsafe:tt)?)? fn $($rest:tt)*) => {
            $pub $( unsafe $($is_unsafe)?)? extern "C" fn $($rest)*
        };
}

#[cfg(not(target_arch = "x86_64"))]
#[macro_export]
macro_rules! cdecl {
        (fn $args:tt $(-> $ret:tt)?) => { unsafe extern "cdecl" fn $args $(-> $ret)? };
}

#[cfg(target_arch = "x86_64")]
#[macro_export]
macro_rules! cdecl {
        (fn $args:tt $(-> $ret:ty)?) => { unsafe extern "C" fn $args $(-> $ret)? };
}

#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: ((uuid >> 80) & 0xffff) as u16,
            data3: ((uuid >> 64) & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct AMFRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl AMFRect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct AMFSize {
    width: i32,
    height: i32,
}

impl AMFSize {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct AMFPoint {
    x: i32,
    y: i32,
}

impl AMFPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFFloatPoint2D {
    x: f32,
    y: f32,
}

impl AMFFloatPoint2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFFloatSize {
    width: f32,
    height: f32,
}

impl AMFFloatSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFFloatPoint3D {
    x: f32,
    y: f32,
    z: f32,
}

impl AMFFloatPoint3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFFloatVector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl AMFFloatVector4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFRate {
    num: u32,
    den: u32,
}

impl AMFRate {
    pub fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFRatio {
    num: u32,
    den: u32,
}

impl AMFRatio {
    pub fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }
}

#[derive(PartialEq, Clone, Copy)]
#[repr(C)]
pub struct AMFColor {
    rgba: u32,
}

impl AMFColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            rgba: u32::from_le_bytes([r, g, b, a]),
        }
    }

    pub fn from_u32(rgba: u32) -> Self {
        Self { rgba }
    }

    pub fn as_rgba(&self) -> [u8; 4] {
        self.rgba.to_be_bytes()
    }

    pub fn as_u32(&self) -> u32 {
        self.rgba
    }
}

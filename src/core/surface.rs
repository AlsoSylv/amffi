use std::mem::ManuallyDrop;

use crate::{
    core::{
        data::{AMFData, AMFDataVtbl},
        interface::{Guid, Interface},
        plane::{AMFPlane, AMFPlaneType},
        result::{AMFError, AMFResult},
    },
    stdcall,
};

#[derive(Debug)]
#[repr(C)]
pub enum AMFSurfaceFormat {
    Unknown = 0,
    Nv12,
    YV12,
    BGRA,
    ARGB,
    RGBA,
    GRAY8,
    YUV420P,
    U8V8,
    YUV2,
    P010,
    RGBAF16,
    UYVY,
    R10G10B10A2,
    Y210,
    AYUV,
    Y410,
    Y416,
    GRAY32,
    P012,
    P016,
}

#[repr(C)]
pub enum AMFFrameType {
    StereoFlag = 0x10000000,
    /// Also StereoLeft
    LeftFlag = AMFFrameType::StereoFlag as isize | 0x20000000,
    /// Also StereoRight
    RightFlag = AMFFrameType::StereoFlag as isize | 0x40000000,
    /// Also StereoBoth
    BothFlag = AMFFrameType::LeftFlag as isize | AMFFrameType::RightFlag as isize,
    InterleavedFlag = 0x01000000,
    FieldFlag = 0x02000000,
    EvenFlag = 0x04000000,
    OddFlag = 0x08000000,
    Unknown = -1,
    Progressive = 0,
    InterleavedEvenFirst = AMFFrameType::InterleavedFlag as isize | AMFFrameType::EvenFlag as isize,
    InterleavedOddFirst = AMFFrameType::InterleavedFlag as isize | AMFFrameType::OddFlag as isize,
    FieldSingleEven = AMFFrameType::FieldFlag as isize | AMFFrameType::EvenFlag as isize,
    FieldSingleOdd = AMFFrameType::FieldFlag as isize | AMFFrameType::OddFlag as isize,
    EvenFirstStereoLeft = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::EvenFlag as isize
        | AMFFrameType::LeftFlag as isize,
    EvenFirstStereoRight = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::EvenFlag as isize
        | AMFFrameType::RightFlag as isize,
    EvenFirstStereoBoth = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::EvenFlag as isize
        | AMFFrameType::BothFlag as isize,
    OddFirstStereoLeft = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::OddFlag as isize
        | AMFFrameType::LeftFlag as isize,
    OddFirstStereoRight = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::OddFlag as isize
        | AMFFrameType::RightFlag as isize,
    OddFirstStereoBoth = AMFFrameType::InterleavedFlag as isize
        | AMFFrameType::OddFlag as isize
        | AMFFrameType::BothFlag as isize,
}

impl AMFSurfaceFormat {
    pub fn first() -> Self {
        Self::Nv12
    }

    pub fn last() -> Self {
        Self::P016
    }
}

pub type AMFSurfaceObserver = *mut *const AMFSurfaceObserverVtbl;

#[repr(C)]
pub struct AMFSurfaceObserverVtbl {
    on_surface_data_release: stdcall!(fn(this: *mut *const Self, surface: ManuallyDrop<AMFSurface>)),
}

#[repr(C)]
pub(crate) struct InternalSurfaceObserver<T: SurfaceObserver> {
    vtbl: &'static AMFSurfaceObserverVtbl,
    this: T,
}

impl<T: SurfaceObserver> InternalSurfaceObserver<T> {
    pub(crate) fn new(observer: T) -> Self {
        Self {
            vtbl: &AMFSurfaceObserverVtbl {
                on_surface_data_release: internal_observer::<T>,
            },
            this: observer,
        }
    }
}

pub trait SurfaceObserver {
    fn on_surface_data_release(&mut self, surface: ManuallyDrop<AMFSurface>);
}


stdcall! {
    fn internal_observer<T: SurfaceObserver>(this: *mut *const AMFSurfaceObserverVtbl, surface: ManuallyDrop<AMFSurface>) {
        let this = unsafe { &mut *(this as *mut InternalSurfaceObserver<T>) };
        this.this.on_surface_data_release(surface);
    }
}

impl<T> SurfaceObserver for T
where
    T: FnMut(ManuallyDrop<AMFSurface>),
{
    fn on_surface_data_release(&mut self, surface: ManuallyDrop<AMFSurface>) {
        self(surface)
    }
}

#[test]
#[ignore = "This is a compile test only"]
fn test_function_observer() {
    {
        let mut captures = false;
        let observer = |_: ManuallyDrop<AMFSurface>| {
            captures = true;
        };
        InternalSurfaceObserver::new(observer);
    }

    {
        fn observer(_: ManuallyDrop<AMFSurface>) {}
        InternalSurfaceObserver::new(observer);
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct AMFSurface(<Self as std::ops::Deref>::Target);

#[repr(C)]
pub struct AMFSurfaceVtbl {
    base: AMFDataVtbl,
    get_format: stdcall!(fn(this: *mut *const Self) -> AMFSurfaceFormat),
    get_plane_count: stdcall!(fn(this: *mut *const Self) -> isize),
    get_plane_at: stdcall!(fn(this: *mut *const Self, idx: isize) -> ManuallyDrop<AMFPlane>),
    get_plane: stdcall!(fn(this: *mut *const Self, ty: AMFPlaneType) -> ManuallyDrop<AMFPlane>),
    get_frame_type: stdcall!(fn(this: *mut *const Self) -> AMFFrameType),
    set_frame_type: stdcall!(fn(this: *mut *const Self, ty: AMFFrameType)),
    set_crop:
        stdcall!(fn(this: *mut *const Self, x: i32, y: i32, width: i32, height: i32) -> AMFResult),
    copy_surface_region: stdcall!(
        fn(
            this: *mut *const Self,
            dst: *mut *const Self,
            dst_x: i32,
            dst_y: i32,
            src_x: i32,
            src_y: i32,
            width: i32,
            height: i32,
        ) -> AMFResult
    ),
    add_observer: stdcall!(fn(this: *mut *const Self, observer: AMFSurfaceObserver)),
    remove_observer: stdcall!(fn(this: *mut *const Self, observer: AMFSurfaceObserver)),
}

impl AMFSurface {
    pub fn get_format(&self) -> AMFSurfaceFormat {
        unsafe { (self.vtable().get_format)(self.as_raw()) }
    }

    pub fn get_plane_count(&self) -> isize {
        unsafe { (self.vtable().get_plane_count)(self.as_raw()) }
    }

    pub fn get_plane_at(&self, idx: isize) -> ManuallyDrop<AMFPlane> {
        unsafe { (self.vtable().get_plane_at)(self.as_raw(), idx) }
    }

    pub fn get_plane(&self, ty: AMFPlaneType) -> ManuallyDrop<AMFPlane> {
        unsafe { (self.vtable().get_plane)(self.as_raw(), ty) }
    }

    pub fn get_frame_type(&self) -> AMFFrameType {
        unsafe { (self.vtable().get_frame_type)(self.as_raw()) }
    }

    pub fn set_frame_type(&self, ty: AMFFrameType) {
        unsafe { (self.vtable().set_frame_type)(self.as_raw(), ty) }
    }

    pub fn set_crop(&self, x: i32, y: i32, width: i32, height: i32) -> Result<(), AMFError> {
        unsafe { (self.vtable().set_crop)(self.as_raw(), x, y, width, height) }.into_error()
    }

    pub fn copy_surface_region(
        &self,
        dst: &mut AMFSurface,
        dst_x: i32,
        dst_y: i32,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), AMFError> {
        unsafe {
            (self.vtable().copy_surface_region)(
                self.as_raw(),
                dst.as_raw(),
                dst_x,
                dst_y,
                src_x,
                src_y,
                width,
                height,
            )
        }
        .into_error()
    }

    pub fn add_observer<T: SurfaceObserver>(&self, observer: T) -> SurfaceObserverHandle<T> {
        let internal_observer = Box::into_raw(Box::new(InternalSurfaceObserver::new(observer)));
        unsafe { (self.vtable().add_observer)(self.as_raw(), internal_observer as _) };
        SurfaceObserverHandle { ptr: internal_observer, surface: self.clone() }
    }

    pub fn remove_observer<T: SurfaceObserver>(&self, handle: SurfaceObserverHandle<T>) {
        drop(handle)
    }
}

pub struct SurfaceObserverHandle<T: SurfaceObserver> {
    ptr: *mut InternalSurfaceObserver<T>,
    surface: AMFSurface,
}

impl<T: SurfaceObserver> Drop for SurfaceObserverHandle<T> {
    fn drop(&mut self) {
        // Safety: `*mut InternalSurfaceObserver<T>` is always `*mut *const AMFSurfaceObserverVtbl`
        unsafe { (self.surface.vtable().remove_observer)(self.surface.as_raw(), self.ptr as _) };
        // Safety: This was allocated as a Box and then leaked
        unsafe {
            drop(Box::from_raw(self.ptr));
        };
    }
}

impl Interface for AMFSurface {
    type Vtbl = AMFSurfaceVtbl;

    const GUID: super::interface::Guid = Guid::from_values(
        0x4bf46198,
        0x8b7b,
        0x49d0,
        [0xaa, 0x72, 0x48, 0xd4, 0x7, 0xce, 0x24, 0xc5],
    );

    fn as_raw_interface(&self) -> *mut *const super::interface::AMFInterfaceVtbl {
        self.0.as_raw_interface()
    }
}

impl std::ops::Deref for AMFSurface {
    type Target = AMFData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::stdcall;

#[repr(C)]
pub struct AMFDebug(*mut *const AMFDebugVtbl);

#[repr(C)]
pub struct AMFDebugVtbl {
    enable_performance_monitor: stdcall!(fn(this: *mut *const AMFDebugVtbl, enable: bool)),
    performance_monitor_enabled: stdcall!(fn(this: *mut *const AMFDebugVtbl) -> bool),
    asserts_enable: stdcall!(fn(this: *mut *const AMFDebugVtbl, enable: bool)),
    asserts_enabled: stdcall!(fn(this: *mut *const AMFDebugVtbl) -> bool),
}

impl AMFDebug {
    pub fn enable_performance_monitor(&self, enable: bool) {
        unsafe { (self.vtable().enable_performance_monitor)(self.as_raw(), enable) }
    }
    pub fn performance_monitor_enabled(&self) -> bool {
        unsafe { (self.vtable().performance_monitor_enabled)(self.as_raw()) }
    }
    pub fn asserts_enable(&self, enable: bool) {
        unsafe { (self.vtable().asserts_enable)(self.as_raw(), enable) }
    }
    pub fn asserts_enabled(&self) -> bool {
        unsafe { (self.vtable().asserts_enabled)(self.as_raw()) }
    }

    unsafe fn vtable(&self) -> &AMFDebugVtbl {
        unsafe { &**self.as_raw() }
    }

    unsafe fn as_raw(&self) -> *mut *const AMFDebugVtbl {
        self.0
    }
}

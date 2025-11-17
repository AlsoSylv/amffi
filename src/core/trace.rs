//! TODO: Expose traces

use std::ffi::c_void;

use widestring::{WideCStr, WideChar, widecstr};

use crate::{core::result::AMFResult, stdcall};

#[repr(i32)]
#[derive(Default)]
pub enum AMFTraceLevel {
    Error = 0,
    Warning = 1,
    #[default]
    Info = 2,
    Debug = 3,
    Trace = 4,
    Test = 5,
    NoLog = 100,
}

pub const AMF_TRACE_WRITER_CONSOLE: &WideCStr = widecstr!("Console");
pub const AMF_TRACE_WRITER_DEBUG_OUTPUT: &WideCStr = widecstr!("DebugOutput");
pub const AMF_TRACE_WRITER_FILE: &WideCStr = widecstr!("File");

#[repr(transparent)]
pub struct AMFTrace(*mut *const AMFTraceVtbl);

#[repr(C)]
pub struct AMFTraceVtbl {
    trace_w: stdcall!(
        fn(
            this: *mut *const Self,
            src_path: *const WideChar,
            line: i32,
            level: i32,
            scope: *const WideChar,
            count_args: i32,
            format: *const WideChar,
            ...
        )
    ),
    trace: stdcall!(
        fn(
            this: *mut *const Self,
            src_path: *const WideChar,
            line: i32,
            level: i32,
            scope: *const WideChar,
            message: *const WideChar,
            va_list: *mut *mut c_void,
        )
    ),
    set_global_level: stdcall!(fn(this: *mut *const Self, level: i32) -> i32),
    get_global_level: stdcall!(fn(this: *mut *const Self) -> i32),
    enable_writer: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, enable: bool) -> bool),
    writer_enabled: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar) -> bool),
    trace_enable_async: stdcall!(fn(this: *mut *const Self, enable: bool) -> AMFResult),
    trace_flush: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    set_path: stdcall!(fn(this: *mut *const Self, path: *const WideChar) -> AMFResult),
    get_path: stdcall!(fn(this: *mut *const Self, path: *mut WideChar, size: *mut isize) -> AMFResult),
    set_writer_level: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, level: i32) -> i32),
    get_writer_level: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar) -> i32),
    set_writer_level_for_scope: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, scope: *const WideChar, level: i32) -> i32),
    get_writer_level_for_scope: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, scope: *const WideChar) -> i32),
}

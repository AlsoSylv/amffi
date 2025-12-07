//! TODO: Expose traces

use std::{
    ffi::c_void,
    panic::Location,
    path::{Path, PathBuf},
    ptr::null_mut,
};

use widestring::{WideCStr, WideCString, WideChar, error::ContainsNul, widecstr};

use crate::{
    core::{
        data::AMFMemoryType,
        result::{AMFError, AMFResult},
        surface::AMFSurfaceFormat,
    },
    stdcall,
};

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

impl TryFrom<i32> for AMFTraceLevel {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, i32> {
        match value {
            value if value == AMFTraceLevel::Error as i32 => Ok(AMFTraceLevel::Error),
            value if value == AMFTraceLevel::Warning as i32 => Ok(AMFTraceLevel::Warning),
            value if value == AMFTraceLevel::Info as i32 => Ok(AMFTraceLevel::Info),
            value if value == AMFTraceLevel::Debug as i32 => Ok(AMFTraceLevel::Debug),
            value if value == AMFTraceLevel::Trace as i32 => Ok(AMFTraceLevel::Trace),
            value if value == AMFTraceLevel::Test as i32 => Ok(AMFTraceLevel::Test),
            value if value == AMFTraceLevel::NoLog as i32 => Ok(AMFTraceLevel::NoLog),
            invalid => Err(invalid),
        }
    }
}

pub struct AMFTraceWriterConsole;

impl TraceWriter for AMFTraceWriterConsole {
    const NAME: &WideCStr = widecstr!("Console");
}

pub struct AMFTraceWriterDebugOutput;

impl TraceWriter for AMFTraceWriterDebugOutput {
    const NAME: &WideCStr = widecstr!("DebugOutput");
}

pub struct AMFTraceWriterFile;

impl TraceWriter for AMFTraceWriterFile {
    const NAME: &WideCStr = widecstr!("File");
}

#[repr(C)]
pub struct AMFTraceWriterVtbl {
    write: stdcall!(fn(this: *mut *const Self, scope: *mut WideChar, message: *mut WideChar)),
    flush: stdcall!(fn(this: *mut *const Self)),
}

#[repr(C)]
pub(crate) struct InternalTraceWriter<T: CustomTraceWriter> {
    vtbl: &'static AMFTraceWriterVtbl,
    this: T,
}

impl<T: CustomTraceWriter> InternalTraceWriter<T> {
    pub(crate) fn new(writer: T) -> Self {
        Self {
            vtbl: &AMFTraceWriterVtbl {
                write: internal_write::<T>,
                flush: internal_flush::<T>,
            },
            this: writer,
        }
    }
}

pub trait TraceWriter {
    const NAME: &WideCStr;
}

pub trait CustomTraceWriter: TraceWriter {
    fn write(&mut self, scope: &WideCStr, message: &WideCStr);

    fn flush(&mut self);
}

stdcall! {
    unsafe fn internal_write<T: CustomTraceWriter>(this: *mut *const AMFTraceWriterVtbl, scope: *mut WideChar, message: *mut WideChar) {
        let this = unsafe { &mut *(this as *mut InternalTraceWriter<T>) };
        let scope = unsafe { WideCStr::from_ptr_str(scope) };
        let message = unsafe { WideCStr::from_ptr_str(message) };
        this.this.write(scope, message);
    }
}

stdcall! {
    unsafe fn internal_flush<T: CustomTraceWriter>(this: *mut *const AMFTraceWriterVtbl) {
        let this = unsafe { &mut *(this as *mut InternalTraceWriter<T>) };
        this.this.flush();
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct AMFTrace(*mut *const AMFTraceVtbl);

impl Default for AMFTrace {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

#[repr(C)]
pub struct AMFTraceVtbl {
    // TODO: To wrap this properly, a VaList is required
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
    enable_writer:
        stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, enable: bool) -> bool),
    writer_enabled: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar) -> bool),
    // TODO: Find documentation before exposing
    trace_enable_async: stdcall!(fn(this: *mut *const Self, enable: bool) -> AMFResult),
    trace_flush: stdcall!(fn(this: *mut *const Self) -> AMFResult),
    // TODO: Wrap path in `File` somehow
    set_path: stdcall!(fn(this: *mut *const Self, path: *const WideChar) -> AMFResult),
    get_path:
        stdcall!(fn(this: *mut *const Self, path: *mut WideChar, size: *mut isize) -> AMFResult),
    set_writer_level:
        stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar, level: i32) -> i32),
    get_writer_level: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar) -> i32),
    set_writer_level_for_scope: stdcall!(
        fn(
            this: *mut *const Self,
            writer_id: *const WideChar,
            scope: *const WideChar,
            level: i32,
        ) -> i32
    ),
    get_writer_level_for_scope: stdcall!(
        fn(this: *mut *const Self, writer_id: *const WideChar, scope: *const WideChar) -> i32
    ),
    get_indentation: stdcall!(fn(this: *mut *const Self) -> i32),
    indent: stdcall!(fn(this: *mut *const Self, add_indent: i32)),
    register_writer: stdcall!(
        fn(
            this: *mut *const Self,
            writer_id: *const WideChar,
            writer: *mut *const AMFTraceWriterVtbl,
            enabled: bool,
        )
    ),
    unregister_writer: stdcall!(fn(this: *mut *const Self, writer_id: *const WideChar)),
    get_result_text: stdcall!(fn(this: *mut *const Self, res: AMFResult) -> *const WideChar),
    surface_get_format_name:
        stdcall!(fn(this: *mut *const Self, surface_format: AMFSurfaceFormat) -> *const WideChar),
    surface_get_format_by_name:
        stdcall!(fn(this: *mut *const Self, name: *const WideChar) -> AMFSurfaceFormat),
    get_memory_type_name:
        stdcall!(fn(this: *mut *const Self, mem_ty: AMFMemoryType) -> *const WideChar),
    get_memory_type_by_name:
        stdcall!(fn(this: *mut *const Self, name: *const WideChar) -> AMFMemoryType),
    // TODO: AudioBuffer bindings, isize is being used in place of `AMFAudioFormat`
    get_sample_format_name:
        stdcall!(fn(this: *mut *const Self, audio_format: isize) -> *const WideChar),
    get_sample_format_by_name: stdcall!(fn(this: *mut *const Self, name: *const WideChar) -> isize),
}

impl AMFTrace {
    #[inline(always)]
    unsafe fn as_raw(&self) -> *mut *const AMFTraceVtbl {
        self.0
    }

    #[inline(always)]
    unsafe fn vtable(&self) -> &AMFTraceVtbl {
        unsafe { &**self.as_raw() }
    }

    #[track_caller]
    /// # Errors
    /// Returns an error if `scope`, `message`, or the file name contain `null`
    pub fn trace_w(
        &self,
        level: AMFTraceLevel,
        scope: &'static str,
        message: String,
    ) -> Result<(), ContainsNul<WideChar>> {
        let caller = Location::caller();
        let file = WideCString::from_str(caller.file())?;
        let line = caller.line();
        let scope = WideCString::from_str(scope)?;
        let message = WideCString::from_str(message)?;
        unsafe {
            (self.vtable().trace_w)(
                self.as_raw(),
                file.as_ptr(),
                line as i32,
                level as i32,
                scope.as_ptr(),
                0,
                message.as_ptr(),
            )
        }
        Ok(())
    }

    pub fn set_writer_enabled<T: TraceWriter>(&self, enabled: bool) -> bool {
        unsafe { (self.vtable().enable_writer)(self.as_raw(), T::NAME.as_ptr(), enabled) }
    }

    pub fn writer_enabled<T: TraceWriter>(&self) -> bool {
        unsafe { (self.vtable().writer_enabled)(self.as_raw(), T::NAME.as_ptr()) }
    }

    pub fn trace_flush(&self) -> Result<(), AMFError> {
        unsafe { (self.vtable().trace_flush)(self.as_raw()).into_error() }
    }

    pub fn set_path(&self, path: &Path) -> Result<(), AMFError> {
        let wide_path = WideCString::from_os_str(path).unwrap();
        unsafe { (self.vtable().set_path)(self.as_raw(), wide_path.as_ptr()).into_error() }
    }

    pub fn get_path(&self) -> Result<PathBuf, AMFError> {
        let mut size = 0;
        unsafe { (self.vtable().get_path)(self.as_raw(), null_mut(), &raw mut size).into_error() }?;
        let mut buffer = vec![0; size as usize];
        unsafe {
            (self.vtable().get_path)(self.as_raw(), buffer.as_mut_ptr(), &raw mut size).into_error()
        }?;
        Ok(WideCString::from_vec(buffer).unwrap().to_os_string().into())
    }

    pub fn set_global_level(&self, level: AMFTraceLevel) -> AMFTraceLevel {
        unsafe {
            (self.vtable().set_global_level)(self.as_raw(), level as i32)
                .try_into()
                .expect("SetGlobalLevel returns a valid trace level")
        }
    }

    pub fn get_global_level(&self) -> AMFTraceLevel {
        unsafe {
            (self.vtable().get_global_level)(self.as_raw())
                .try_into()
                .expect("GetGlobalLevel returns a valid trace level")
        }
    }

    pub fn set_writer_level<T: TraceWriter>(&self, level: AMFTraceLevel) -> AMFTraceLevel {
        unsafe {
            (self.vtable().set_writer_level)(self.as_raw(), T::NAME.as_ptr(), level as i32)
                .try_into()
                .expect("SetGlobalLevel returns a valid trace level")
        }
    }

    pub fn get_writer_level<T: TraceWriter>(&self) -> AMFTraceLevel {
        unsafe {
            (self.vtable().get_writer_level)(self.as_raw(), T::NAME.as_ptr())
                .try_into()
                .expect("GetGlobalLevel returns a valid trace level")
        }
    }

    pub fn set_writer_level_per_scope<T: TraceWriter>(
        &self,
        scope: &WideCStr,
        level: AMFTraceLevel,
    ) -> AMFTraceLevel {
        unsafe {
            (self.vtable().set_writer_level_for_scope)(
                self.as_raw(),
                T::NAME.as_ptr(),
                scope.as_ptr(),
                level as i32,
            )
            .try_into()
            .expect("SetGlobalLevel returns a valid trace level")
        }
    }

    pub fn get_writer_leve_per_scopel<T: TraceWriter>(&self, scope: &WideCStr) -> AMFTraceLevel {
        unsafe {
            (self.vtable().get_writer_level_for_scope)(
                self.as_raw(),
                T::NAME.as_ptr(),
                scope.as_ptr(),
            )
            .try_into()
            .expect("GetGlobalLevel returns a valid trace level")
        }
    }

    pub fn get_indentation(&self) -> i32 {
        unsafe { (self.vtable().get_indentation)(self.as_raw()) }
    }

    pub fn add_indentation(&self, to_add: i32) {
        unsafe { (self.vtable().indent)(self.as_raw(), to_add) }
    }

    pub fn register_writer<T: CustomTraceWriter>(
        &self,
        writer: T,
        enabled: bool,
    ) -> TraceWriterHandle<T> {
        let internal_writer = Box::into_raw(Box::new(InternalTraceWriter::new(writer)));
        unsafe {
            (self.vtable().register_writer)(
                self.as_raw(),
                T::NAME.as_ptr(),
                internal_writer as _,
                enabled,
            )
        };
        TraceWriterHandle {
            ptr: internal_writer,
            name: T::NAME.to_ucstring(),
            trace: self.clone(),
        }
    }

    pub fn get_error_text(&self, err: AMFError) -> &WideCStr {
        unsafe {
            WideCStr::from_ptr_str((self.vtable().get_result_text)(
                self.as_raw(),
                err.into_result(),
            ))
        }
    }

    pub fn surface_get_format_name(&self, surface_format: AMFSurfaceFormat) -> &WideCStr {
        unsafe {
            WideCStr::from_ptr_str((self.vtable().surface_get_format_name)(
                self.as_raw(),
                surface_format,
            ))
        }
    }

    pub fn surface_get_format_by_name(&self, name: &WideCStr) -> AMFSurfaceFormat {
        unsafe { (self.vtable().surface_get_format_by_name)(self.as_raw(), name.as_ptr()) }
    }

    pub fn get_memory_type_name(&self, memory_type: AMFMemoryType) -> &WideCStr {
        unsafe {
            WideCStr::from_ptr_str((self.vtable().get_memory_type_name)(
                self.as_raw(),
                memory_type,
            ))
        }
    }

    pub fn get_memory_type_by_name(&self, name: &WideCStr) -> AMFMemoryType {
        unsafe { (self.vtable().get_memory_type_by_name)(self.as_raw(), name.as_ptr()) }
    }
}

pub struct TraceWriterHandle<T: CustomTraceWriter> {
    ptr: *mut InternalTraceWriter<T>,
    name: WideCString,
    trace: AMFTrace,
}

impl<T: CustomTraceWriter> Drop for TraceWriterHandle<T> {
    fn drop(&mut self) {
        unsafe { (self.trace.vtable().unregister_writer)(self.trace.as_raw(), self.name.as_ptr()) }
        unsafe { drop(Box::from_raw(self.ptr)) }
    }
}

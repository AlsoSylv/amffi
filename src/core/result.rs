#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum AMFResult {
    Ok = 0,
    Fail,

    /// common errors
    Unexpected,

    AccessDenied,
    InvalidArg,
    OutOfRange,

    OutOfMemory,
    InvalidPointer,

    NoInterface,
    NotImplemented,
    NotSupported,
    NotFound,

    AlreadyInitialized,
    NotInitialized,

    InvalidFormat, // invalid data format

    WrongState,
    FileNotOpen, // cannot open file

    // device common codes
    NoDevice,

    // device directx
    DirectxFailed,
    // device opencl
    OpenCLFailed,
    // device opengl
    GLXFailed, //failed to use GLX
    // device XV
    XVFailed, //failed to use Xv extension
    // device alsa
    ALSAFailed, //failed to use ALSA

    // component common codes

    //result codes
    Eof,
    Repeat,
    InputFull,         //returned by AMFComponent::SubmitInput if input queue is full
    ResolutionChanged, //resolution changed client needs to Drain/Terminate/Init
    ResolutionUpdated, //resolution changed in adaptive mode. New ROI will be set on output on newly decoded frames

    //error codes
    InvalidDataType,           //invalid data type
    InvalidResolution,         //invalid resolution (width or height)
    CodecNotSupported,         //codec not supported
    SurfaceFormatNotSupported, //surface format not supported
    SurfaceMustBeShared, //surface should be shared (DX11: (MiscFlags & D3D11_RESOURCE_MISC_SHARED) == 0, DX9: No shared handle found)

    // component video decoder
    DecoderNotPresent,              //failed to create the decoder
    DecoderSurfaceAllocationFailed, //failed to create the surface for decoding
    DecoderNoFreeSurfaces,

    // component video encoder
    EncoderNotPresent, //failed to create the encoder

    // component video processor

    // component video converter

    // component dem
    DEMError,
    DEMPropertyReadOnly,
    DEMRemoteDisplayCreateFailed,
    DEMStartEncodingFailed,
    DEMQueryOutputFailed,

    // component TAN
    TanClippyWasRequired, // Resulting data was truncated to meet output type's value limits.
    TanUnsupportedVersion, // Not supported version requested, solely for TANCreateContext().

    NeedMoreInput, //returned by AMFComponent::SubmitInput did not produce a buffer because more input submissions are required.

    // device vulkan
    VulkanFailed,
}

impl AMFResult {
    pub fn into_error(self) -> Result<(), AMFError> {
        AMFError::from_result(self)
    }
}

impl AMFError {
    pub fn from_result(value: AMFResult) -> Result<(), Self> {
        if value == AMFResult::Ok {
            Ok(())
        } else {
            // Safety: They are one case different, so they always align
            unsafe { Err(std::mem::transmute::<AMFResult, AMFError>(value)) }
        }
    }

    pub(crate) fn into_result(self) -> AMFResult {
        unsafe { std::mem::transmute(self) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum AMFError {
    Fail = AMFResult::Fail as isize,

    /// common errors
    Unexpected,

    AccessDenied,
    InvalidArg,
    OutOfRange,

    OutOfMemory,
    InvalidPointer,

    NoInterface,
    NotImplemented,
    NotSupported,
    NotFound,

    AlreadyInitialized,
    NotInitialized,

    InvalidFormat, // invalid data format

    WrongState,
    FileNotOpen, // cannot open file

    // device common codes
    NoDevice,

    // device directx
    DirectxFailed,
    // device opencl
    OpenCLFailed,
    // device opengl
    GLXFailed, //failed to use GLX
    // device XV
    XVFailed, //failed to use Xv extension
    // device alsa
    ALSAFailed, //failed to use ALSA

    // component common codes

    //result codes
    Eof,
    Repeat,
    InputFull,         //returned by AMFComponent::SubmitInput if input queue is full
    ResolutionChanged, //resolution changed client needs to Drain/Terminate/Init
    ResolutionUpdated, //resolution changed in adaptive mode. New ROI will be set on output on newly decoded frames

    //error codes
    InvalidDataType,           //invalid data type
    InvalidResolution,         //invalid resolution (width or height)
    CodecNotSupported,         //codec not supported
    SurfaceFormatNotSupported, //surface format not supported
    SurfaceMustBeShared, //surface should be shared (DX11: (MiscFlags & D3D11_RESOURCE_MISC_SHARED) == 0, DX9: No shared handle found)

    // component video decoder
    DecoderNotPresent,              //failed to create the decoder
    DecoderSurfaceAllocationFailed, //failed to create the surface for decoding
    DecoderNoFreeSurfaces,

    // component video encoder
    EncoderNotPresent, //failed to create the encoder

    // component video processor

    // component video converter

    // component dem
    DEMError,
    DEMPropertyReadOnly,
    DEMRemoteDisplayCreateFailed,
    DEMStartEncodingFailed,
    DEMQueryOutputFailed,

    // component TAN
    TanClippyWasRequired, // Resulting data was truncated to meet output type's value limits.
    TanUnsupportedVersion, // Not supported version requested, solely for TANCreateContext().

    NeedMoreInput, //returned by AMFComponent::SubmitInput did not produce a buffer because more input submissions are required.

    // device vulkan
    VulkanFailed,
}

impl core::fmt::Display for AMFError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for AMFError {}

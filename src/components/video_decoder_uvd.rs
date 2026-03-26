use widestring::{WideCStr, widecstr};

pub const AMF_VIDEO_DECODER_UVD_MPEG2: &WideCStr = widecstr!("AMFVideoDecoderUVD_MPEG2");
pub const AMF_VIDEO_DECODER_UVD_MPEG4: &WideCStr = widecstr!("AMFVideoDecoderUVD_MPEG4");
pub const AMF_VIDEO_DECODER_UVD_WMV3: &WideCStr = widecstr!("AMFVideoDecoderUVD_WMV3");
pub const AMF_VIDEO_DECODER_UVD_VC1: &WideCStr = widecstr!("AMFVideoDecoderUVD_VC1");
pub const AMF_VIDEO_DECODER_UVD_H264_AVC: &WideCStr = widecstr!("AMFVideoDecoderUVD_H264_AVC");
pub const AMF_VIDEO_DECODER_UVD_H264_MVC: &WideCStr = widecstr!("AMFVideoDecoderUVD_H264_MVC");
pub const AMF_VIDEO_DECODER_UVD_H264_SVC: &WideCStr = widecstr!("AMFVideoDecoderUVD_H264_SVC");
pub const AMF_VIDEO_DECODER_UVD_MJPEG: &WideCStr = widecstr!("AMFVideoDecoderUVD_MJPEG");
pub const AMF_VIDEO_DECODER_HW_H265_HEVC: &WideCStr = widecstr!("AMFVideoDecoderHW_H265_HEVC");
#[deprecated(note = "use AMF_VIDEO_DECODER_HW_H265_HEVC instead")]
pub const AMF_VIDEO_DECODER_HW_H265_MAIN10: &WideCStr = widecstr!("AMFVideoDecoderHW_H265_MAIN10");
pub const AMF_VIDEO_DECODER_HW_VP9: &WideCStr = widecstr!("AMFVideoDecoderHW_VP9");
#[deprecated(note = "use AMF_VIDEO_DECODER_HW_VP9 instead")]
pub const AMF_VIDEO_DECODER_HW_VP9_10BIT: &WideCStr = widecstr!("AMFVideoDecoderHW_VP9_10BIT");
pub const AMF_VIDEO_DECODER_HW_AV1: &WideCStr = widecstr!("AMFVideoDecoderHW_AV1");
#[deprecated(note = "use AMF_VIDEO_DECODER_HW_AV1 instead")]
pub const AMF_VIDEO_DECODER_HW_AV1_12BIT: &WideCStr = widecstr!("AMFVideoDecoderHW_AV1_12BIT");

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AMFVideocDecoderMode {
    Regular = 0,
    Compliant,
    LowLatency,
}

#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AMFTimestampMode {
    #[default]
    Presentation = 0,
    TsSort,
    TsDecode,
}

pub const AMF_VIDEO_DECODER_SURFACE_COPY: &WideCStr = widecstr!("SurfaceCopy");
pub const AMF_VIDEO_DECODER_EXTRADATA: &WideCStr = widecstr!("ExtraData");
pub const AMF_VIDEO_DECODER_FRAME_RATE: &WideCStr = widecstr!("FrameRate");
pub const AMF_TIMESTAMP_MODE: &WideCStr = widecstr!("TimestampMode");

pub const AMF_VIDEO_DECODER_ADAPTIVE_RESOLUTION_CHANGE: &WideCStr =
    widecstr!("AdaptiveResolutionChange");
pub const AMF_VIDEO_DECODER_ALLOC_SIZE: &WideCStr = widecstr!("AllocSize");
pub const AMF_VIDEO_DECODER_CURRENT_SIZE: &WideCStr = widecstr!("CurrentSize");

pub const AMF_VIDEO_DECODER_REORDER_MODE: &WideCStr = widecstr!("ReorderMode");
pub const AMF_VIDEO_DECODER_SURFACE_POOL_SIZE: &WideCStr = widecstr!("SurfacePoolSize");
pub const AMF_VIDEO_DECODER_DPB_SIZE: &WideCStr = widecstr!("DPBSize");

pub const AMF_VIDEO_DECODER_DEFAULT_SURFACES_FOR_TRANSIT: usize = 5;

pub const AMF_VIDEO_DECODER_CAP_NUM_OF_STREAMS: &WideCStr = widecstr!("NumOfStreams");

pub const AMF_VIDEO_DECODER_COLOR_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("ColorTransferChar");
pub const AMF_VIDEO_DECODER_COLOR_PRIMARIES: &WideCStr = widecstr!("ColorPrimaries");
pub const AMF_VIDEO_DECODER_HDR_METADATA: &WideCStr = widecstr!("HdrMetadata");

#[deprecated(note = "use AMF_VIDEO_DECODER_COLOR_RANGE instead")]
pub const AMF_VIDEO_DECODER_FULL_RANGE_COLOR: &WideCStr = widecstr!("FullRangeColor");

pub const AMF_VIDEO_DECODER_COLOR_RANGE: &WideCStr = widecstr!("ColorRange");

pub const AMF_VIDEO_DECODER_COLOR_PROFILE: &WideCStr = widecstr!("ColorProfile");

pub const AMF_VIDEO_DECODER_OUTPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("OutColorTransferChar");
pub const AMF_VIDEO_DECODER_OUTPUT_COLOR_PRIMARIES: &WideCStr = widecstr!("OutputColorPrimaries");
pub const AMF_VIDEO_DECODER_OUTPUT_HDR_METADATA: &WideCStr = widecstr!("OutHDRMetadata");

pub const AMF_VIDEO_DECODER_LOW_LATENCY: &WideCStr = widecstr!("LowLatencyDecode");

#[cfg(target_os = "android")]
pub const AMF_VIDEO_DECODER_NATIVEWINDOW: &WideCStr = widecstr!("AndroidNativeWindow");

#[cfg(target_os = "macos")]
pub const AMF_VIDEO_DECODER_NATIVEWINDOW: &WideCStr = widecstr!("AppleNativeWindow");

pub const AMF_VIDEO_DECODER_ENABLE_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("EnableDecoderSmartAccessVideo");
pub const AMF_VIDEO_DECODER_SKIP_TRANSFER_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("SkipTransferSmartAccessVideo");
pub const AMF_VIDEO_DECODER_OUTPUT_FORMAT: &WideCStr = widecstr!("OutputDecodeFormat");

pub const AMF_VIDEO_DECODER_CAP_SUPPORT_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("SupportSmartAccessVideo");

pub const AMF_VIDEO_DECODER_SURFACE_CPU: &WideCStr = widecstr!("SurfaceCpu");

pub const AMF_VIDEO_DECODER_INSTANCE_INDEX: &WideCStr = widecstr!("DecoderInstance");
pub const AMF_VIDEO_DECODER_CAP_NUM_OF_HW_INSTANCES: &WideCStr =
    widecstr!("NumOfHwDecoderInstances");

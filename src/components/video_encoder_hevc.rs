use widestring::{WideCStr, widecstr};

pub const AMF_VIDEO_ENCODER_HEVC: &WideCStr = widecstr!("AMFVideoEncoderHW_HEVC");

#[repr(C)]
pub enum AMFVideoEncoderHevcUsage {
    Transcoding = 0,
    UltraLowLatency,
    LowLatency,
    Webcam,
    HighQuality,
    LowLatencyHighQuality,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcProfile {
    Main = 1,
    Main10 = 2,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcTier {
    Main = 0,
    High = 1,
}

#[repr(C)]
pub enum AMFVideoEncoderLevel {
    _1 = 30,
    _2 = 60,
    _2_1 = 63,
    _3 = 90,
    _3_1 = 93,
    _4 = 120,
    _4_1 = 123,
    _5 = 150,
    _5_1 = 153,
    _5_2 = 156,
    _6 = 180,
    _6_1 = 183,
    _6_2 = 186,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcRateControlMethod {
    Unknown = -1,
    ConstantQp = 0,
    LatencyConstrainedVbr,
    PeakConstrainedVbr,
    Cbr,
    QualityVbr,
    HighQualityVbr,
    HighQualityCbr,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcPictureType {
    None = 0,
    Skip,
    Idr,
    I,
    P,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcDataType {
    Idr,
    I,
    P,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcQualityPreset {
    Quality = 0,
    Balanced = 5,
    Speed = 10,
    HighQuality = 15,
}

#[repr(C)]
pub enum AMFVideoHeaderInsertionMode {
    None = 0,
    GopAligned,
    IdrAligned,
    Suppressed,
}

#[repr(C)]
pub enum AMFVideoEncoderPictureTransferMode {
    Off = 0,
    On,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcNominalRange {
    Studio = 0,
    Full = 1,
}

#[repr(C)]
pub enum AMFVideoEncoderLtrMode {
    ResetUnused = 0,
    KeepUnused,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcOutputMode {
    Frame,
    Slice,
}

#[repr(C)]
pub enum AMFVideoEncoderHevcOutputBufferType {
    Frame = 0,
    Slice = 1,
    SliceLast = 2,
}

pub const AMF_VIDEO_ENCODER_HEVC_INSTANCE_INDEX: &WideCStr = widecstr!("HevcEncoderInstance");
pub const AMF_VIDEO_ENCODER_HEVC_FRAMESIZE: &WideCStr = widecstr!("HevcFrameSize");

pub const AMF_VIDEO_ENCODER_HEVC_USAGE: &WideCStr = widecstr!("HevcUsage");
pub const AMF_VIDEO_ENCODER_HEVC_PROFILE: &WideCStr = widecstr!("HevcProfile");
pub const AMF_VIDEO_ENCODER_HEVC_TIER: &WideCStr = widecstr!("HevcTier");
pub const AMF_VIDEO_ENCODER_HEVC_PROFILE_LEVEL: &WideCStr = widecstr!("HevcProfileLevel");
pub const AMF_VIDEO_ENCODER_HEVC_MAX_LTR_FRAMES: &WideCStr = widecstr!("HevcMaxOfLTRFrames");
pub const AMF_VIDEO_ENCODER_HEVC_LTR_MODE: &WideCStr = widecstr!("HevcLTRMode");
pub const AMF_VIDEO_ENCODER_HEVC_MAX_NUM_REFRAMES: &WideCStr = widecstr!("HevcMaxNumRefFrames");
pub const AMF_VIDEO_ENCODER_HEVC_QUALITY_PRESET: &WideCStr = widecstr!("HevcQualityPreset");
pub const AMF_VIDEO_ENCODER_HEVC_EXTRADATA: &WideCStr = widecstr!("HevcExtraData");
pub const AMF_VIDEO_ENCODER_HEVC_ASPECT_RATIO: &WideCStr = widecstr!("HevcAspectRatio");
pub const AMF_VIDEO_ENCODER_HEVC_LOWLATENCY_MODE: &WideCStr = widecstr!("LowLatencyInternal");
pub const AMF_VIDEO_ENCODER_HEVC_PRE_ANALYSIS_ENABLE: &WideCStr =
    widecstr!("HevcEnablePreAnalysis");
pub const AMF_VIDEO_ENCODER_HEVC_INPUT_FULL_RANGE_COLOR: &WideCStr =
    widecstr!("HevcInputFullRangeColor");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_FULL_RANGE_COLOR: &WideCStr = widecstr!("HevcNominalRange");
pub const AMF_VIDEO_ENCODER_HEVC_NOMINAL_RANGE: &WideCStr = widecstr!("HevcNominalRange");
pub const AMF_VIDEO_ENCODER_HEVC_MAX_NUM_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("HevcMaxNumOfTemporalLayers");

pub const AMF_VIDEO_ENCODER_HEVC_NUM_GOPS_PER_IDR: &WideCStr = widecstr!("HevcGOPSPerIDR");
pub const AMF_VIDEO_ENCODER_HEVC_GOP_SIZE: &WideCStr = widecstr!("HevcGOPSize");
pub const AMF_VIDEO_ENCODER_HEVC_DE_BLOCKING_FILTER_DISABLE: &WideCStr =
    widecstr!("HevcDeBlockingFilter");
pub const AMF_VIDEO_ENCODER_HEVC_SLICES_PER_FRAME: &WideCStr = widecstr!("HevcSlicesPerFrame");
pub const AMF_VIDEO_ENCODER_HEVC_HEADER_INSERTION_MODE: &WideCStr =
    widecstr!("HevcHeaderInsertionMode");
pub const AMF_VIDEO_ENCODER_HEVC_INTRA_REFRESH_NUM_CTBS_PER_SLOT: &WideCStr =
    widecstr!("HevcIntraRefreshCTBsNumberPerSlot");

pub const AMF_VIDEO_ENCODER_HEVC_RATE_CONTROL_METHOD: &WideCStr =
    widecstr!("HevcRateControlMethod");
pub const AMF_VIDEO_ENCODER_HEVC_QVBR_QUALITY_LEVEL: &WideCStr = widecstr!("HevcQvbrQualityLevel");
pub const AMF_VIDEO_ENCODER_HEVC_VBV_BUFFER_SIZE: &WideCStr = widecstr!("HevcVBVBufferSize");
pub const AMF_VIDEO_ENCODER_HEVC_INITIAL_VBV_BUFFER_FULLNESS: &WideCStr =
    widecstr!("HevcInitialVBVBufferFullness");
pub const AMF_VIDEO_ENCODER_HEVC_ENABLE_VBAQ: &WideCStr = widecstr!("HevcEnableVBAQ");
pub const AMF_VIDEO_ENCODER_HEVC_HIGH_MOTION_QUALITY_BOOST_ENABLE: &WideCStr =
    widecstr!("HevcHighMotionQualityBoostEnable");

pub const AMF_VIDEO_ENCODER_HEVC_PREENCODE_ENABLE: &WideCStr =
    widecstr!("HevcRateControlPreAnalysisEnable");
pub const AMF_VIDEO_ENCODER_HEVC_RATE_CONTROL_PREANALYSIS_ENABLE: &WideCStr =
    widecstr!("HevcRateControlPreAnalysisEnable");

pub const AMF_VIDEO_ENCODER_HEVC_MOTION_HALF_PIXEL: &WideCStr = widecstr!("HevcHalfPixel");
pub const AMF_VIDEO_ENCODER_HEVC_MOTION_QUARTERPIXEL: &WideCStr = widecstr!("HevcQuarterPixel");

pub const AMF_VIDEO_ENCODER_HEVC_COLOR_BIT_DEPTH: &WideCStr = widecstr!("HevcColorBitDepth");

pub const AMF_VIDEO_ENCODER_HEVC_INPUT_COLOR_PROFILE: &WideCStr = widecstr!("HevcInColorProfile");
pub const AMF_VIDEO_ENCODER_HEVC_INPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("HevcInColorTransferChar");
pub const AMF_VIDEO_ENCODER_HEVC_INPUT_COLOR_PRIMARIES: &WideCStr =
    widecstr!("HevcInColorPrimaries");
pub const AMF_VIDEO_ENCODER_HEVC_INPUT_MATRIX_COEFF: &WideCStr = widecstr!("HevcInMatrixCoeff");

pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_COLOR_PROFILE: &WideCStr = widecstr!("HevcOutColorProfile");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("HevcOutColorTransferChar");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_COLOR_PRIMARIES: &WideCStr =
    widecstr!("HevcOutColorPrimaries");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_MATRIX_COEFF: &WideCStr = widecstr!("HevcOutMatrixCoeff");

pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_MODE: &WideCStr = widecstr!("HevcOutputMode");

pub const AMF_VIDEO_ENCODER_HEVC_FRAMERATE: &WideCStr = widecstr!("HevcFrameRate");

pub const AMF_VIDEO_ENCODER_HEVC_ENFORCE_HRD: &WideCStr = widecstr!("HevcEnforceHRD");
pub const AMF_VIDEO_ENCODER_HEVC_FILLER_DATA_ENABLE: &WideCStr = widecstr!("HevcFillerDataEnable");
pub const AMF_VIDEO_ENCODER_HEVC_TARGET_BITRATE: &WideCStr = widecstr!("HevcTargetBitrate");
pub const AMF_VIDEO_ENCODER_HEVC_PEAK_BITRATE: &WideCStr = widecstr!("HevcPeakBitrate");

pub const AMF_VIDEO_ENCODER_HEVC_MAX_AU_SIZE: &WideCStr = widecstr!("HevcMaxAUSize");

pub const AMF_VIDEO_ENCODER_HEVC_MIN_QP_I: &WideCStr = widecstr!("HevcMinQP_I");
pub const AMF_VIDEO_ENCODER_HEVC_MAX_QP_I: &WideCStr = widecstr!("HevcMaxQP_I");
pub const AMF_VIDEO_ENCODER_HEVC_MIN_QP_P: &WideCStr = widecstr!("HevcMinQP_P");
pub const AMF_VIDEO_ENCODER_HEVC_MAX_QP_P: &WideCStr = widecstr!("HevcMaxQP_P");

pub const AMF_VIDEO_ENCODER_HEVC_QP_I: &WideCStr = widecstr!("HevcQP_I");
pub const AMF_VIDEO_ENCODER_HEVC_QP_P: &WideCStr = widecstr!("HevcQP_P");

pub const AMF_VIDEO_ENCODER_HEVC_RATE_CONTROL_SKIP_FRAME_ENABLE: &WideCStr =
    widecstr!("HevcRateControlSkipFrameEnable");

pub const AMF_VIDEO_ENCODER_HEVC_INPUT_HDR_METADATA: &WideCStr = widecstr!("HevcInHDRMetadata");

pub const AMF_VIDEO_ENCODER_HEVC_NUM_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("HevcNumOfTemporalLayers");

pub const AMF_VIDEO_ENCODER_HEVC_PICTURE_TRANSFER_MODE: &WideCStr =
    widecstr!("HevcPicTransferMode");

pub const AMF_VIDEO_ENCODER_HEVC_QUERY_TIMEOUT: &WideCStr = widecstr!("HevcQueryTimeout");
pub const AMF_VIDEO_ENCODER_HEVC_MEMORY_TYPE: &WideCStr = widecstr!("HevcEncoderMemoryType");
pub const AMF_VIDEO_ENCODER_HEVC_ENABLE_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("HevcEnableEncoderSmartAccessVideo");
pub const AMF_VIDEO_ENCODER_HEVC_INPUT_QUEUE_SIZE: &WideCStr = widecstr!("HevcInputQueueSize");

pub const AMF_VIDEO_ENCODER_HEVC_END_OF_SEQUENCE: &WideCStr = widecstr!("HevcEndOfSequence");
pub const AMF_VIDEO_ENCODER_HEVC_FORCE_PICTURE_TYPE: &WideCStr = widecstr!("HevcForcePictureType");
pub const AMF_VIDEO_ENCODER_HEVC_INSERT_AUD: &WideCStr = widecstr!("HevcInsertAUD");
pub const AMF_VIDEO_ENCODER_HEVC_INSERT_HEADER: &WideCStr = widecstr!("HevcInsertHeader");

pub const AMF_VIDEO_ENCODER_HEVC_MARK_CURRENT_WITH_LTR_INDEX: &WideCStr =
    widecstr!("HevcMarkCurrentWithLTRIndex");
pub const AMF_VIDEO_ENCODER_HEVC_FORCE_LTR_REFERENCE_BITFIELD: &WideCStr =
    widecstr!("HevcForceLTRReferenceBitfield");
pub const AMF_VIDEO_ENCODER_HEVC_ROI_DATA: &WideCStr = widecstr!("HevcROIData");
pub const AMF_VIDEO_ENCODER_HEVC_REFERENCE_PICTURE: &WideCStr = widecstr!("HevcReferencePicture");
pub const AMF_VIDEO_ENCODER_HEVC_PSNR_FEEDBACK: &WideCStr = widecstr!("HevcPSNRFeedback");
pub const AMF_VIDEO_ENCODER_HEVC_SSIM_FEEDBACK: &WideCStr = widecstr!("HevcSSIMFeedback");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTICS_FEEDBACK: &WideCStr =
    widecstr!("HevcStatisticsFeedback");
pub const AMF_VIDEO_ENCODER_HEVC_BLOCK_QP_FEEDBACK: &WideCStr = widecstr!("HevcBlockQpFeedback");

pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_DATA_TYPE: &WideCStr = widecstr!("HevcOutputDataType");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_MARKED_LTR_INDEX: &WideCStr =
    widecstr!("HevcMarkedLTRIndex");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_REFERENCED_LTR_INDEX_BITFIELD: &WideCStr =
    widecstr!("HevcReferencedLTRIndexBitfield");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_TEMPORAL_LAYER: &WideCStr =
    widecstr!("HevcOutputTemporalLayer");
pub const AMF_VIDEO_ENCODER_HEVC_OUTPUT_BUFFER_TYPE: &WideCStr = widecstr!("HevcOutputBufferType");
pub const AMF_VIDEO_ENCODER_HEVC_RECONSTRUCTED_PICTURE: &WideCStr =
    widecstr!("HevcReconstructedPicture");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PSNR_Y: &WideCStr = widecstr!("PSNRY");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PSNR_U: &WideCStr = widecstr!("PSNRU");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PSNR_V: &WideCStr = widecstr!("PSNRV");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PSNR_ALL: &WideCStr = widecstr!("PSNRALL");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SSIM_Y: &WideCStr = widecstr!("SSIMY");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SSIM_U: &WideCStr = widecstr!("SSIMU");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SSIM_V: &WideCStr = widecstr!("SSIMV");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SSIM_ALL: &WideCStr = widecstr!("SSIMALL");

pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_FRAME_QP: &WideCStr =
    widecstr!("HevcStatisticsFeedbackFrameQP");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_AVERAGE_QP: &WideCStr =
    widecstr!("HevcStatisticsFeedbackAvgQP");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_MAX_QP: &WideCStr =
    widecstr!("HevcStatisticsFeedbackMaxQP");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_MIN_QP: &WideCStr =
    widecstr!("HevcStatisticsFeedbackMinQP");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PIX_NUM_INTRA: &WideCStr =
    widecstr!("HevcStatisticsFeedbackPixNumIntra");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PIX_NUM_INTER: &WideCStr =
    widecstr!("HevcStatisticsFeedbackPixNumInter");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_PIX_NUM_SKIP: &WideCStr =
    widecstr!("HevcStatisticsFeedbackPixNumSkip");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_BITCOUNT_RESIDUAL: &WideCStr =
    widecstr!("HevcStatisticsFeedbackBitcountResidual");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_BITCOUNT_MOTION: &WideCStr =
    widecstr!("HevcStatisticsFeedbackBitcountMotion");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_BITCOUNT_INTER: &WideCStr =
    widecstr!("HevcStatisticsFeedbackBitcountInter");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_BITCOUNT_INTRA: &WideCStr =
    widecstr!("HevcStatisticsFeedbackBitcountIntra");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_BITCOUNT_ALL_MINUS_HEADER: &WideCStr =
    widecstr!("HevcStatisticsFeedbackBitcountAllMinusHeader");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_MV_X: &WideCStr = widecstr!("HevcStatisticsFeedbackMvX");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_MV_Y: &WideCStr = widecstr!("HevcStatisticsFeedbackMvY");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_RD_COST_FINAL: &WideCStr =
    widecstr!("HevcStatisticsFeedbackRdCostFinal");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_RD_COST_INTRA: &WideCStr =
    widecstr!("HevcStatisticsFeedbackRdCostIntra");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_RD_COST_INTER: &WideCStr =
    widecstr!("HevcStatisticsFeedbackRdCostInter");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SAD_FINAL: &WideCStr =
    widecstr!("HevcStatisticsFeedbackSadFinal");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SAD_INTRA: &WideCStr =
    widecstr!("HevcStatisticsFeedbackSadIntra");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_SAD_INTER: &WideCStr =
    widecstr!("HevcStatisticsFeedbackSadInter");
pub const AMF_VIDEO_ENCODER_HEVC_STATISTIC_VARIANCE: &WideCStr =
    widecstr!("HevcStatisticsFeedbackVariance");

pub const AMF_VIDEO_ENCODER_HEVC_BLOCK_QP_MAP: &WideCStr = widecstr!("HevcBlockQpMap");

pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_BITRATE: &WideCStr = widecstr!("HevcMaxBitrate");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_NUM_OF_STREAMS: &WideCStr = widecstr!("HevcNumOfStreams");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_PROFILE: &WideCStr = widecstr!("HevcMaxProfile");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_TIER: &WideCStr = widecstr!("HevcMaxTier");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_LEVEL: &WideCStr = widecstr!("HevcMaxLevel");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MIN_REFERENCE_FRAMES: &WideCStr =
    widecstr!("HevcMinReferenceFrames");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_REFERENCE_FRAMES: &WideCStr =
    widecstr!("HevcMaxReferenceFrames");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("HevcMaxTemporalLayers");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_NUM_OF_HW_INSTANCES: &WideCStr =
    widecstr!("HevcNumOfHwInstances");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_COLOR_CONVERSION: &WideCStr = widecstr!("HevcColorConversion");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_PRE_ANALYSIS: &WideCStr = widecstr!("HevcPreAnalysis");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_ROI: &WideCStr = widecstr!("HevcROIMap");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_MAX_THROUGHPUT: &WideCStr = widecstr!("HevcMaxThroughput");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_REQUESTED_THROUGHPUT: &WideCStr =
    widecstr!("HevcRequestedThroughput");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_QUERY_TIMEOUT_SUPPORT: &WideCStr =
    widecstr!("HevcQueryTimeoutSupport");
pub const AMF_VIDEO_ENCODER_CAPS_HEVC_QUERY_TIMEOUT_SUPPORT: &WideCStr =
    widecstr!("HevcQueryTimeoutSupport");
pub const AMF_VIDEO_ENCODER_HEVC_CAP_SUPPORT_SLICE_OUTPUT: &WideCStr =
    widecstr!("HevcSupportSliceOutput");

pub const AMF_VIDEO_ENCODER_HEVC_CAP_SUPPORT_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("HevcEncoderSupportSmartAccessVideo");

pub const AMF_VIDEO_ENCODER_HEVC_MULTI_HW_INSTANCE_ENCODE: &WideCStr =
    widecstr!("HevcMultiHwInstanceEncode");

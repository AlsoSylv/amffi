use widestring::{WideCStr, widecstr};

pub const AMF_VIDEO_ENCODER_VCE_AVC: &WideCStr = widecstr!("AMFVideoEncoderVCE_AVC");
pub const AMF_VIDEO_ENCODER_VCE_SVC: &WideCStr = widecstr!("AMFVideoEncoderVCE_SVC");

#[repr(C)]
pub enum AMFVideoEncoderUsage {
    Transcoding = 0,
    UltraLowLatency,
    LowLatency,
    Webcam,
    HighQuality,
    LowLatencyHighQuality,
}

#[repr(C)]
pub enum AMFVideoEncoderProfile {
    Unknown = 0,
    Baseline = 66,
    Main = 77,
    High = 100,
    ConstrainedBaseline = 256,
    ConstrainedHigh = 257,
}

#[repr(C)]
pub enum AMFVideoEncoderH264Level {
    _1 = 10,
    _1_1 = 11,
    _1_2 = 12,
    _1_3 = 13,
    _2 = 20,
    _2_1 = 21,
    _2_2 = 22,
    _3 = 30,
    _3_1 = 31,
    _3_2 = 32,
    _4 = 40,
    _4_1 = 41,
    _4_2 = 42,
    _5 = 50,
    _5_1 = 51,
    _5_2 = 52,
    _6 = 60,
    _6_1 = 61,
    _6_2 = 62,
}

#[repr(C)]
pub enum AMFVideoEncoderScantype {
    Progressive = 0,
    Interlaced,
}

#[repr(C)]
pub enum AMFVideoEncoderRateControlMethod {
    Unknown = -1,
    ConstantQp = 0,
    Cbr,
    PeakConstrainedVbr,
    LatencyConstrainedVbr,
    QualityVbr,
    HighQualityVbr,
    HighQualityCbr,
}

#[repr(C)]
pub enum AMFVideoEncoderQualityPreset {
    Balanced = 0,
    Speed,
    Quality,
    HighQuality,
}

#[repr(C)]
pub enum AMFVideoEncoderPictureStructure {
    None = 0,
    Frame,
    TopField,
    BottomField,
}

#[repr(C)]
pub enum AMFVideoEncoderPictureType {
    None = 0,
    Skip,
    Idr,
    I,
    P,
    B,
}

#[repr(C)]
pub enum AMFVideoEncoderOutputDataType {
    Idr,
    I,
    P,
    B,
}

#[repr(C)]
pub enum AMFVideoEncoderPreencodeMode {
    Disable = 0,
    Enable = 1,
}

#[repr(C)]
pub enum AMFVideoEncoderCoding {
    Undefined = 0,
    Cabac,
    Calv,
}

#[repr(C)]
pub enum AMFVideoEncoderTransferMode {
    Off = 0,
    On,
}

#[repr(C)]
pub enum AMFVideoEncoderLtrMode {
    ResetUnused = 0,
    KeepUnused,
}

#[repr(C)]
pub enum AMFVideoEncoderOutputMode {
    Frame = 0,
    Slice = 1,
}

#[repr(C)]
pub enum AMFVideoEncoderOutputBufferType {
    Frame = 0,
    Slice = 1,
    SliceLast = 2,
}

pub const AMF_VIDEO_ENCODER_INSTANCE_INDEX: &WideCStr = widecstr!("EncoderInstance");
pub const AMF_VIDEO_ENCODER_FRAMESIZE: &WideCStr = widecstr!("FrameSize");
pub const AMF_VIDEO_ENCODER_EXTRADATA: &WideCStr = widecstr!("ExtraData");
pub const AMF_VIDEO_ENCODER_USAGE: &WideCStr = widecstr!("Usage");
pub const AMF_VIDEO_ENCODER_PROFILE: &WideCStr = widecstr!("Profile");
pub const AMF_VIDEO_ENCODER_PROFILE_LEVEL: &WideCStr = widecstr!("ProfileLevel");
pub const AMF_VIDEO_ENCODER_MAX_LTR_FRAMES: &WideCStr = widecstr!("MaxOfLTRFrames");
pub const AMF_VIDEO_ENCODER_LTR_MODE: &WideCStr = widecstr!("LTRMode");
pub const AMF_VIDEO_ENCODER_SCANTYPE: &WideCStr = widecstr!("ScanType");
pub const AMF_VIDEO_ENCODER_MAX_NUM_REFRAMES: &WideCStr = widecstr!("MaxNumRefFrames");
pub const AMF_VIDEO_ENCODER_MAX_CONSECUTIVE_BPICTURES: &WideCStr =
    widecstr!("MaxConsecutiveBPictures");
pub const AMF_VIDEO_ENCODER_ADAPTIVE_MINIGOP: &WideCStr = widecstr!("AdaptiveMiniGOP");
pub const AMF_VIDEO_ENCODER_ASPECT_RATIO: &WideCStr = widecstr!("AspectRatio");
pub const AMF_VIDEO_ENCODER_FULL_RANGE_COLOR: &WideCStr = widecstr!("FullRangeColor");
pub const AMF_VIDEO_ENCODER_LOWLATENCY_MODE: &WideCStr = widecstr!("LowLatencyInternal");
pub const AMF_VIDEO_ENCODER_PRE_ANALYSIS_ENABLE: &WideCStr = widecstr!("EnablePreAnalysis");
pub const AMF_VIDEO_ENCODER_PREENCODE_ENABLE: &WideCStr = widecstr!("RateControlPreanalysisEnable");
pub const AMF_VIDEO_ENCODER_RATE_CONTROL_PREANALYSIS_ENABLE: &WideCStr =
    widecstr!("RateControlPreanalysisEnable");
pub const AMF_VIDEO_ENCODER_RATE_CONTROL_METHOD: &WideCStr = widecstr!("RateControlMethod");
pub const AMF_VIDEO_ENCODER_QVBR_QUALITY_LEVEL: &WideCStr = widecstr!("QvbrQualityLevel");
pub const AMF_VIDEO_ENCODER_MAX_NUM_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("MaxNumOfTemporalLayers");
pub const AMF_VIDEO_ENCODER_QUALITY_PRESET: &WideCStr = widecstr!("QualityPreset");
pub const AMF_VIDEO_ENCODER_COLOR_BIT_DEPTH: &WideCStr = widecstr!("ColorBitDepth");
pub const AMF_VIDEO_ENCODER_INPUT_COLOR_PROFILE: &WideCStr = widecstr!("InColorProfile");
pub const AMF_VIDEO_ENCODER_INPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("InColorTransferChar");
pub const AMF_VIDEO_ENCODER_INPUT_COLOR_PRIMARIES: &WideCStr = widecstr!("InColorPrimaries");
pub const AMF_VIDEO_ENCODER_INPUT_HDR_METADATA: &WideCStr = widecstr!("InHDRMetadata");
pub const AMF_VIDEO_ENCODER_OUTPUT_COLOR_PROFILE: &WideCStr = widecstr!("OutColorProfile");
pub const AMF_VIDEO_ENCODER_OUTPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("OutColorTransferChar");
pub const AMF_VIDEO_ENCODER_OUTPUT_COLOR_PRIMARIES: &WideCStr = widecstr!("OutColorPrimaries");
pub const AMF_VIDEO_ENCODER_OUTPUT_HDR_METADATA: &WideCStr = widecstr!("OutHDRMetadata");
pub const AMF_VIDEO_ENCODER_OUTPUT_MODE: &WideCStr = widecstr!("OutputMode");
pub const AMF_VIDEO_ENCODER_FRAMERATE: &WideCStr = widecstr!("FrameRate");
pub const AMF_VIDEO_ENCODER_B_PIC_DELTA_QP: &WideCStr = widecstr!("BPicturesDeltaQP");
pub const AMF_VIDEO_ENCODER_REF_B_PIC_DELTA_QP: &WideCStr = widecstr!("ReferenceBPicturesDeltaQP");
pub const AMF_VIDEO_ENCODER_ENFORCE_HRD: &WideCStr = widecstr!("EnforceHRD");
pub const AMF_VIDEO_ENCODER_FILLER_DATA_ENABLE: &WideCStr = widecstr!("FillerDataEnable");
pub const AMF_VIDEO_ENCODER_ENABLE_VBAQ: &WideCStr = widecstr!("EnableVBAQ");
pub const AMF_VIDEO_ENCODER_HIGH_MOTION_QUALITY_BOOST_ENABLE: &WideCStr =
    widecstr!("HighMotionQualityBoostEnable");
pub const AMF_VIDEO_ENCODER_VBV_BUFFER_SIZE: &WideCStr = widecstr!("VBVBufferSize");
pub const AMF_VIDEO_ENCODER_INITIAL_VBV_BUFFER_FULLNESS: &WideCStr =
    widecstr!("InitialVBVBufferFullness");
pub const AMF_VIDEO_ENCODER_MAX_AU_SIZE: &WideCStr = widecstr!("MaxAUSize");
pub const AMF_VIDEO_ENCODER_MIN_QP: &WideCStr = widecstr!("MinQP");
pub const AMF_VIDEO_ENCODER_MAX_QP: &WideCStr = widecstr!("MaxQP");
pub const AMF_VIDEO_ENCODER_QP_I: &WideCStr = widecstr!("QPI");
pub const AMF_VIDEO_ENCODER_QP_P: &WideCStr = widecstr!("QPP");
pub const AMF_VIDEO_ENCODER_QP_B: &WideCStr = widecstr!("QPB");
pub const AMF_VIDEO_ENCODER_TARGET_BITRATE: &WideCStr = widecstr!("TargetBitrate");
pub const AMF_VIDEO_ENCODER_PEAK_BITRATE: &WideCStr = widecstr!("PeakBitrate");
pub const AMF_VIDEO_ENCODER_RATE_CONTROL_SKIP_FRAME_ENABLE: &WideCStr =
    widecstr!("RateControlSkipFrameEnable");
pub const AMF_VIDEO_ENCODER_HEADER_INSERTION_SPACING: &WideCStr =
    widecstr!("HeaderInsertionSpacing");
pub const AMF_VIDEO_ENCODER_B_PIC_PATTERN: &WideCStr = widecstr!("BPicturesPattern");
pub const AMF_VIDEO_ENCODER_DE_BLOCKING_FILTER: &WideCStr = widecstr!("DeBlockingFilter");
pub const AMF_VIDEO_ENCODER_B_REFERENCE_ENABLE: &WideCStr = widecstr!("BReferenceEnable");
pub const AMF_VIDEO_ENCODER_IDR_PERIOD: &WideCStr = widecstr!("IDRPeriod");
pub const AMF_VIDEO_ENCODER_INTRA_PERIOD: &WideCStr = widecstr!("IntraPeriod");
pub const AMF_VIDEO_ENCODER_INTRA_REFRESH_NUM_MBS_PER_SLOT: &WideCStr =
    widecstr!("IntraRefreshMBsNumberPerSlot");
pub const AMF_VIDEO_ENCODER_SLICES_PER_FRAME: &WideCStr = widecstr!("SlicesPerFrame");
pub const AMF_VIDEO_ENCODER_CABAC_ENABLE: &WideCStr = widecstr!("CABACEnable");
pub const AMF_VIDEO_ENCODER_MOTION_HALF_PIXEL: &WideCStr = widecstr!("HalfPixel");
pub const AMF_VIDEO_ENCODER_MOTION_QUARTERPIXEL: &WideCStr = widecstr!("QuarterPixel");
pub const AMF_VIDEO_ENCODER_NUM_TEMPORAL_ENHANCMENT_LAYERS: &WideCStr =
    widecstr!("NumOfTemporalEnhancmentLayers");
pub const AMF_VIDEO_ENCODER_PICTURE_TRANSFER_MODE: &WideCStr = widecstr!("PicTransferMode");
pub const AMF_VIDEO_ENCODER_QUERY_TIMEOUT: &WideCStr = widecstr!("QueryTimeout");
pub const AMF_VIDEO_ENCODER_MEMORY_TYPE: &WideCStr = widecstr!("EncoderMemoryType");
pub const AMF_VIDEO_ENCODER_ENABLE_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("EnableEncoderSmartAccessVideo");
pub const AMF_VIDEO_ENCODER_INPUT_QUEUE_SIZE: &WideCStr = widecstr!("InputQueueSize");
pub const AMF_VIDEO_ENCODER_END_OF_SEQUENCE: &WideCStr = widecstr!("EndOfSequence");
pub const AMF_VIDEO_ENCODER_END_OF_STREAM: &WideCStr = widecstr!("EndOfStream");
pub const AMF_VIDEO_ENCODER_FORCE_PICTURE_TYPE: &WideCStr = widecstr!("ForcePictureType");
pub const AMF_VIDEO_ENCODER_INSERT_AUD: &WideCStr = widecstr!("InsertAUD");
pub const AMF_VIDEO_ENCODER_INSERT_SPS: &WideCStr = widecstr!("InsertSPS");
pub const AMF_VIDEO_ENCODER_INSERT_PPS: &WideCStr = widecstr!("InsertPPS");
pub const AMF_VIDEO_ENCODER_PICTURE_STRUCTURE: &WideCStr = widecstr!("PictureStructure");
pub const AMF_VIDEO_ENCODER_MARK_CURRENT_WITH_LTR_INDEX: &WideCStr =
    widecstr!("MarkCurrentWithLTRIndex");
pub const AMF_VIDEO_ENCODER_FORCE_LTR_REFERENCE_BITFIELD: &WideCStr =
    widecstr!("ForceLTRReferenceBitfield");
pub const AMF_VIDEO_ENCODER_ROI_DATA: &WideCStr = widecstr!("ROIData");
pub const AMF_VIDEO_ENCODER_REFERENCE_PICTURE: &WideCStr = widecstr!("ReferencePicture");
pub const AMF_VIDEO_ENCODER_PSNR_FEEDBACK: &WideCStr = widecstr!("PSNRFeedback");
pub const AMF_VIDEO_ENCODER_SSIM_FEEDBACK: &WideCStr = widecstr!("SSIMFeedback");
pub const AMF_VIDEO_ENCODER_STATISTICS_FEEDBACK: &WideCStr = widecstr!("StatisticsFeedback");
pub const AMF_VIDEO_ENCODER_BLOCK_QP_FEEDBACK: &WideCStr = widecstr!("BlockQpFeedback");
pub const AMF_VIDEO_ENCODER_OUTPUT_DATA_TYPE: &WideCStr = widecstr!("OutputDataType");
pub const AMF_VIDEO_ENCODER_OUTPUT_MARKED_LTR_INDEX: &WideCStr = widecstr!("MarkedLTRIndex");
pub const AMF_VIDEO_ENCODER_OUTPUT_REFERENCED_LTR_INDEX_BITFIELD: &WideCStr =
    widecstr!("ReferencedLTRIndexBitfield");
pub const AMF_VIDEO_ENCODER_OUTPUT_TEMPORAL_LAYER: &WideCStr = widecstr!("OutputTemporalLayer");
pub const AMF_VIDEO_ENCODER_OUTPUT_BUFFER_TYPE: &WideCStr = widecstr!("OutputBufferType");
pub const AMF_VIDEO_ENCODER_PRESENTATION_TIME_STAMP: &WideCStr = widecstr!("PresentationTimeStamp");
pub const AMF_VIDEO_ENCODER_RECONSTRUCTED_PICTURE: &WideCStr = widecstr!("ReconstructedPicture");
pub const AMF_VIDEO_ENCODER_STATISTIC_PSNR_Y: &WideCStr = widecstr!("PSNRY");
pub const AMF_VIDEO_ENCODER_STATISTIC_PSNR_U: &WideCStr = widecstr!("PSNRU");
pub const AMF_VIDEO_ENCODER_STATISTIC_PSNR_V: &WideCStr = widecstr!("PSNRV");
pub const AMF_VIDEO_ENCODER_STATISTIC_PSNR_ALL: &WideCStr = widecstr!("PSNRALL");
pub const AMF_VIDEO_ENCODER_STATISTIC_SSIM_Y: &WideCStr = widecstr!("SSIMY");
pub const AMF_VIDEO_ENCODER_STATISTIC_SSIM_U: &WideCStr = widecstr!("SSIMU");
pub const AMF_VIDEO_ENCODER_STATISTIC_SSIM_V: &WideCStr = widecstr!("SSIMV");
pub const AMF_VIDEO_ENCODER_STATISTIC_SSIM_ALL: &WideCStr = widecstr!("SSIMALL");
pub const AMF_VIDEO_ENCODER_STATISTIC_FRAME_QP: &WideCStr = widecstr!("StatisticsFeedbackFrameQP");
pub const AMF_VIDEO_ENCODER_STATISTIC_AVERAGE_QP: &WideCStr = widecstr!("StatisticsFeedbackAvgQP");
pub const AMF_VIDEO_ENCODER_STATISTIC_MAX_QP: &WideCStr = widecstr!("StatisticsFeedbackMaxQP");
pub const AMF_VIDEO_ENCODER_STATISTIC_MIN_QP: &WideCStr = widecstr!("StatisticsFeedbackMinQP");
pub const AMF_VIDEO_ENCODER_STATISTIC_PIX_NUM_INTRA: &WideCStr =
    widecstr!("StatisticsFeedbackPixNumIntra");
pub const AMF_VIDEO_ENCODER_STATISTIC_PIX_NUM_INTER: &WideCStr =
    widecstr!("StatisticsFeedbackPixNumInter");
pub const AMF_VIDEO_ENCODER_STATISTIC_PIX_NUM_SKIP: &WideCStr =
    widecstr!("StatisticsFeedbackPixNumSkip");
pub const AMF_VIDEO_ENCODER_STATISTIC_BITCOUNT_RESIDUAL: &WideCStr =
    widecstr!("StatisticsFeedbackBitcountResidual");
pub const AMF_VIDEO_ENCODER_STATISTIC_BITCOUNT_MOTION: &WideCStr =
    widecstr!("StatisticsFeedbackBitcountMotion");
pub const AMF_VIDEO_ENCODER_STATISTIC_BITCOUNT_INTER: &WideCStr =
    widecstr!("StatisticsFeedbackBitcountInter");
pub const AMF_VIDEO_ENCODER_STATISTIC_BITCOUNT_INTRA: &WideCStr =
    widecstr!("StatisticsFeedbackBitcountIntra");
pub const AMF_VIDEO_ENCODER_STATISTIC_BITCOUNT_ALL_MINUS_HEADER: &WideCStr =
    widecstr!("StatisticsFeedbackBitcountAllMinusHeader");
pub const AMF_VIDEO_ENCODER_STATISTIC_MV_X: &WideCStr = widecstr!("StatisticsFeedbackMvX");
pub const AMF_VIDEO_ENCODER_STATISTIC_MV_Y: &WideCStr = widecstr!("StatisticsFeedbackMvY");
pub const AMF_VIDEO_ENCODER_STATISTIC_RD_COST_FINAL: &WideCStr =
    widecstr!("StatisticsFeedbackRdCostFinal");
pub const AMF_VIDEO_ENCODER_STATISTIC_RD_COST_INTRA: &WideCStr =
    widecstr!("StatisticsFeedbackRdCostIntra");
pub const AMF_VIDEO_ENCODER_STATISTIC_RD_COST_INTER: &WideCStr =
    widecstr!("StatisticsFeedbackRdCostInter");
pub const AMF_VIDEO_ENCODER_STATISTIC_SATD_FINAL: &WideCStr =
    widecstr!("StatisticsFeedbackSatdFinal");
pub const AMF_VIDEO_ENCODER_STATISTIC_SATD_INTRA: &WideCStr =
    widecstr!("StatisticsFeedbackSatdIntra");
pub const AMF_VIDEO_ENCODER_STATISTIC_SATD_INTER: &WideCStr =
    widecstr!("StatisticsFeedbackSatdInter");
pub const AMF_VIDEO_ENCODER_STATISTIC_VARIANCE: &WideCStr = widecstr!("StatisticsFeedbackVariance");
pub const AMF_VIDEO_ENCODER_BLOCK_QP_MAP: &WideCStr = widecstr!("BlockQpMap");
pub const AMF_VIDEO_ENCODER_HDCP_COUNTER: &WideCStr = widecstr!("HDCPCounter");
pub const AMF_VIDEO_ENCODER_MAX_INSTANCES: &WideCStr = widecstr!("EncoderMaxInstances");
pub const AMF_VIDEO_ENCODER_MULTI_INSTANCE_MODE: &WideCStr = widecstr!("MultiInstanceMode");
pub const AMF_VIDEO_ENCODER_CURRENT_QUEUE: &WideCStr = widecstr!("MultiInstanceCurrentQueue");
pub const AMF_VIDEO_ENCODER_CAP_MAX_BITRATE: &WideCStr = widecstr!("MaxBitrate");
pub const AMF_VIDEO_ENCODER_CAP_NUM_OF_STREAMS: &WideCStr = widecstr!("NumOfStreams");
pub const AMF_VIDEO_ENCODER_CAP_MAX_PROFILE: &WideCStr = widecstr!("MaxProfile");
pub const AMF_VIDEO_ENCODER_CAP_MAX_LEVEL: &WideCStr = widecstr!("MaxLevel");
pub const AMF_VIDEO_ENCODER_CAP_BFRAMES: &WideCStr = widecstr!("BFrames");
pub const AMF_VIDEO_ENCODER_CAP_MIN_REFERENCE_FRAMES: &WideCStr = widecstr!("MinReferenceFrames");
pub const AMF_VIDEO_ENCODER_CAP_MAX_REFERENCE_FRAMES: &WideCStr = widecstr!("MaxReferenceFrames");
pub const AMF_VIDEO_ENCODER_CAP_MAX_TEMPORAL_LAYERS: &WideCStr = widecstr!("MaxTemporalLayers");
pub const AMF_VIDEO_ENCODER_CAP_FIXED_SLICE_MODE: &WideCStr = widecstr!("FixedSliceMode");
pub const AMF_VIDEO_ENCODER_CAP_NUM_OF_HW_INSTANCES: &WideCStr = widecstr!("NumOfHwInstances");
pub const AMF_VIDEO_ENCODER_CAP_COLOR_CONVERSION: &WideCStr = widecstr!("ColorConversion");
pub const AMF_VIDEO_ENCODER_CAP_PRE_ANALYSIS: &WideCStr = widecstr!("PreAnalysis");
pub const AMF_VIDEO_ENCODER_CAP_ROI: &WideCStr = widecstr!("ROIMap");
pub const AMF_VIDEO_ENCODER_CAP_MAX_THROUGHPUT: &WideCStr = widecstr!("MaxThroughput");
pub const AMF_VIDEO_ENCODER_CAP_REQUESTED_THROUGHPUT: &WideCStr = widecstr!("RequestedThroughput");
pub const AMF_VIDEO_ENCODER_CAPS_QUERY_TIMEOUT_SUPPORT: &WideCStr =
    widecstr!("QueryTimeoutSupport");
pub const AMF_VIDEO_ENCODER_CAP_QUERY_TIMEOUT_SUPPORT: &WideCStr = widecstr!("QueryTimeoutSupport");
pub const AMF_VIDEO_ENCODER_CAP_SUPPORT_SLICE_OUTPUT: &WideCStr = widecstr!("SupportSliceOutput");
pub const AMF_VIDEO_ENCODER_CAP_SUPPORT_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("EncoderSupportSmartAccessVideo");

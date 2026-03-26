use widestring::{WideCStr, widecstr};

pub const AMF_VIDEO_ENCODER_AV1: &WideCStr = widecstr!("AMFVideoEncoderHW_AV1");

#[derive(Default)]
pub enum AMFVideoEncoderAv1EncodinglatencyMode {
    #[default]
    None = 0,
    PowerSavingRealTime = 1,
    RealTime = 2,
    LowestLatency = 3,
}

#[derive(Default)]
pub enum AMFVideoEncoderAv1Usage {
    #[default]
    Transcoding = 0,
    UltraLowLatency = 2,
    LowLatency = 1,
    Webcam = 3,
    HighQuality = 4,
    LowLatencyHighQuality = 5,
}

#[derive(Default)]
pub enum AMFVideoEncoderAv1Profile {
    #[default]
    Main = 1,
}

pub enum AMFVideoEncoderAv1LevelEnum {
    _2_0 = 0,
    _2_1 = 1,
    _2_2 = 2,
    _2_3 = 3,
    _3_0 = 4,
    _3_1 = 5,
    _3_2 = 6,
    _3_3 = 7,
    _4_0 = 8,
    _4_1 = 9,
    _4_2 = 10,
    _4_3 = 11,
    _5_0 = 12,
    _5_1 = 13,
    _5_2 = 14,
    _5_3 = 15,
    _6_0 = 16,
    _6_1 = 17,
    _6_2 = 18,
    _6_3 = 19,
    _7_0 = 20,
    _7_1 = 21,
    _7_2 = 22,
    _7_3 = 23,
}

pub enum AMFVideoEncoderAv1RateControlMethod {
    Unknown = -1,
    ConstantQp = 0,
    LowLatencyConstrainedVbr = 1,
    PeakConstrainedVbr = 2,
    Cbr = 3,
    QualityVbr = 4,
    HighQualityVbr = 5,
    HighQualityCbr = 6,
}

pub enum AMFVideoEncoderAv1AlignmentMode {
    _64X16Only = 1,
    _64x16_1080pCoded1082 = 2,
    NoRestrictions = 3,
    _8X2Only = 4,
}

pub enum AMFVideoEncoderAv1ForceFrameType {
    None = 0,
    Key = 1,
    IntraOnly = 2,
    Switch = 3,
    ShowExisting = 4,
}

pub enum AMFVideoEncoderAv1OutputFrameType {
    Key = 0,
    IntraOnly = 1,
    Inter = 2,
    Switch = 3,
    ShowExisting = 4,
}

pub enum AMFVideoEncoderAv1QualityPreset {
    HighQuality = 0,
    Quality = 30,
    Balanced = 70,
    Speed = 100,
}

pub enum AMFVideoEncoderAv1HeaderInsertionMode {
    None = 0,
    GopAligned = 1,
    KeyFrameAligned = 2,
    Surpressed = 3,
}

pub enum AMFVideoEncoderAv1SwitchFrameInsertionMode {
    None = 0,
    FixedInterval = 1,
}

pub enum AMFVideoEncoderAv1CdefMode {
    Disable = 0,
    EnableDefault = 1,
}

pub enum AMFVideoEncoderAv1CdfFrameEndUpdateMode {
    Disable = 0,
    EnableDefault = 1,
}

pub enum AMFVideoEncoderAv1AqMode {
    None = 0,
    Caq = 1,
}

pub enum AMFVideoEncoderAv1IntraRefreshMode {
    Disable = 0,
    GopAligned = 1,
    Conttinous = 2,
}
pub enum AMFVideoEncoderAv1LtrMode {
    ResetUnused = 0,
    KeepUnused = 1,
}

pub enum AMFVideoEncoderAv1OutputMode {
    Frame = 0,
    Tile = 1,
}

pub enum AMFVideoEncoderAv1OutputBufferType {
    Frame = 0,
    Tile = 1,
    TileLast = 2,
}

pub const AMF_VIDEO_ENCODER_AV1_ENCODER_INSTANCE_INDEX: &WideCStr =
    widecstr!("Av1EncoderInstanceIndex");
pub const AMF_VIDEO_ENCODER_AV1_ENCODING_LATENCY_MODE: &WideCStr =
    widecstr!("Av1EncodingLatencyMode");
pub const AMF_VIDEO_ENCODER_AV1_QUERY_TIMEOUT: &WideCStr = widecstr!("Av1QueryTimeout");
pub const AMF_VIDEO_ENCODER_AV1_USAGE: &WideCStr = widecstr!("Av1Usage");
pub const AMF_VIDEO_ENCODER_AV1_FRAMESIZE: &WideCStr = widecstr!("Av1FrameSize");
pub const AMF_VIDEO_ENCODER_AV1_COLOR_BIT_DEPTH: &WideCStr = widecstr!("Av1ColorBitDepth");
pub const AMF_VIDEO_ENCODER_AV1_PROFILE: &WideCStr = widecstr!("Av1Profile");
pub const AMF_VIDEO_ENCODER_AV1_LEVEL: &WideCStr = widecstr!("Av1Level");
pub const AMF_VIDEO_ENCODER_AV1_TILES_PER_FRAME: &WideCStr = widecstr!("Av1NumTilesPerFrame");
pub const AMF_VIDEO_ENCODER_AV1_QUALITY_PRESET: &WideCStr = widecstr!("Av1QualityPreset");
pub const AMF_VIDEO_ENCODER_AV1_SCREEN_CONTENT_TOOLS: &WideCStr =
    widecstr!("Av1ScreenContentTools");
pub const AMF_VIDEO_ENCODER_AV1_ORDER_HINT: &WideCStr = widecstr!("Av1OrderHint");
pub const AMF_VIDEO_ENCODER_AV1_FRAME_ID: &WideCStr = widecstr!("Av1FrameId");
pub const AMF_VIDEO_ENCODER_AV1_TILE_GROUP_OBU: &WideCStr = widecstr!("Av1TileGroupObu");
pub const AMF_VIDEO_ENCODER_AV1_CDEF_MODE: &WideCStr = widecstr!("Av1CdefMode");
pub const AMF_VIDEO_ENCODER_AV1_ERROR_RESILIENT_MODE: &WideCStr =
    widecstr!("Av1ErrorResilientMode");
pub const AMF_VIDEO_ENCODER_AV1_RATE_CONTROL_METHOD: &WideCStr = widecstr!("Av1RateControlMethod");
pub const AMF_VIDEO_ENCODER_AV1_QVBR_QUALITY_LEVEL: &WideCStr = widecstr!("Av1QvbrQualityLevel");
pub const AMF_VIDEO_ENCODER_AV1_INITIAL_VBV_BUFFER_FULLNESS: &WideCStr =
    widecstr!("Av1InitialVBVBufferFullness");
pub const AMF_VIDEO_ENCODER_AV1_ALIGNMENT_MODE: &WideCStr = widecstr!("Av1AlignmentMode");
pub const AMF_VIDEO_ENCODER_AV1_PRE_ANALYSIS_ENABLE: &WideCStr = widecstr!("Av1EnablePreAnalysis");
pub const AMF_VIDEO_ENCODER_AV1_RATE_CONTROL_PREENCODE: &WideCStr =
    widecstr!("Av1RateControlPreEncode");
pub const AMF_VIDEO_ENCODER_AV1_HIGH_MOTION_QUALITY_BOOST: &WideCStr =
    widecstr!("Av1HighMotionQualityBoost");
pub const AMF_VIDEO_ENCODER_AV1_AQ_MODE: &WideCStr = widecstr!("Av1AQMode");
pub const AMF_VIDEO_ENCODER_AV1_MAX_NUM_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("Av1MaxNumOfTemporalLayers");
pub const AMF_VIDEO_ENCODER_AV1_MAX_LTR_FRAMES: &WideCStr = widecstr!("Av1MaxNumLTRFrames");
pub const AMF_VIDEO_ENCODER_AV1_LTR_MODE: &WideCStr = widecstr!("Av1LTRMode");
pub const AMF_VIDEO_ENCODER_AV1_MAX_NUM_REFRAMES: &WideCStr = widecstr!("Av1MaxNumRefFrames");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_HDR_METADATA: &WideCStr = widecstr!("Av1InHDRMetadata");
pub const AMF_VIDEO_ENCODER_AV1_EXTRA_DATA: &WideCStr = widecstr!("Av1ExtraData");
pub const AMF_VIDEO_ENCODER_AV1_ENABLE_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("Av1EnableEncoderSmartAccessVideo");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_QUEUE_SIZE: &WideCStr = widecstr!("Av1InputQueueSize");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_MODE: &WideCStr = widecstr!("AV1OutputMode");
pub const AMF_VIDEO_ENCODER_AV1_PALETTE_MODE: &WideCStr = widecstr!("Av1PaletteMode");
pub const AMF_VIDEO_ENCODER_AV1_FORCE_INTEGER_MV: &WideCStr = widecstr!("Av1ForceIntegerMv");
pub const AMF_VIDEO_ENCODER_AV1_CDF_UPDATE: &WideCStr = widecstr!("Av1CdfUpdate");
pub const AMF_VIDEO_ENCODER_AV1_CDF_FRAME_END_UPDATE_MODE: &WideCStr =
    widecstr!("Av1CdfFrameEndUpdateMode");
pub const AMF_VIDEO_ENCODER_AV1_VBV_BUFFER_SIZE: &WideCStr = widecstr!("Av1VBVBufferSize");
pub const AMF_VIDEO_ENCODER_AV1_FRAMERATE: &WideCStr = widecstr!("Av1FrameRate");
pub const AMF_VIDEO_ENCODER_AV1_ENFORCE_HRD: &WideCStr = widecstr!("Av1EnforceHRD");
pub const AMF_VIDEO_ENCODER_AV1_FILLER_DATA: &WideCStr = widecstr!("Av1FillerData");
pub const AMF_VIDEO_ENCODER_AV1_TARGET_BITRATE: &WideCStr = widecstr!("Av1TargetBitrate");
pub const AMF_VIDEO_ENCODER_AV1_PEAK_BITRATE: &WideCStr = widecstr!("Av1PeakBitrate");
pub const AMF_VIDEO_ENCODER_AV1_MAX_COMPRESSED_FRAME_SIZE: &WideCStr =
    widecstr!("Av1MaxCompressedFrameSize");
pub const AMF_VIDEO_ENCODER_AV1_MIN_Q_INDEX_INTRA: &WideCStr = widecstr!("Av1MinQIndex_Intra");
pub const AMF_VIDEO_ENCODER_AV1_MAX_Q_INDEX_INTRA: &WideCStr = widecstr!("Av1MaxQIndex_Intra");
pub const AMF_VIDEO_ENCODER_AV1_MIN_Q_INDEX_INTER: &WideCStr = widecstr!("Av1MinQIndex_Inter");
pub const AMF_VIDEO_ENCODER_AV1_MAX_Q_INDEX_INTER: &WideCStr = widecstr!("Av1MaxQIndex_Inter");
pub const AMF_VIDEO_ENCODER_AV1_MIN_Q_INDEX_INTER_B: &WideCStr = widecstr!("Av1MinQIndex_Inter_B");
pub const AMF_VIDEO_ENCODER_AV1_MAX_Q_INDEX_INTER_B: &WideCStr = widecstr!("Av1MaxQIndex_Inter_B");
pub const AMF_VIDEO_ENCODER_AV1_Q_INDEX_INTRA: &WideCStr = widecstr!("Av1QIndex_Intra");
pub const AMF_VIDEO_ENCODER_AV1_Q_INDEX_INTER: &WideCStr = widecstr!("Av1QIndex_Inter");
pub const AMF_VIDEO_ENCODER_AV1_Q_INDEX_INTER_B: &WideCStr = widecstr!("Av1QIndex_Inter_B");
pub const AMF_VIDEO_ENCODER_AV1_RATE_CONTROL_SKIP_FRAME: &WideCStr =
    widecstr!("Av1RateControlSkipFrameEnable");
pub const AMF_VIDEO_ENCODER_AV1_GOP_SIZE: &WideCStr = widecstr!("Av1GOPSize");
pub const AMF_VIDEO_ENCODER_AV1_INTRA_PERIOD: &WideCStr = widecstr!("Av1IntraPeriod");
pub const AMF_VIDEO_ENCODER_AV1_HEADER_INSERTION_MODE: &WideCStr =
    widecstr!("Av1HeaderInsertionMode");
pub const AMF_VIDEO_ENCODER_AV1_SWITCH_FRAME_INSERTION_MODE: &WideCStr =
    widecstr!("Av1SwitchFrameInsertionMode");
pub const AMF_VIDEO_ENCODER_AV1_SWITCH_FRAME_INTERVAL: &WideCStr =
    widecstr!("Av1SwitchFrameInterval");
pub const AMF_VIDEO_ENCODER_AV1_NUM_TEMPORAL_LAYERS: &WideCStr = widecstr!("Av1NumTemporalLayers");
pub const AMF_VIDEO_ENCODER_AV1_INTRA_REFRESH_MODE: &WideCStr = widecstr!("Av1IntraRefreshMode");
pub const AMF_VIDEO_ENCODER_AV1_INTRAREFRESH_STRIPES: &WideCStr =
    widecstr!("Av1IntraRefreshNumOfStripes");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_COLOR_PROFILE: &WideCStr = widecstr!("Av1InputColorProfile");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("Av1InputColorTransferChar");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_COLOR_PRIMARIES: &WideCStr =
    widecstr!("Av1InputColorPrimaries");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_MATRIX_COEFF: &WideCStr = widecstr!("Av1InMatrixCoeff");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_COLOR_PROFILE: &WideCStr =
    widecstr!("Av1OutputColorProfile");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_TRANSFER_CHARACTERISTIC: &WideCStr =
    widecstr!("Av1OutputColorTransferChar");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_COLOR_PRIMARIES: &WideCStr =
    widecstr!("Av1OutputColorPrimaries");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_MATRIX_COEFF: &WideCStr = widecstr!("Av1OutMatrixCoeff");
pub const AMF_VIDEO_ENCODER_AV1_FORCE_FRAME_TYPE: &WideCStr = widecstr!("Av1ForceFrameType");
pub const AMF_VIDEO_ENCODER_AV1_FORCE_INSERT_SEQUENCE_HEADER: &WideCStr =
    widecstr!("Av1ForceInsertSequenceHeader");
pub const AMF_VIDEO_ENCODER_AV1_MARK_CURRENT_WITH_LTR_INDEX: &WideCStr =
    widecstr!("Av1MarkCurrentWithLTRIndex");
pub const AMF_VIDEO_ENCODER_AV1_FORCE_LTR_REFERENCE_BITFIELD: &WideCStr =
    widecstr!("Av1ForceLTRReferenceBitfield");
pub const AMF_VIDEO_ENCODER_AV1_ROI_DATA: &WideCStr = widecstr!("Av1ROIData");
pub const AMF_VIDEO_ENCODER_AV1_PSNR_FEEDBACK: &WideCStr = widecstr!("Av1PSNRFeedback");
pub const AMF_VIDEO_ENCODER_AV1_SSIM_FEEDBACK: &WideCStr = widecstr!("Av1SSIMFeedback");
pub const AMF_VIDEO_ENCODER_AV1_STATISTICS_FEEDBACK: &WideCStr = widecstr!("Av1StatisticsFeedback");
pub const AMF_VIDEO_ENCODER_AV1_BLOCK_Q_INDEX_FEEDBACK: &WideCStr =
    widecstr!("Av1BlockQIndexFeedback");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_FRAME_TYPE: &WideCStr = widecstr!("Av1OutputFrameType");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_MARKED_LTR_INDEX: &WideCStr = widecstr!("Av1MarkedLTRIndex");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_REFERENCED_LTR_INDEX_BITFIELD: &WideCStr =
    widecstr!("Av1ReferencedLTRIndexBitfield");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_BUFFER_TYPE: &WideCStr = widecstr!("AV1OutputBufferType");
pub const AMF_VIDEO_ENCODER_AV1_RECONSTRUCTED_PICTURE: &WideCStr =
    widecstr!("Av1ReconstructedPicture");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PSNR_Y: &WideCStr = widecstr!("Av1PSNRY");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PSNR_U: &WideCStr = widecstr!("Av1PSNRU");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PSNR_V: &WideCStr = widecstr!("Av1PSNRV");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PSNR_ALL: &WideCStr = widecstr!("Av1PSNRALL");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SSIM_Y: &WideCStr = widecstr!("Av1SSIMY");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SSIM_U: &WideCStr = widecstr!("Av1SSIMU");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SSIM_V: &WideCStr = widecstr!("Av1SSIMV");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SSIM_ALL: &WideCStr = widecstr!("Av1SSIMALL");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_FRAME_Q_INDEX: &WideCStr =
    widecstr!("Av1StatisticsFeedbackFrameQIndex");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_AVERAGE_Q_INDEX: &WideCStr =
    widecstr!("Av1StatisticsFeedbackAvgQIndex");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_MAX_Q_INDEX: &WideCStr =
    widecstr!("Av1StatisticsFeedbackMaxQIndex");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_MIN_Q_INDEX: &WideCStr =
    widecstr!("Av1StatisticsFeedbackMinQIndex");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PIX_NUM_INTRA: &WideCStr =
    widecstr!("Av1StatisticsFeedbackPixNumIntra");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PIX_NUM_INTER: &WideCStr =
    widecstr!("Av1StatisticsFeedbackPixNumInter");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_PIX_NUM_SKIP: &WideCStr =
    widecstr!("Av1StatisticsFeedbackPixNumSkip");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_BITCOUNT_RESIDUAL: &WideCStr =
    widecstr!("Av1StatisticsFeedbackBitcountResidual");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_BITCOUNT_MOTION: &WideCStr =
    widecstr!("Av1StatisticsFeedbackBitcountMotion");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_BITCOUNT_INTER: &WideCStr =
    widecstr!("Av1StatisticsFeedbackBitcountInter");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_BITCOUNT_INTRA: &WideCStr =
    widecstr!("Av1StatisticsFeedbackBitcountIntra");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_BITCOUNT_ALL_MINUS_HEADER: &WideCStr =
    widecstr!("Av1StatisticsFeedbackBitcountAllMinusHeader");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_MV_X: &WideCStr = widecstr!("Av1StatisticsFeedbackMvX");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_MV_Y: &WideCStr = widecstr!("Av1StatisticsFeedbackMvY");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_RD_COST_FINAL: &WideCStr =
    widecstr!("Av1StatisticsFeedbackRdCostFinal");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_RD_COST_INTRA: &WideCStr =
    widecstr!("Av1StatisticsFeedbackRdCostIntra");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_RD_COST_INTER: &WideCStr =
    widecstr!("Av1StatisticsFeedbackRdCostInter");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SAD_FINAL: &WideCStr =
    widecstr!("Av1StatisticsFeedbackSadFinal");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SAD_INTRA: &WideCStr =
    widecstr!("Av1StatisticsFeedbackSadIntra");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SAD_INTER: &WideCStr =
    widecstr!("Av1StatisticsFeedbackSadInter");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_SSE: &WideCStr = widecstr!("Av1StatisticsFeedbackSSE");
pub const AMF_VIDEO_ENCODER_AV1_STATISTIC_VARIANCE: &WideCStr =
    widecstr!("Av1StatisticsFeedbackVariance");
pub const AMF_VIDEO_ENCODER_AV1_BLOCK_Q_INDEX_MAP: &WideCStr = widecstr!("Av1BlockQIndexMap");
pub const AMF_VIDEO_ENCODER_AV1_CAP_NUM_OF_HW_INSTANCES: &WideCStr =
    widecstr!("Av1CapNumOfHwInstances");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_THROUGHPUT: &WideCStr = widecstr!("Av1CapMaxThroughput");
pub const AMF_VIDEO_ENCODER_AV1_CAP_REQUESTED_THROUGHPUT: &WideCStr =
    widecstr!("Av1CapRequestedThroughput");
pub const AMF_VIDEO_ENCODER_AV1_CAP_COLOR_CONVERSION: &WideCStr =
    widecstr!("Av1CapColorConversion");
pub const AMF_VIDEO_ENCODER_AV1_CAP_PRE_ANALYSIS: &WideCStr = widecstr!("Av1PreAnalysis");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_BITRATE: &WideCStr = widecstr!("Av1MaxBitrate");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_PROFILE: &WideCStr = widecstr!("Av1MaxProfile");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_LEVEL: &WideCStr = widecstr!("Av1MaxLevel");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_NUM_TEMPORAL_LAYERS: &WideCStr =
    widecstr!("Av1CapMaxNumTemporalLayers");
pub const AMF_VIDEO_ENCODER_AV1_CAP_MAX_NUM_LTR_FRAMES: &WideCStr =
    widecstr!("Av1CapMaxNumLTRFrames");
pub const AMF_VIDEO_ENCODER_AV1_CAP_SUPPORT_TILE_OUTPUT: &WideCStr =
    widecstr!("AV1SupportTileOutput");
pub const AMF_VIDEO_ENCODER_AV1_CAP_BFRAMES: &WideCStr = widecstr!("AV1BFrames");
pub const AMF_VIDEO_ENCODER_AV1_CAP_SUPPORT_SMART_ACCESS_VIDEO: &WideCStr =
    widecstr!("Av1EncoderSupportSmartAccessVideo");
pub const AMF_VIDEO_ENCODER_AV1_CAP_WIDTH_ALIGNMENT_FACTOR: &WideCStr =
    widecstr!("Av1WidthAlignmentFactor");
pub const AMF_VIDEO_ENCODER_AV1_CAP_HEIGHT_ALIGNMENT_FACTOR: &WideCStr =
    widecstr!("Av1HeightAlignmentFactor");
pub const AMF_VIDEO_ENCODER_AV1_MULTI_HW_INSTANCE_ENCODE: &WideCStr =
    widecstr!("Av1MultiHwInstanceEncode");
pub const AMF_VIDEO_ENCODER_AV1_INPUT_FULL_RANGE_COLOR: &WideCStr =
    widecstr!("Av1InputFullRangeColor");
pub const AMF_VIDEO_ENCODER_AV1_OUTPUT_FULL_RANGE_COLOR: &WideCStr = widecstr!("Av1NominalRange");
pub const AMF_VIDEO_ENCODER_AV1_NOMINAL_RANGE: &WideCStr = widecstr!("Av1NominalRange");
pub const AMF_VIDEO_ENCODER_AV1_MAX_CONSECUTIVE_BPICTURES: &WideCStr =
    widecstr!("Av1MaxConsecutiveBPictures");
pub const AMF_VIDEO_ENCODER_AV1_B_PIC_PATTERN: &WideCStr = widecstr!("Av1BPicturesPattern");
pub const AMF_VIDEO_ENCODER_AV1_ADAPTIVE_MINIGOP: &WideCStr = widecstr!("Av1AdaptiveMiniGop");

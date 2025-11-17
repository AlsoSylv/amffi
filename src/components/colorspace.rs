use widestring::{WideCStr, widecstr};

#[repr(C)]
pub enum AMFVideoConvertColorProfileEnum {
    Unknown = -1,
    _601 = 0,
    _709 = 1,
    _2020 = 2,
    /// Also `Full_601`
    JPEG = 3,
    Full709,
    Full2020,
    Count,
}

#[repr(C)]
pub enum AMFColorPrimaries {
    Undefined = 0,
    BT709 = 1,
    Unspecified = 2,
    Reserved = 3,
    BT470M = 4,
    BT470BG = 5,
    SMPTE170M = 6,
    SMPTE240M = 7,
    Film = 8,
    BT2020 = 9,
    SMPTE428 = 10,
    SMPTE431 = 11,
    SMPTE432 = 12,
    JedecP22 = 22,
    CCCS = 1000,
}

#[repr(C)]
pub enum AMFColorTransferCharacteristics {
    Undefined = 0,
    BT709 = 1,
    Unspecified = 2,
    Reserved = 3,
    Gamma22 = 4,
    Gamma28 = 5,
    SMPTE170M = 6,
    SMPTE240M = 7,
    Linear = 8,
    Log = 9,
    LogSqrt = 10,
    IEC61966_2_4 = 11,
    BT1361ECG = 12,
    IEC61966_2_1 = 13,
    BT2020_10 = 14,
    BT2020_12 = 15,
    SMPTE2084 = 16,
    SMPTE428 = 17,
    #[allow(non_camel_case_types)]
    ARIB_STD_B67 = 18,
}

#[repr(C)]
pub enum AMFColorBitDepth {
    Undefined = 0,
    Eight = 8,
    Ten = 10,
}

#[repr(C)]
pub struct AMFHDRMetadata {
    pub red_primary: [u16; 2],
    pub green_primary: [u16; 2],
    pub blue_primary: [u16; 2],
    pub white_point: [u16; 2],
    pub max_mastering_luminance: u16,
    pub min_mastering_luminance: u16,
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}

#[repr(C)]
pub enum AMFColorRange {
    Undefined = 0,
    Studio = 1,
    Full = 2,
}

pub const AMF_VIDEO_COLOR_TRANSFER_CHARACTERISTIC: &WideCStr = widecstr!("ColorTransferChar");
pub const AMF_VIDEO_COLOR_PRIMARIES: &WideCStr = widecstr!("ColorPrimaries");
pub const AMF_VIDEO_COLOR_RANGE: &WideCStr = widecstr!("ColorRange");
pub const AMF_VIDEO_COLOR_HDR_METADATA: &WideCStr = widecstr!("HdrMetadata");

use widestring::{WideCStr, widecstr};

pub const AMF_PRE_ANALYSIS: &WideCStr = widecstr!("AMFPreAnalysis");

#[repr(C)]
pub enum AMFPaSceneChangeDetectionSensitivity {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[repr(C)]
pub enum AMFPaActivityType {
    Y = 0,
    YUV = 1,
}

#[repr(C)]
pub enum AMFPaCaqStrength {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[repr(C)]
pub enum AMFPaPaqMode {
    None = 0,
    Caq = 1,
}

#[repr(C)]
pub enum AMFPaTaqMode {
    None = 0,
    Mode1 = 1,
    Mode2 = 2,
}

#[repr(C)]
pub enum AMFPaHighMotionQualityBoostMode {
    None = 0,
    Auto = 1,
}

pub const AMF_PA_ENGINE_TYPE: &WideCStr = widecstr!("PAEngineType");
pub const AMF_PA_QUERY_TIMEOUT: &WideCStr = widecstr!("PAQueryTimeout");
pub const AMF_PA_SCENE_CHANGE_DETECTION_ENABLE: &WideCStr =
    widecstr!("PASceneChangeDetectionEnable");
pub const AMF_PA_SCENE_CHANGE_DETECTION_SENSITIVITY: &WideCStr =
    widecstr!("PASceneChangeDetectionSensitivity");
pub const AMF_PA_STATIC_SCENE_DETECTION_ENABLE: &WideCStr =
    widecstr!("PAStaticSceneDetectionEnable");
pub const AMF_PA_STATIC_SCENE_DETECTION_SENSITIVITY: &WideCStr =
    widecstr!("PAStaticSceneDetectionSensitivity");
pub const AMF_PA_FRAME_SAD_ENABLE: &WideCStr = widecstr!("PAFrameSadEnable");
pub const AMF_PA_ACTIVITY_TYPE: &WideCStr = widecstr!("PAActivityType");
pub const AMF_PA_LTR_ENABLE: &WideCStr = widecstr!("PALongTermReferenceEnable");
pub const AMF_PA_LOOKAHEAD_BUFFER_DEPTH: &WideCStr = widecstr!("PALookAheadBufferDepth");
pub const AMF_PA_PAQ_MODE: &WideCStr = widecstr!("PAPerceptualAQMode");
pub const AMF_PA_TAQ_MODE: &WideCStr = widecstr!("PATemporalAQMode");
pub const AMF_PA_HIGH_MOTION_QUALITY_BOOST_MODE: &WideCStr =
    widecstr!("PAHighMotionQualityBoostMode");
pub const AMF_PA_INITIAL_QP_AFTER_SCENE_CHANGE: &WideCStr =
    widecstr!("PAInitialQPAfterSceneChange");
pub const AMF_PA_MAX_QP_BEFORE_FORCE_SKIP: &WideCStr = widecstr!("PAMaxQPBeforeForceSkip");
pub const AMF_PA_CAQ_STRENGTH: &WideCStr = widecstr!("PACAQStrength");
pub const AMF_PA_ACTIVITY_MAP: &WideCStr = widecstr!("PAActivityMap");
pub const AMF_PA_SCENE_CHANGE_DETECT: &WideCStr = widecstr!("PASceneChangeDetect");
pub const AMF_PA_STATIC_SCENE_DETECT: &WideCStr = widecstr!("PAStaticSceneDetect");

pub const fn get_major_version() -> u16 {
    get_version_numbers()[0]
}

pub const fn get_minor_version() -> u16 {
    get_version_numbers()[1]
}

pub const fn get_release_version() -> u16 {
    get_version_numbers()[2]
}

pub const fn get_build_version() -> u16 {
    get_version_numbers()[3]
}

/// Returns in order [MAJOR, MINOR, RELEASE, BUILD]
pub const fn get_version_numbers() -> [u16; 4] {
    [
        AMF_VERSION_MAJOR,
        AMF_VERSION_MINOR,
        AMF_VERSION_RELEASE,
        AMF_VERSION_BUILD_NUM,
    ]
}

/// 1.4.36.0
pub const AMF_VERSION: u64 = (AMF_VERSION_MAJOR as u64) << 48
    | (AMF_VERSION_MINOR as u64) << 32
    | (AMF_VERSION_RELEASE as u64) << 16
    | (AMF_VERSION_BUILD_NUM as u64);
pub const AMF_VERSION_MAJOR: u16 = 1;
pub const AMF_VERSION_MINOR: u16 = 4;
pub const AMF_VERSION_RELEASE: u16 = 36;
pub const AMF_VERSION_BUILD_NUM: u16 = 0;

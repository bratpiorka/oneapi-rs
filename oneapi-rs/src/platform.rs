use oneapi_rs_sys::platform::ffi;

pub struct Platform(cxx::SharedPtr<ffi::Platform>);

impl Platform {
    pub fn get_platforms() -> Vec<Platform> {
        ffi::Platform::get_platforms()
            .into_iter()
            .map(|platform| Platform(platform.ptr.clone()))
            .collect()
    }
}

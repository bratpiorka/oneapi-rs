#[cxx::bridge(namespace = "sycl_shims")]
pub mod ffi {
    unsafe extern "C++" {
        include!("oneapi-rs-sys/include/platform.hpp");
 
        type Platform;

        #[Self = "Platform"]
        fn get_platforms() -> UniquePtr<CxxVector<Platform>>;

        fn get_version(&self) -> String;
        fn get_name(&self) -> String;
        fn get_vendor(&self) -> String;
    }
}

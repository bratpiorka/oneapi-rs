#include "oneapi-rs-sys/include/shim.hpp"

std::unique_ptr<std::vector<sycl::platform>> get_platforms() {
  return std::make_unique<std::vector<sycl::platform>>(
      sycl::platform::get_platforms());
}

std::unique_ptr<std::string> get_name(sycl::platform const &p) {
  return std::make_unique<std::string>(
      p.get_info<sycl::info::platform::name>());
}
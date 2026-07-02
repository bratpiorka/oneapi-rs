#pragma once
#include "oneapi-rs-sys/include/device.hpp"
#include "oneapi-rs-sys/src/opaque-sys.rs.h"
#include <memory>

std::unique_ptr<Device> get_device(std::unique_ptr<Device> ptr) {
    return std::move(ptr);
}
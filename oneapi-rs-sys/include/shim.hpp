#pragma once
#include <memory>
#include <string>
#include <sycl/sycl.hpp>
#include <vector>

std::unique_ptr<std::vector<sycl::platform>> get_platforms();
std::unique_ptr<std::string> get_name(sycl::platform const &p);
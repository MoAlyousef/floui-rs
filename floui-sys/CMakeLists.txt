cmake_minimum_required(VERSION 3.14)
project(floui_lib)

add_library(floui STATIC src/floui.cpp)
target_compile_features(floui PRIVATE cxx_std_17)

install(TARGETS floui
    DESTINATION ${CMAKE_INSTALL_PREFIX}
    )
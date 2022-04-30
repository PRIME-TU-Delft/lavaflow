cmake_minimum_required (VERSION 3.0)
project(3D_model_construction VERSION 0.1.0)

include(CTest)
enable_testing()

set(CMAKE_VERBOSE_MAKEFILE on)

file(GLOB_RECURSE CORE_HDR src/*.h)
file(GLOB_RECURSE CORE_SRC src/*.cpp)

add_definitions("-std=c++17")
add_executable(core ${CORE_SRC} ${CORE_HDR})

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

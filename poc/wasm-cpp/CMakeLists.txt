cmake_minimum_required (VERSION 3.0)
project (wasm-cpp)

set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/build.emscripten)

file(GLOB_RECURSE CORE_HDR src/*.h)
file(GLOB_RECURSE CORE_SRC src/*.cpp)

add_definitions("-std=c++17")
#add_executable(core ${CORE_SRC} ${CORE_HDR})
add_library(core ${CORE_SRC} ${CORE_HDR})

set_target_properties(core PROPERTIES LINK_FLAGS "-s DEMANGLE_SUPPORT=1 --preload-file ${CMAKE_SOURCE_DIR}/assets --bind")

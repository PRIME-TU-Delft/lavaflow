cmake_minimum_required(VERSION 3.0.0)
project(3D_model_construction VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(3D_model_construction main.cpp)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

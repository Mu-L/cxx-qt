# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

cmake_minimum_required(VERSION 3.24)

project(tests_qt_types_standalone)
set(APP_NAME ${PROJECT_NAME})

# Rust always links against non-debug Windows runtime on *-msvc targets
# Note it is best to set this on the command line to ensure all targets are consistent
# https://github.com/corrosion-rs/corrosion/blob/master/doc/src/common_issues.md#linking-debug-cc-libraries-into-rust-fails-on-windows-msvc-targets
# https://github.com/rust-lang/rust/issues/39016
if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT USE_QT5)
    find_package(Qt6 COMPONENTS Core Gui Qml Test QuickControls2)
    set(Qt "Qt6")
endif()
if(NOT Qt6_FOUND)
    find_package(Qt5 5.15 COMPONENTS Core Gui Qml Test QuickControls2 REQUIRED)
    set(Qt "Qt5")
endif()

if(MSVC)
    # Qt also needs to link against the non-debug version of the MSVC Runtime libraries, see the previous comment.
    # Note: The Qt:: targets are ALIAS targets that do not support setting properties directly.
    # We therefore need to resolve the target names to either Qt5 or Qt6 directly.
    set_property(
        TARGET ${Qt}::Core ${Qt}::Gui ${Qt}::Qml ${Qt}::Test ${Qt}::QuickControls2
        PROPERTY MAP_IMPORTED_CONFIG_DEBUG "RELEASE")
endif()

find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
        CxxQt
        GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
        GIT_TAG main
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

set(CRATE qt_types_standalone)
cxx_qt_import_crate(
    MANIFEST_PATH rust/Cargo.toml
    CRATES ${CRATE}
    LOCKED
    QT_MODULES Qt::Core Qt::Gui Qt::Qml
)

if(BUILD_WASN)
    # Add -DRUST_CXX_NO_EXCEPTIONS to CXXFLAGS, as WASM does not support exceptions
    set(EMSCRIPTEN_CXX_FLAGS "${CMAKE_CXX_FLAGS}")
    list(APPEND EMSCRIPTEN_CXX_FLAGS "-DRUST_CXX_NO_EXCEPTIONS")
    corrosion_set_env_vars(${CRATE} "CXXFLAGS=${EMSCRIPTEN_CXX_FLAGS}")
endif()

add_executable(${APP_NAME}
    cpp/main.cpp
    cpp/qbytearray.h
    cpp/qcolor.h
    cpp/qcoreapplication.h
    cpp/qdate.h
    cpp/qdatetime.h
    cpp/qflags.h
    cpp/qguiapplication.h
    cpp/qhash.h
    cpp/qline.h
    cpp/qlinef.h
    cpp/qlist.h
    cpp/qmap.h
    cpp/qmargins.h
    cpp/qmarginsf.h
    cpp/qmetaobjectconnection.h
    cpp/qmodelindex.h
    cpp/qpen.h
    cpp/qpersistentmodelindex.h
    cpp/qpoint.h
    cpp/qpointf.h
    cpp/qpolygon.h
    cpp/qpolygonf.h
    cpp/qqmlapplicationengine.h
    cpp/qqmlengine.h
    cpp/qrect.h
    cpp/qrectf.h
    cpp/qregion.h
    cpp/qset.h
    cpp/qsize.h
    cpp/qsizef.h
    cpp/qstring.h
    cpp/qstringlist.h
    cpp/qtime.h
    cpp/qtimezone.h
    cpp/qurl.h
    cpp/qvariant.h
    cpp/qvector.h
    cpp/qvector2d.h
    cpp/qvector3d.h
    cpp/qvector4d.h
)
if(NOT USE_QT5)
    target_sources(${APP_NAME} PRIVATE
        cpp/qanystringview.h)
endif()
target_include_directories(${APP_NAME} PRIVATE cpp)

target_link_libraries(${APP_NAME} PRIVATE
    ${CRATE}
    Qt::Test
    Qt::Core
    Qt::Gui
    Qt::Qml
    Qt::QuickControls2
)

if (APPLE AND USE_QT5)
    # When building in CI with Qt 5 if we don't link to CoreFoundation
    # we get linker failures due to functions related to timezones
    target_link_libraries(${APP_NAME} PRIVATE "-framework CoreFoundation")
endif()

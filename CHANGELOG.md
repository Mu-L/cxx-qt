<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

<!-- markdownlint-disable MD024 -->
<!-- ^^ MD024 complains about duplicate headers -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KDAB/cxx-qt/compare/v0.7.2...HEAD)

### Added

- `QDateTime::from_string` to parse `QDateTime` from a `QString`.
- `QSet::reserve` to reserve capacity up-front.
- Support for further types: `QUuid`
- New example: Basic greeter app
- Support for further types: `qreal`, `qint64`, `qintptr`, `qsizetype`, `quint64`, `quintptr`
- Support for `cfg` attributes through to C++ generation
- CXX-Qt-build: Improved compile time and propagation of initializers between crates
- CXX-Qt-build: Multi-crate projects are now possible with Cargo and CMake (see `examples/qml_multi_crates`)
- CXX-Qt-build: Allow forcing initialization of crates/QML modules (`cxx_qt::init_crate!`/`cxx_qt::init_qml_module!`)
- Add pure virtual function specified through the `#[cxx_pure]` attribute
- Add wrappers for up and down casting, for all types which inherit from QObject, available for &T, &mut T and Pin<&mut T>
- `#[base = T]` is now supported in `extern "C++Qt"` blocks
- Casting is automatically implemented for qobjects or types which have `#[base = T]` in `"RustQt"` or `"C++Qt"` blocks
- Support for `QMessageLogContext` and sending log messages to the Qt message handler.
- Serde support for further types: `QByteArray`, `QSet`, `QStringList`, `QVector`, `QUrl`
- Added `QEventLoop` to cxx-qt-lib-extras.
- Add `QScopedMetaObjectConnectionGuard`, which is `QMetaObjectConnectionGuard` with a scoped lifetime. `QMetaObjectConnectionGuard` is now a type alias for `QScopedMetaObjectConnectionGuard<'static>`.
- Support for setting Qt log message patterns with `q_set_message_pattern` and formatting log messages ith `q_format_log_message`.
- Implement `IntoIterator` for `&QHash`, `&QList`, `&QMap`, `&QSet`, and `&QVector`.
- Add `QByteArray:from_base64_encoding` and `QByteArray::to_base64`.

### Removed

- CXX-Qt-build: Interface no longer includes compiler definitions (<https://github.com/KDAB/cxx-qt/issues/1165>)
- CXX-Qt-build: Interface no longer includes initializers

## [0.7.2](https://github.com/KDAB/cxx-qt/compare/v0.7.1...v0.7.2) - 2025-04-28

### Added

- `impl cxx_qt::Initialize for X {}` as shorthand for `impl cxx_qt::Constructor<()> for X {}`
- `extern "RustQt"` blocks no longer have to be marked `unsafe`

### Fixed

- cxx-qt-build: Use shorter paths to avoid file path length limitations on Windows
- CMake: Use Release Qt DLLs and runtime for MSVC Debug builds
- Fix some generated links to book page
- `#[inherit]` attributes are no longer ignored in `extern "C++Qt"` blocks - they now trigger an error

## [0.7.1](https://github.com/KDAB/cxx-qt/compare/v0.7.0...v0.7.1) - 2025-03-04

### Added

- Allow creating a `QImage` from an `image::RgbaImage`.

### Fixed

- Prevent clippy from tripping lint
- Build warnings due to unused unsafe blocks since CXX 1.0.130
- On non-Unix platforms, use deep copying rather than symlinking

## [0.7.0](https://github.com/KDAB/cxx-qt/compare/v0.6.1...v0.7.0) - 2024-10-30

### Added

- New example: ToDo app
- `#[auto_cxx_name]` and `#[auto_rust_name]` attributes for `extern` blocks, which will convert the case of names, automatically camelCase for cxx, and snake_case for rust
- Support for further types: `QLine`, `QLineF`, `QImage`, `QPainter`, `QFont`, `QPen`, `QPolygon`, `QPolygonF`, `QRegion`, `QAnyStringView`
- `internal_pointer_mut()` function on `QModelIndex`
- `c_void` in CXX-Qt-lib for easy access to `void *`
- `CxxQtThread` is now marked as `Sync` so that it can be used by reference
- Add cxx-qt-lib-extras crate which contains: `QCommandLineOption`, `QCommandLineParser`, `QElapsedTimer`, `QApplication`
- Serde support for `QString` (requires "serde" feature on cxx-qt-lib)
- A new QuickControls module, which exposes `QQuickStyle`. This module is enabled by default and is behind the `qt_quickcontrols` feature.
- Add support for specifying read write and notify in qproperty macro, including support for custom user defined functions
- Add support for the constant, required, reset and final flags in the qproperty macro
- QObject subclasses can now inherit from other CXX-Qt generated QObject classes
- `BUILD_WASM` CMake option to support WebAssembly builds and a book page for building for WASM
- Add support for cxx_name and rust_name on qproperty attributes which applies to the QProperty generated as well as functions

### Changed

- `VCPKG` is now being removed entirely
- Connection now return a `QMetaObjectConnectionGuard` and `QMetaObjectConnection` is a separate type
- Internal `cxx-qt` headers have moved to the namespace `cxxqt1` and the folder `cxx-qt`
- `cxx-qt-gen` now does not generate code requiring `cxx-qt-lib`, this allows for `cxx-qt-lib` to be optional
- File name is used for CXX bridges rather than module name to match upstream
- `#[qobject]` attribute is now optional on types in `extern "RustQt"`
- `#[qobject]` attribute is now required on types in `extern "C++Qt"`
- `#[qenum]`s now resolve their namespace independently from their associated QObject
- Reworked cxx-qt-build and the integration with CMake
  - Dependencies are now automatically detected and configured by cxx-qt-build
  - Libraries can pass build information to cxx-qt-build in the form of a `cxx_qt_build::Interface`
  - Add CMake wrappers around corrosion to simplify importing crates and qml modules that were built with cxx-qt-build
  - CMake code has been extracted into a separate repository for faster downloads (kdab/cxx-qt-cmake)
- Folder structure of Rust bridges is now considered in the same way as CXX in `CxxQtBuilder`
- `cxx_file_stem` has been removed from `#[cxx_qt::bridge]` and the source file name is now used for generated headers similar to CXX
- Base attribute now takes an ident not a string, e.g. `#[base = ParentClass]` instead of `#[base = "ParentClass"]`
- No Cxx-qt-lib features are on by default now, instead we have a 'full' feature for the previously enabled features, making them opt in
- Removed implicit auto camel conversion with no attributes, instead use cxx_name and rust_name like CXX (this may come back but be explicit opt-in)

### Removed

- `qt_gui` and `qt_qml` features from `cxx-qt-build` they are only used in `cxx-qt-lib(-headers)` now
- `cxx-qt-lib-headers` and `cxx-qt-lib-extras-headers` are now merged into their respective base crates
- `BuildOpts` are replaced by the `Interface` type which does not need to be reiterated by downstream dependencies
- Locking has been removed from the generated QObjects. Qt/User C++ code is responsible for upholding Rusts Safety guarantees.
  - The `cxx_qt::Locking` trait is no longer available.

## [0.6.1](https://github.com/KDAB/cxx-qt/compare/v0.6.0...v0.6.1) - 2024-04-19

### Fixed

- Missing include for `MaybeLockGuard` when using only `extern "C++Qt"` signals
- Fix build issues with Qt 6.7
- Improve handling of Apple frameworks with Qt
- Run qmlcachegen only when required
- Support for building with no Rust or C++ files in the builder script

## [0.6.0](https://github.com/KDAB/cxx-qt/compare/v0.5.3...v0.6.0) - 2023-11-17

### Added

- Allow associated constants, types and macro invocations within `impl qobject::T` blocks
- Ensure that generated Rust code works when `#![deny(missing_docs)]` is enabled
- Ability to connect and disconnect from signals in Rust triggering a function pointer or closure
- `unsafe impl !cxx_qt::Locking for qobject::T` to disable internal locking
- `Deref` is now implemented for `qobject::T` to reach the `T` Rust struct
- Support for C++ only methods by not having a `#[qinvokable]` attribute
- Ability to define a custom C++ Constructor using `cxx_qt::Constructor`
- `cxx_qt::Initialize` trait for easier default-constructor implementation
- `extern "C++Qt"` block support for declaring existing types with methods and signals
- `#[qenum]` attribute for `Q_ENUM` and `Q_ENUM_NS` support
- `qnamespace!` macro to support exposing namespaced enums to QML

### Changed

- Pretty-print errors messages when build script fails
- `QDateTime` API to use `current_date_time` rather than `current_date`
- Always call `qt_build_utils::setup_linker()` in `CxxQtBuilder` and remove the proxy method
- Moved to `syn` 2.0 internally and for any exported `syn` types
- `impl cxx_qt::Threading for qobject::T` now needs to be specified for `qt_thread()` to be available
- `#[cxx_qt::qsignals]` and `#[cxx_qt::inherit]` are now used in an `extern "RustQt"` block as `#[qsignal]` and `#[inherit]`
- `#[qinvokable]` is now defined as a signature in `extern "RustQt"`
- `rust_mut` is now safe to call
- `#[qproperty]` is now defined as an attribute on the qobject rather than the field
- QObject struct is now split between the bridge and implementation outside via a type alias
- `qobject` module is no longer generated
- `impl cxx_qt::trait for qobject::T` inside the bridge is now `impl cxx_qt::trait for T`
- `qobject::T` as the self parameter in the bridge is now `T`
- `#[cxx_override]`, `#[cxx_final]`, `#[cxx_virtual]` are now independant attributes rather than embedded in `#[qinvokable]`
- Use `set_organization_name` instead of `q{core,gui}application_set_organization_name` in cxx-qt-lib

### Fixed

- Do not use -bundle otherwise CMake builds are missing qt-static-initalizers (note this is broken in rustc 1.69)
- Do not import `Pin` in hidden module as invokables are outside now, resolving IDE integration
- Rust always links against a non-debug Windows runtime with \*-msvc targets, so we need to link to MultiThreadedDLL

### Removed

- Removed support for `cxx_type` and `cxx_return_type` and related conversion methods.
- Removed `newCppObject` function that allowed creation of default-constructed QObject from Rust.
- Generation of getter and setter for private Rust fields
- Generation of mutable getter for properties, instead use `rust_mut`

## [0.5.3](https://github.com/KDAB/cxx-qt/compare/v0.5.2...v0.5.3) - 2023-05-19

### Fixed

- Ensure that QVariant{Hash,List,Map} cxx-qt-lib equivalents are registered so that they work in QML
- Stop generating `mut` on self pins unnecessarily

## [0.5.2](https://github.com/KDAB/cxx-qt/compare/v0.5.1...v0.5.2) - 2023-04-27

### Fixed

- Builds failing due to `link modifiers combination +bundle,+whole-archive is unstable when generating rlibs`

## [0.5.1](https://github.com/KDAB/cxx-qt/compare/v0.5.0...v0.5.1) - 2023-03-27

### Fixed

- `qrc` resources added to `CxxQtBuilder` or `QtBuild` now trigger `cargo:rerun-if-changed` for file entries
- Fix not being able to use `QVariant` as a `#[qproperty]`, because the `PartialEq` implementation was missing

## [0.5.0](https://github.com/KDAB/cxx-qt/compare/v0.4.1...v0.5.0) - 2023-03-08

## Added

- Support for inheriting methods from the superclass into Rust using `#[cxx_qt::inherit]`.
- Register QML types at build time: `#[cxxqt::qobject(qml_uri = "foo.bar", qml_version = "1.0")]`
- Register QRC resources at build time in Cargo builds (don't need to call initialization function from Rust `main` function)
- Support for container types: `QSet<T>`, `QHash<K, V>`, `QList<T>`, `QMap<K, V>`, `QVector<T>`
- Support for further types: `QByteArray`, `QCoreApplication`, `QGuiApplication`, `QMargins`, `QMarginsF`, `QModelIndex`, `QPersistentModelIndex`, `QQmlApplicationEngine`, `QQmlEngine`, `QStringList`, `QTimeZone`, `QVector2D`, `QVector3D`, `QVector4D`
- Support for nesting objects in properties, invokables, and signals with `*mut T`
- Allow for marking signals as existing in the base class
- Support for conversions to types in third-party crates: `bytes`, `chrono`, `http`, `rgb`, `time`, `url`
- Add several quality of life functions to builtin cxx-qt-lib types, including `Default` constructors, string formatting, `std::cmp` order and operators

### Changed

- `QVariant` now has a uses a `QVariantValue` trait for supported types, allowing custom types to be used with QVariant
- `QtGui` and `QtQml` types in cxx-qt-lib are now behind the features `qt_gui` and `qt_qml`

### Fixed

- Support for generating correct C++ code for `Pin<T>` Rust types
- Support namespace attribute on shared types, QObject struct, and extern blocks
- Asserts for 32bit platforms such as Wasm
- Errors from the generation not pointing to the span where they occurred

## [0.4.1](https://github.com/KDAB/cxx-qt/compare/v0.4.0...v0.4.1) - 2022-11-18

### Added

- Multiple QObjects can be defined in one bridge

### Fixed

- Fixed linking Qt with macOS frameworks. This allows using Qt from Homebrew.

## [0.4.0](https://github.com/KDAB/cxx-qt/compare/v0.3.0...v0.4.0) - 2022-10-28

### Added

- Addition of qt-build-utils crate
- Add generic system for performing type conversions in C++

### Changed

- Refactor of API so that CXX-Qt is a superset of CXX
- Rewrite of build system, removal of custom cmake file, corrosion is used for CMake and support for building with only Cargo
- Refactor of internal generation code so it's split into stages and removal of pattern matching for types, so arbritary CXX types can be supported
- Mark Qt relocatable types as trivial to CXX
- Use Rust closures to queue tasks onto the Qt thread

### Removed

- Support for nested objects

## [0.3.0](https://github.com/KDAB/cxx-qt/compare/v0.2.1...v0.3.0) - 2022-06-10

### Added

- Add a demo for showing complex threading and async
- Support for declaring and emitting signals

### Changed

- Use CXX itself for bridging Qt types

## [0.2.1](https://github.com/KDAB/cxx-qt/compare/v0.2.0...v0.2.1) - 2022-03-21

### Added

- Add vcpkg for Windows and macOS, then use this for CI
- Support mutable invokables

## [0.2.0](https://github.com/KDAB/cxx-qt/compare/v0.1.0...v0.2.0) - 2022-02-28

### Added

- Add more Qt types, QDate, QDateTime, Qpoint, QRect, QRectF, QSize, QTime, QUrl
- Support Qt 6 for Qt types
- Use a "CppObj" to represent the C++ context
- Add a book for documentation

### Changed

- Use postEvent for emitting events safely

## [0.1.0](https://github.com/KDAB/cxx-qt/releases/tag/v0.1.0) - 2021-12-03

### Added

- Initial release
- Support for bridging properties and invokables
- Support for QColor, QPointF, QSizeF, QString, QVariant
- Support for nested objects

// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_build::CxxQtBuilder;
use std::path::PathBuf;

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt-lib-extras")
}

fn write_headers_in(subfolder: &str) {
    println!("cargo::rerun-if-changed=include/{subfolder}");

    for entry in
        std::fs::read_dir(format!("include/{subfolder}")).expect("Failed to read include directory")
    {
        let entry = entry.expect("Failed to read header file!");
        let header_name = entry.file_name();
        println!(
            "cargo::rerun-if-changed=include/{subfolder}/{header_name}",
            header_name = header_name.to_string_lossy()
        );

        // TODO: Do we want to add the headers into a subdirectory?
        std::fs::copy(entry.path(), header_dir().join(header_name))
            .expect("Failed to copy header file!");
    }
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    write_headers_in("core");
    write_headers_in("gui");
}

fn main() {
    write_headers();

    let mut builder = CxxQtBuilder::new().qt_module("Gui").qt_module("Widgets");

    let rust_bridges = vec![
        "core/qelapsedtimer",
        "core/qeventloop",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
        "gui/qapplication",
    ];

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    let cpp_files = vec![
        "core/qelapsedtimer",
        "core/qcommandlineoption",
        "core/qcommandlineparser",
        "gui/qapplication",
    ];

    builder = builder.cc_builder(move |cc| {
        for cpp_file in &cpp_files {
            cc.file(format!("src/{cpp_file}.cpp"));
            println!("cargo::rerun-if-changed=src/{cpp_file}.cpp");
        }
        cc.file("src/qt_types.cpp");
        println!("cargo::rerun-if-changed=src/qt_types.cpp");
    });
    println!("cargo::rerun-if-changed=src/assertion_utils.h");

    let interface = builder
        // Use a short name due to the Windows file path limit!
        // We don't re-export these headers anyway.
        .include_prefix("private")
        .crate_include_root(Some("include".to_owned()))
        .build();

    // Disable exporting the standard include directory, as we are exporting custom header
    interface.reexport_dependency("cxx-qt-lib").export();
}

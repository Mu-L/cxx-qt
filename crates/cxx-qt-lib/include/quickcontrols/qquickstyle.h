// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

// The definitions file is auto-generated by the build script
#include <cxx-qt-lib/definitions.h>

#ifdef CXX_QT_QUICKCONTROLS_FEATURE

#include <memory>

#include <QtQuickControls2/QQuickStyle>

namespace rust {
namespace cxxqtlib1 {

inline QString (*qquickstyleName)() = QQuickStyle::name;

inline void (*qquickstyleSetFallbackStyle)(const QString&) =
  QQuickStyle::setFallbackStyle;

inline void (*qquickstyleSetStyle)(const QString&) = QQuickStyle::setStyle;

}
}

#endif

// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cinttypes>
#include <memory>

#include <QtCore/QDateTime>
#include <QtCore/QTimeZone>

#include "rust/cxx.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust {

template<>
struct IsRelocatable<QDateTime> : ::std::true_type
{};

} // namespace rust

namespace rust {
namespace cxxqtlib1 {

inline QDateTime (*qdatetimeCurrentDateTime)() = QDateTime::currentDateTime;
inline QDateTime (*qdatetimeCurrentDateTimeUtc)() =
  QDateTime::currentDateTimeUtc;
inline qint64 (*qdatetimeCurrentMSecsSinceEpoch)() =
  QDateTime::currentMSecsSinceEpoch;
inline qint64 (*qdatetimeCurrentSecsSinceEpoch)() =
  QDateTime::currentSecsSinceEpoch;

inline QDateTime (*qdatetimeFromMSecsSinceEpoch)(qint64, const QTimeZone&) =
  QDateTime::fromMSecsSinceEpoch;
inline QDateTime (*qdatetimeFromSecsSinceEpoch)(qint64, const QTimeZone&) =
  QDateTime::fromSecsSinceEpoch;

void
qdatetimeSetDate(QDateTime& datetime, QDate date);
void
qdatetimeSetTime(QDateTime& datetime, QTime time);
::std::unique_ptr<QTimeZone>
qdatetimeTimeZone(const QDateTime& datetime);
void
qdatetimeSetTimeZone(QDateTime& datetime, const QTimeZone& timeZone);
QDateTime
qdatetimeFromQString(const QString& string, Qt::DateFormat format);
}
}

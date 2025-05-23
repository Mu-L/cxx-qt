// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include <QtCore/QThread>
#include <QtCore/QTimer>
#include <QtTest/QSignalSpy>
#include <QtTest/QTest>

#include "basic_cxx_qt/src/data.cxxqt.h"
#include "basic_cxx_qt/src/empty.cxxqt.h"
#include "basic_cxx_qt/src/lib.cxxqt.h"
#include "basic_cxx_qt/src/naming.cxxqt.h"
#include "basic_cxx_qt/src/types.cxxqt.h"

class CxxQtTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  // CXX-Qt allows basic interaction between C++ (with Qt) and Rust
  void test_basic_interaction()
  {
    cxx_qt::my_object::MyObject obj;
    object_namespace::RenamedObject obj2;
    bridge_namespace::NamedObject obj3;

    obj.sayHi(QStringLiteral("Hello World!"), 32);

    const auto value1 = obj2.getDouble();
    QCOMPARE(value1, 50);

    obj3.sayHi(QStringLiteral("Hello World!"), 32);
    // Check that an invokable can be called and the return value is correct
    const auto value = obj.doubleNumber(32);
    qInfo() << "Double of 32 is:" << value;
    QCOMPARE(value, 64);

    // Track the signal count of numberChanged, stringChanged
    QSignalSpy numberSpy(&obj, &cxx_qt::my_object::MyObject::numberChanged);
    QSignalSpy stringSpy(&obj, &cxx_qt::my_object::MyObject::stringChanged);

    // Check the number property
    QCOMPARE(obj.getNumber(), 0);
    QCOMPARE(numberSpy.count(), 0);
    obj.setNumber(16);
    QCOMPARE(numberSpy.count(), 1);
    QCOMPARE(obj.getNumber(), 16);

    // Check the string property
    QCOMPARE(obj.getString(), QString());
    QCOMPARE(stringSpy.count(), 0);
    obj.setString(QStringLiteral("Hello"));
    QCOMPARE(stringSpy.count(), 1);
    QCOMPARE(obj.getString(), QStringLiteral("Hello"));

    // Check the double number self
    QCOMPARE(obj.getNumber(), 16);
    QCOMPARE(numberSpy.count(), 1);
    obj.doubleNumberSelf();
    QCOMPARE(obj.getNumber(), 32);
    QCOMPARE(numberSpy.count(), 2);

    qInfo() << "Number is:" << obj.getNumber()
            << "String is:" << obj.getString();
  }

  // CXX-Qt allows basic interaction between C++ (with Qt) and Rust using Serde
  void test_basic_interaction_serde()
  {
    cxx_qt::my_data::MyData data;

    // Track the signal count of numberChanged, stringChanged
    QSignalSpy numberSpy(&data, &cxx_qt::my_data::MyData::numberChanged);
    QSignalSpy stringSpy(&data, &cxx_qt::my_data::MyData::stringChanged);

    // Check that initial value of the deserialised data
    QCOMPARE(data.getNumber(), 4);
    QCOMPARE(data.getString(), QStringLiteral("Hello World!"));

    // Check the number changed property
    QCOMPARE(numberSpy.count(), 0);
    data.setNumber(16);
    QCOMPARE(numberSpy.count(), 1);
    QCOMPARE(data.getNumber(), 16);

    // Check the string property
    QCOMPARE(stringSpy.count(), 0);
    data.setString(QStringLiteral("Hello"));
    QCOMPARE(stringSpy.count(), 1);
    QCOMPARE(data.getString(), QStringLiteral("Hello"));

    // Check that initial value of the deserialised data
    QCOMPARE(data.asJsonStr(),
             QStringLiteral("{\"number\":16,\"string\":\"Hello\"}"));
  }

  // CXX-Qt allows Rust code to queue a request
  void test_queue_request()
  {
    cxx_qt::my_object::MyObject obj;
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    obj.queueTest();
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    QTRY_COMPARE(obj.fetchUpdateCallCount(), 1);
  }

  // CXX-Qt allows Rust code to queue multiple requests
  void test_queue_multiple_requests()
  {
    cxx_qt::my_object::MyObject obj;
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    obj.queueTest();
    obj.queueTest();
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    QTRY_COMPARE(obj.fetchUpdateCallCount(), 2);
  }

  // CXX-Qt allows Rust code to queue requests in multiple threads
  void test_queue_requests_multiple_threads()
  {
    cxx_qt::my_object::MyObject obj;
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    obj.queueTestMultiThread();
    QCOMPARE(obj.fetchUpdateCallCount(), 0);
    QTRY_COMPARE(obj.fetchUpdateCallCount(), 100);
  }

  // CXX-Qt types are exposed to C++ correctly
  void test_primitive_types()
  {
    cxx_qt::my_types::MyTypes types;

    QSignalSpy booleanSpy(&types, &cxx_qt::my_types::MyTypes::booleanChanged);
    QSignalSpy float32Spy(&types, &cxx_qt::my_types::MyTypes::float_32Changed);
    QSignalSpy float64Spy(&types, &cxx_qt::my_types::MyTypes::float_64Changed);
    QSignalSpy int8Spy(&types, &cxx_qt::my_types::MyTypes::int_8Changed);
    QSignalSpy int16Spy(&types, &cxx_qt::my_types::MyTypes::int_16Changed);
    QSignalSpy int32Spy(&types, &cxx_qt::my_types::MyTypes::int_32Changed);
    QSignalSpy uint8Spy(&types, &cxx_qt::my_types::MyTypes::uint_8Changed);
    QSignalSpy uint16Spy(&types, &cxx_qt::my_types::MyTypes::uint_16Changed);
    QSignalSpy uint32Spy(&types, &cxx_qt::my_types::MyTypes::uint_32Changed);

    QCOMPARE(types.getBoolean(), false);
    QCOMPARE(booleanSpy.count(), 0);
    types.setBoolean(true);
    QCOMPARE(booleanSpy.count(), 1);
    QCOMPARE(types.getBoolean(), true);

    QCOMPARE(types.getFloat_32(), 0.0);
    QCOMPARE(float32Spy.count(), 0);
    types.setFloat_32(0.33f);
    QCOMPARE(float32Spy.count(), 1);
    QCOMPARE(types.getFloat_32(), 0.33f);

    QCOMPARE(types.getFloat_64(), 0.0);
    QCOMPARE(float64Spy.count(), 0);
    types.setFloat_64(0.33);
    QCOMPARE(float64Spy.count(), 1);
    QCOMPARE(types.getFloat_64(), 0.33);

    QCOMPARE(types.getInt_8(), 0);
    QCOMPARE(int8Spy.count(), 0);
    types.setInt_8(4);
    QCOMPARE(int8Spy.count(), 1);
    QCOMPARE(types.getInt_8(), 4);

    QCOMPARE(types.getInt_16(), 0);
    QCOMPARE(int16Spy.count(), 0);
    types.setInt_16(4);
    QCOMPARE(int16Spy.count(), 1);
    QCOMPARE(types.getInt_16(), 4);

    QCOMPARE(types.getInt_32(), 0);
    QCOMPARE(int32Spy.count(), 0);
    types.setInt_32(4);
    QCOMPARE(int32Spy.count(), 1);
    QCOMPARE(types.getInt_32(), 4);

    QCOMPARE(types.getUint_8(), 0);
    QCOMPARE(uint8Spy.count(), 0);
    types.setUint_8(4);
    QCOMPARE(uint8Spy.count(), 1);
    QCOMPARE(types.getUint_8(), 4);

    QCOMPARE(types.getUint_16(), 0);
    QCOMPARE(uint16Spy.count(), 0);
    types.setUint_16(4);
    QCOMPARE(uint16Spy.count(), 1);
    QCOMPARE(types.getUint_16(), 4);

    QCOMPARE(types.getUint_32(), 0);
    QCOMPARE(uint32Spy.count(), 0);
    types.setUint_32(4);
    QCOMPARE(uint32Spy.count(), 1);
    QCOMPARE(types.getUint_32(), 4);
  }

  // Tests that we can build an empty QObject end to end
  void testEmpty() { Empty empty; }

#ifndef RUST_CXX_NO_EXCEPTION
  void testThrowException()
  {
    cxx_qt::my_object::MyObject obj;
    bool thrown = false;
    try {
      obj.throwException();
      Q_UNREACHABLE();
    } catch (const rust::Error& e) {
      QCOMPARE(e.what(), "RustException");
      thrown = true;
    }

    QCOMPARE(thrown, true);
  }
#endif
};

QTEST_MAIN(CxxQtTest)
#include "main.moc"

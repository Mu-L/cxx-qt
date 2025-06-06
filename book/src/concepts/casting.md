<!--
SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Casting

With the [base](../bridge/attributes.md) attribute, it is possible to inherit from another type.
In order to access this parent class, we provide an API to cast up or down.
Currently, this is supported for objects in both `extern "RustQt"` *and* `extern "C++Qt"` blocks, which have either a `#[qobject]` attribute,
or a `#[base = T]` attribute. See the [attributes documentation](../bridge/attributes.md) for more details on these attributes.
> Note: Types in "C++Qt" blocks are **required** to have the `#[qobject]` attribute

## Accessing the base class

To access the methods of a base class in Rust, use the `Upcast` trait like so `use cxx_qt::casting::Upcast;`.
Objects with base classes can then be accessed with the following methods

| Self Type        | Method         |
|------------------|----------------|
| `&self`          | `upcast()`     |
| `&mut self`      | `upcast_mut()` |
| `Pin<&mut self>` | `upcast_pin()` |

This will then return a reference to the base in the same format as the self type, e.g. `upcast()` returns `&Base`, etc...

## Accessing the child class

This also works in the opposite direction, allowing access to the child a base class was obtained from.
To do this, use the `Downcast` trait like so `use cxx_qt::Downcast;`.
The child can then be accessed in the same manner, with the following methods

| Self Type        | Method           |
|------------------|------------------|
| `&self`          | `downcast()`     |
| `&mut self`      | `downcast_mut()` |
| `Pin<&mut self>` | `downcast_pin()` |

These will return an `Option<T>`, as it is possible that downcasting will fail,
if the type is not actually of the given subclass,
and these also return in the same format as the self type, e.g. `downcast()` returns `Option<&Sub>`, etc...

## Transitive casting

Given 3 types, where there is a grandparent relationship, e.g. that using 2 casts, you can go from A -> B -> C,
CXX-Qt inlcudes a macro for automatically implementing a cast between A and C. This property also extends for longer chains.
For example, if you have a deeply nested set of inheritance, you can quickly generate helpers to cast from your child type to any of its ancestors.

```rust, ignore
use cxx_qt::impl_transitive_cast;

impl_transitive_cast!(A, B, C, D);
```

Will generate casting from A -> C, and A -> D, **provided** A -> B -> C -> D is already implemented.

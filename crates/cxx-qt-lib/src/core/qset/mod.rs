// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#[cfg(not(target_os = "emscripten"))]
use crate::QDateTime;
use crate::{QByteArray, QDate, QPersistentModelIndex, QString, QTime, QUrl, QUuid};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};
use std::fmt;

/// The `QSet` class is a template class that provides a hash-table-based set.
///
/// Note that this means that `T` needs to have a global [`qHash()`](https://doc.qt.io/qt/qhash.html#qHash) function.
///
/// To use `QSet` with a custom type, implement the [`QSetElement`] trait for `T`.
///
/// Qt Documentation: [QSet]("https://doc.qt.io/qt/qset.html#details")
#[repr(C)]
pub struct QSet<T>
where
    T: QSetElement,
{
    _space: MaybeUninit<usize>,
    _value: PhantomData<T>,
}

impl<T> Clone for QSet<T>
where
    T: QSetElement,
{
    /// Constructs a copy of the `QSet`.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QSet<T>
where
    T: QSetElement,
{
    /// Constructs an empty set.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QSet<T>
where
    T: QSetElement,
{
    /// Destroys the `QSet`.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> PartialEq for QSet<T>
where
    T: QSetElement + PartialEq,
{
    /// Returns `true` if the sets contain the same (key, value) pairs, otherwise `false`.
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|x| other.contains(x))
    }
}

impl<T> Eq for QSet<T> where T: QSetElement + PartialEq {}

impl<T> fmt::Debug for QSet<T>
where
    T: QSetElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> QSet<T>
where
    T: QSetElement,
{
    /// Removes all elements from the set.
    pub fn clear(&mut self) {
        T::clear(self);
    }

    /// Returns `true` if the set contains item `value`; otherwise returns `false`.
    pub fn contains(&self, value: &T) -> bool {
        T::contains(self, value)
    }

    /// Inserts item `value` into the set, if `value` isn't already in the set.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the set.
    pub fn insert_clone(&mut self, value: &T) {
        T::insert_clone(self, value);
    }

    /// Returns `true` if the set contains no elements; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            set: self,
            index: 0,
        }
    }

    /// Returns the number of items in the set.
    pub fn len(&self) -> isize {
        T::len(self)
    }

    /// Removes any occurrence of item `value` from the set.
    /// Returns `true` if an item was actually removed; otherwise returns `false`.
    pub fn remove(&mut self, value: &T) -> bool {
        T::remove(self, value)
    }

    /// Ensures that the set's internal hash table consists of at least `size` buckets.
    ///
    /// This function is useful for code that needs to build a huge set and wants to avoid repeated reallocation.
    ///
    /// Ideally, `size` should be slightly more than the maximum number of elements expected in the set. `size` doesn't have to be prime, because `QSet` will use a prime number internally anyway. If `size` is an underestimate, the worst that will happen is that the `QSet` will be a bit slower.
    ///
    /// In general, you will rarely ever need to call this function. `QSet`'s internal hash table automatically shrinks or grows to provide good performance without wasting too much memory.
    pub fn reserve(&mut self, size: isize) {
        T::reserve(self, size);
    }

    /// Helper function for handling Rust values.
    pub(crate) fn reserve_usize(&mut self, size: usize) {
        if size != 0 {
            T::reserve(self, isize::try_from(size).unwrap_or(isize::MAX));
        }
    }
}

impl<T> QSet<T>
where
    T: QSetElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts item `value` into the set, if `value` isn't already in the set.
    pub fn insert(&mut self, value: T) {
        T::insert(self, value);
    }
}

impl<'a, T> Extend<&'a T> for QSet<T>
where
    T: QSetElement,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve_usize(iter.size_hint().0);
        for element in iter {
            self.insert_clone(element);
        }
    }
}

impl<T> Extend<T> for QSet<T>
where
    T: QSetElement + ExternType<Kind = cxx::kind::Trivial>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve_usize(iter.size_hint().0);
        for element in iter {
            self.insert(element);
        }
    }
}

impl<'a, T> FromIterator<&'a T> for QSet<T>
where
    T: QSetElement,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut qset = Self::default();
        qset.extend(iter);
        qset
    }
}

impl<T> FromIterator<T> for QSet<T>
where
    T: QSetElement + ExternType<Kind = cxx::kind::Trivial>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut qset = Self::default();
        qset.extend(iter);
        qset
    }
}

unsafe impl<T> ExternType for QSet<T>
where
    T: ExternType + QSetElement,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QSetElement,
{
    set: &'a QSet<T>,
    index: isize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QSetElement,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.set.len() {
            let next = unsafe { T::get_unchecked(self.set, self.index) };
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: QSetElement,
{
    fn len(&self) -> usize {
        (self.set.len() - self.index) as usize
    }
}

impl<'a, T> IntoIterator for &'a QSet<T>
where
    T: QSetElement,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Trait implementation for an element in a [`QSet`].
pub trait QSetElement: Sized {
    type TypeId;

    fn clear(set: &mut QSet<Self>);
    fn clone(set: &QSet<Self>) -> QSet<Self>;
    fn contains(set: &QSet<Self>, value: &Self) -> bool;
    fn default() -> QSet<Self>;
    fn drop(set: &mut QSet<Self>);
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(set: &QSet<Self>, pos: isize) -> &Self;
    fn insert(set: &mut QSet<Self>, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(set: &mut QSet<Self>, value: &Self);
    fn len(set: &QSet<Self>) -> isize;
    fn remove(set: &mut QSet<Self>, value: &Self) -> bool;
    fn reserve(set: &mut QSet<Self>, size: isize);
}

macro_rules! impl_qset_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        mod $module;

        impl QSetElement for $typeName {
            type TypeId = type_id!($typeId);

            fn clear(set: &mut QSet<Self>) {
                set.cxx_clear()
            }

            fn clone(set: &QSet<Self>) -> QSet<Self> {
                $module::clone(set)
            }

            fn contains(set: &QSet<Self>, value: &Self) -> bool {
                set.cxx_contains(value)
            }

            fn default() -> QSet<Self> {
                $module::default()
            }

            fn drop(set: &mut QSet<Self>) {
                $module::drop(set);
            }

            unsafe fn get_unchecked(set: &QSet<Self>, pos: isize) -> &Self {
                $module::get_unchecked(set, pos)
            }

            fn insert(set: &mut QSet<Self>, value: Self) {
                $module::insert(set, &value);
            }

            fn insert_clone(set: &mut QSet<Self>, value: &Self) {
                $module::insert(set, value);
            }

            fn len(set: &QSet<Self>) -> isize {
                $module::len(set)
            }

            fn remove(set: &mut QSet<Self>, value: &Self) -> bool {
                set.cxx_remove(value)
            }

            fn reserve(set: &mut QSet<Self>, size: isize) {
                $module::reserve(set, size);
            }
        }
    };
}

impl_qset_element!(bool, qset_bool, "QSet_bool");
impl_qset_element!(f32, qset_f32, "QSet_f32");
impl_qset_element!(f64, qset_f64, "QSet_f64");
impl_qset_element!(i8, qset_i8, "QSet_i8");
impl_qset_element!(i16, qset_i16, "QSet_i16");
impl_qset_element!(i32, qset_i32, "QSet_i32");
impl_qset_element!(i64, qset_i64, "QSet_i64");
impl_qset_element!(QByteArray, qset_qbytearray, "QSet_QByteArray");
impl_qset_element!(QDate, qset_qdate, "QSet_QDate");
#[cfg(not(target_os = "emscripten"))]
impl_qset_element!(QDateTime, qset_qdatetime, "QSet_QDateTime");
impl_qset_element!(
    QPersistentModelIndex,
    qset_qpersistentmodelindex,
    "QSet_QPersistentModelIndex"
);
impl_qset_element!(QString, qset_qstring, "QSet_QString");
impl_qset_element!(QTime, qset_qtime, "QSet_QTime");
impl_qset_element!(QUrl, qset_qurl, "QSet_QUrl");
impl_qset_element!(QUuid, qset_quuid, "QSet_QUuid");
impl_qset_element!(u8, qset_u8, "QSet_u8");
impl_qset_element!(u16, qset_u16, "QSet_u16");
impl_qset_element!(u32, qset_u32, "QSet_u32");
impl_qset_element!(u64, qset_u64, "QSet_u64");

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn qset_serde() {
        let mut set = QSet::default();
        set.insert(0);
        set.insert(1);
        set.insert(2);
        assert_eq!(crate::serde_impl::roundtrip(&set), set)
    }
}

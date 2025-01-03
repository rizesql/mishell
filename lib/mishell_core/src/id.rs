use std::any;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<Marker, Val = usize> {
    inner: Val,
    _phantom: PhantomData<Marker>,
}

impl<M, V> Id<M, V> {
    #[inline]
    pub const fn new(inner: V) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<M, V: Copy> Id<M, V> {
    #[inline]
    pub const fn get(self) -> V {
        self.inner
    }
}

impl<M, V: Display> Debug for Id<M, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let marker = any::type_name::<M>().split("::").last().expect("not empty");
        write!(f, "{marker}Id({})", self.inner)
    }
}

impl<M, V: Serialize> Serialize for Id<M, V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl<'de, M, V: Deserialize<'de>> Deserialize<'de> for Id<M, V> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            inner: V::deserialize(deserializer)?,
            _phantom: PhantomData,
        })
    }
}

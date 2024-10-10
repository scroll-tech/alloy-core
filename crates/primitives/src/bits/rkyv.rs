use core::fmt::{Debug, Formatter};
use std::hash::Hash;
use super::*;

impl From<&ArchivedAddress> for Address {
    fn from(archived: &ArchivedAddress) -> Self {
        Address::from(archived.0.0)
    }
}

impl From<ArchivedAddress> for Address {
    fn from(archived: ArchivedAddress) -> Self {
        Address::from(archived.0.0)
    }
}

impl Debug for ArchivedAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&Address::from(self.0.0), f)
    }
}

impl<const N: usize> From<&ArchivedFixedBytes<N>> for FixedBytes<N> {
    fn from(archived: &ArchivedFixedBytes<N>) -> Self {
        FixedBytes(archived.0)
    }
}

impl<const N: usize> From<ArchivedFixedBytes<N>> for FixedBytes<N> {
    fn from(archived: ArchivedFixedBytes<N>) -> Self {
        FixedBytes(archived.0)
    }
}

impl<const N: usize> Debug for ArchivedFixedBytes<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&FixedBytes(self.0), f)
    }
}

impl<const N: usize> Copy for ArchivedFixedBytes<N> {}

impl<const N: usize> Clone for ArchivedFixedBytes<N> {
    fn clone(&self) -> Self {
        ArchivedFixedBytes(self.0)
    }
}

impl<const N: usize> PartialEq for ArchivedFixedBytes<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> Eq for ArchivedFixedBytes<N> {}

impl<const N: usize> Hash for ArchivedFixedBytes<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
use crate::{Address, FixedBytes};
use rkyv::{Archive, Archived, CheckBytes, Deserialize, Fallible, Serialize};
use std::convert::Infallible;

impl Archive for Address {
    type Archived = Address;
    type Resolver = <[u8; 20] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.0.resolve(pos, resolver, out as *mut [u8; 20]);
    }
}

impl<C: ?Sized> CheckBytes<C> for Address {
    type Error = Infallible;
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for Address {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Serialize::<S>::serialize(&self.0, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Address, D> for Archived<Address>
{
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<Address, D::Error> {
        Ok(*self)
    }
}


impl<const N: usize> Archive for FixedBytes<N> {
    type Archived = FixedBytes<N>;
    type Resolver = <[u8; N] as Archive>::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        self.0.resolve(pos, resolver, out as *mut [u8; N]);
    }
}

impl<C: ?Sized, const N: usize> CheckBytes<C> for FixedBytes<N> {
    type Error = Infallible;
    unsafe fn check_bytes<'a>(value: *const Self, _: &mut C) -> Result<&'a Self, Self::Error> {
        Ok(&*value)
    }
}

impl<S: Fallible + ?Sized, const N: usize> Serialize<S> for FixedBytes<N> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Serialize::<S>::serialize(&self.0, serializer)
    }
}

impl<D: Fallible + ?Sized, const N: usize> Deserialize<FixedBytes<N>, D>
    for Archived<FixedBytes<N>>
{
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<FixedBytes<N>, D::Error> {
        Ok(*self)
    }
}

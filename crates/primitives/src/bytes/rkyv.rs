use crate::Bytes;
use bytes::BytesMut;
use rkyv::{
    bytecheck::CheckBytes,
    rancor::Fallible,
    ser::{Allocator, Writer},
    vec::{ArchivedVec, VecResolver},
    Archive, Deserialize, Place, Serialize,
};
use rkyv::rancor::{Source, Trace};
use rkyv::validation::ArchiveContext;

impl Archive for Bytes {
    type Archived = ArchivedVec<u8>;
    type Resolver = VecResolver;

    #[inline]
    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        ArchivedVec::resolve_from_slice(self.as_ref(), resolver, out);
    }
}

impl<S: Fallible + Allocator + Writer + ?Sized> Serialize<S> for Bytes {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Bytes, D> for ArchivedVec<u8> {
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<Bytes, D::Error> {
        let mut result = BytesMut::new();
        result.extend_from_slice(self.as_slice());
        Ok(Bytes(result.freeze()))
    }
}

unsafe impl<C: Fallible+ ArchiveContext + Sized> CheckBytes<C> for Bytes
where <C as Fallible>::Error: Source + Trace,
{
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
        ArchivedVec::<u8>::check_bytes(value as *const ArchivedVec<u8>, context)
    }
}

use crate::Bytes;
use rkyv::{Archive, Archived, Deserialize, Fallible, Serialize};
use bytes::BytesMut;
use rkyv::ser::{ScratchSpace, Serializer};
use rkyv::vec::{ArchivedVec, VecResolver};

impl Archive for Bytes {
    type Archived = ArchivedVec<u8>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_ref(), pos, resolver, out);
    }
}

impl<S: ScratchSpace + Serializer + ?Sized> Serialize<S> for Bytes {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Bytes, D> for ArchivedVec<Archived<u8>> {
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<Bytes, D::Error> {
        let mut result = BytesMut::new();
        result.extend_from_slice(self.as_slice());
        Ok(Bytes(result.freeze()))
    }
}

#[macro_use]
mod macros;

mod address;
pub use address::{Address, AddressChecksumBuffer, AddressError};
#[cfg(feature = "rkyv")]
pub use address::{ArchivedAddress, AddressResolver};

mod bloom;
pub use bloom::{Bloom, BloomInput, BLOOM_BITS_PER_ITEM, BLOOM_SIZE_BITS, BLOOM_SIZE_BYTES};

mod fixed;
pub use fixed::FixedBytes;
#[cfg(feature = "rkyv")]
pub use fixed::{ArchivedFixedBytes, FixedBytesResolver};

mod function;
pub use function::Function;

#[cfg(feature = "rlp")]
mod rlp;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rkyv")]
mod rkyv;

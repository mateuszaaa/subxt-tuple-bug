//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

use polkadot_sdk::polkadot_sdk_frame as frame;

use polkadot_sdk::frame_support::pallet_prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet]
pub mod pallet {

    use polkadot_sdk::sp_runtime::Serialize;

    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type TupleStorageMap<T: Config> =
        StorageMap<_, Blake2_128Concat, (u32, u32), Option<u32>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        _phantom: PhantomData<T>,
    }

    #[pallet::storage]
    pub type TupleWrapperStorageMap<T: Config> =
        StorageMap<_, Blake2_128Concat, TupleWrapper, Option<u32>, ValueQuery>;

    #[derive(
        Default, Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Copy, MaxEncodedLen
    )]
    pub struct TupleWrapper(pub (u32, u32));

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> GenesisConfig<T> {
            GenesisConfig {
                _phantom: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            TupleStorageMap::<T>::insert((1u32, 2u32), Some(3u32));
            TupleWrapperStorageMap::<T>::insert(TupleWrapper((1u32, 2u32)), Some(4u32));
        }
    }
}

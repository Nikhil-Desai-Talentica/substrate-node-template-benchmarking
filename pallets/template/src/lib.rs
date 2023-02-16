#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// #[pallet::constant]
		// type MaxKeys: Get<u32>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn some_num)]
	pub type SomeNum<T> = StorageValue<_, u64>;
	// pub(super) type UINTStore<T: Config> = StorageValue<_, BoundedVec<u64, T::MaxKeys>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// NumKeysWroteToStorage(u32),
		// NumKeysReadFromStorage(u32),
		ValueChanged {old: u64, new: u64},
	}

	// #[pallet::error]
	// pub enum Error<T> {
	// 	NumKeysExceedsMax,
	// }

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_some_num(origin: OriginFor<T>, value: u64) -> DispatchResult {

			let _sender = ensure_signed(origin)?;

			// ensure!(num_keys <= T::MaxKeys, Error::<T>::NumKeysExceedsMax);

			// for i in 0..num_keys {
			// 	T::UINTStore[i] = value;
			// }

			let maybe_old_num = <SomeNum<T>>::get();
			let old: u64 = maybe_old_num.unwrap_or_default();
			<SomeNum<T>>::put(value);

			Self::deposit_event(Event::ValueChanged { old: old, new: value});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1).ref_time())]
		pub fn get_some_num(origin: OriginFor<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let maybe_some_num = <SomeNum<T>>::get();

			let _some_num = maybe_some_num.unwrap_or_default();

			Ok(())
		}
	}
}

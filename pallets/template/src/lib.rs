#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_callee::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// #[pallet::constant]
		// type MaxKeys: Get<u32>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	// `SomeNum` stores an integer
	#[pallet::storage]
	#[pallet::getter(fn some_num)]
	pub type SomeNum<T> = StorageValue<_, i64>;

	// `SomeStr` stores a byte vector of arbitrary length
	#[pallet::storage]
	#[pallet::getter(fn some_str)]
	pub type SomeStr<T> = StorageValue<_, Vec<u8>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Event emitted when the value of `SomeNum` is changed
		NumChanged {old: i64, new: i64},
		// Event emitted when the value of `SomeStr` is changed
		StrChanged,
		// Sample event emitted to benchmark event emission
		SampleEmit,
	}

	fn fib(n: u32) -> u128 {
		if n < 2 {
			n.into()
		} else {
			fib(n-1).wrapping_add(fib(n-2))
		}
	}

	fn odd_prod(n: u32) -> u128 {
		(1..=n as u128).fold(1, |prod, x| prod.wrapping_mul(2 * x - 1))
	}

	fn triangle_num(n: u32) -> u128 {
		(1..=n as u128).fold(0, |sum, x| sum.wrapping_add(x))
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

		// Change the value of `SomeNum` to the passed in `value`
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_some_num(origin: OriginFor<T>, value: i64) -> DispatchResult {

			let _sender = ensure_signed(origin)?;

			// ensure!(num_keys <= T::MaxKeys, Error::<T>::NumKeysExceedsMax);

			// for i in 0..num_keys {
			// 	T::UINTStore[i] = value;
			// }

			let maybe_old_num = <SomeNum<T>>::get();
			let old: i64 = maybe_old_num.unwrap_or_default();
			<SomeNum<T>>::put(value);

			Self::deposit_event(Event::NumChanged { old: old, new: value });

			Ok(())
		}

		// Read the value of `SomeNum`
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1).ref_time())]
		pub fn get_some_num(origin: OriginFor<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let maybe_some_num = <SomeNum<T>>::get();

			let _some_num = maybe_some_num.unwrap_or_default();

			Ok(())
		}

		// Change the value of `SomeStr` to the value passed in as `new_str`
		#[pallet::call_index(2)]
		#[pallet::weight(50_000_000)]
		pub fn update_some_str(origin: OriginFor<T>, new_str: Vec<u8>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let maybe_some_str = <SomeStr<T>>::get();

			let _old_str = maybe_some_str.unwrap_or_default();

			<SomeStr<T>>::put(new_str);

			Self::deposit_event(Event::StrChanged);

			Ok(())
		}

		// Read the value contained in `SomeStr`
		#[pallet::call_index(3)]
		#[pallet::weight(50_000_000)]
		pub fn get_some_str(origin: OriginFor<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let maybe_some_str = <SomeStr<T>>::get();

			let _old_str = maybe_some_str.unwrap_or_default();

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn fibonacci(origin: OriginFor<T>, n: u32) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let fib_n = fib(n);

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(10_000)]
		pub fn odd_product(origin: OriginFor<T>, n: u32) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let odd_product_n = odd_prod(n);

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(10_000)]
		pub fn triangle_number(origin: OriginFor<T>, n: u32) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let triangle_number_n = triangle_num(n);

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(10_000)]
		pub fn emit_sample_event(origin: OriginFor<T>) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			Self::deposit_event(Event::SampleEmit);

			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(10_000)]
		pub fn store_num_in_callee(origin: OriginFor<T>, value: i64) -> DispatchResult {
			pallet_callee::Pallet::<T>::store_num(origin, value);
			Ok(())
		}
	}
}

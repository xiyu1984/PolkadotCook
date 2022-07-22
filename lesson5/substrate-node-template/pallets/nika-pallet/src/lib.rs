#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {    
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type StarKeeper<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    // Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Register(T::AccountId, Vec::<u8>),
		UnRegister(T::AccountId, Vec::<u8>),
		Transferred(T::AccountId, T::AccountId, Vec::<u8>),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		AlreadyExist,
		NotExist,
		OwnerNeeded,
		TransferFailed,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn register(origin: OriginFor<T>, certificate: Vec<u8>) -> DispatchResult {
			let owner = ensure_signed(origin)?;

            // Check if the claim exists.
            ensure!(!StarKeeper::<T>::contains_key(&certificate), Error::<T>::AlreadyExist);

			// Update storage.
			<StarKeeper<T>>::insert(
                &certificate,
                (owner.clone(), frame_system::Pallet::<T>::block_number())
            );

			// Emit an event.
			Self::deposit_event(Event::Register(owner, certificate));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn unregister(origin: OriginFor<T>, certificate: Vec<u8>) -> DispatchResult {
			let owner = ensure_signed(origin)?;

            // Check if the claim exists.
            let (old_owner, _) = StarKeeper::<T>::get(&certificate).ok_or(Error::<T>::NotExist)?;

            ensure!(old_owner == owner, Error::<T>::OwnerNeeded);

            // Update storage
            StarKeeper::<T>::remove(&certificate);

            Self::deposit_event(Event::UnRegister(old_owner, certificate));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, receiver: T::AccountId, certificate: Vec<u8>) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			// Can not transfer the claim to self.
            ensure!(owner != receiver, Error::<T>::TransferFailed);

            // Check if the claim exists.
            let (old_owner, _) = StarKeeper::<T>::get(&certificate).ok_or(Error::<T>::NotExist)?;

            ensure!(old_owner == owner, Error::<T>::OwnerNeeded);

            // Update storage
            StarKeeper::<T>::remove(&certificate);
			<StarKeeper<T>>::insert(
                &certificate,
                (receiver.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::Transferred(owner, receiver, certificate));

			Ok(())
		}
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}

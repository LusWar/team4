#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
use frame_support::debug;
use frame_support::{
    ensure, decl_module, decl_storage, decl_event, decl_error,
    dispatch, traits::{Get, ExistenceRequirement},
    storage::IterableStorageDoubleMap,
};

use frame_support::weights::{Weight, DispatchClass, FunctionOf, Pays};

use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The minimum length a claim may be.
    type MinLength: Get<usize>;
    /// The maximum length a claim may be.
    type MaxLength: Get<usize>;
    type MaxMemoLength: Get<usize>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Poe {
		Proofs get(fn get_proof): map hasher(blake2_128_concat) Vec<u8> => Option<(T::AccountId, T::BlockNumber, Vec<u8>)>;
		AccountProofs get(fn get_account_proofs): map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
	{
		ClaimCreated(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ProofNotExist,
		NotClaimOwner,
		/// A claim is too short.
		TooShort,
		/// A claim is too long.
		TooLong,
        MemoTooLong,
		PriceTooLow,
		ClaimNotForSale,
		CannotBuyYourOwnClaim,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// The minimum length a claim may be.
		const MinLength: u32 = T::MinLength::get() as u32;

		/// The maximum length a claim may be.
		const MaxLength: u32 = T::MaxLength::get() as u32;
		const MaxMemoLength: u32 = T::MaxMemoLength::get() as u32;

		#[weight = 10_000]
		pub fn create_claim(origin, claim: Vec<u8>, memo: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(claim.len() >= T::MinLength::get(), Error::<T>::TooShort);
			ensure!(claim.len() <= T::MaxLength::get(), Error::<T>::TooLong);
            ensure!(memo.len() <= T::MaxMemoLength::get(), Error::<T>::MemoTooLong);

			let o = Self::get_proof(&claim);
			ensure!(None == o, Error::<T>::ProofAlreadyExist);

            AccountProofs::<T>::append(&sender, &claim);
			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number(), memo));
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			Ok(())
		}
	}
}

impl<T> Module<T> where T: Trait {
    pub(crate) fn must_get_with_owner(sender: &T::AccountId, claim: &Vec<u8>) -> Result<(T::AccountId, T::BlockNumber, Vec<u8>), dispatch::DispatchError> {
        let o = Self::get_proof(&claim);
        ensure!(None != o, Error::<T>::ProofNotExist);
        let (acc, bn, memo) = o.expect("must be a Some ;qed");
        ensure!(&acc == sender, Error::<T>::NotClaimOwner);
        Ok((acc, bn, memo))
    }
}


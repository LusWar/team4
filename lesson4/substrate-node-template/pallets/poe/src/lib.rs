#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Get,Currency,ExistenceRequirement},
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;
use pallet_timestamp as timestamp;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
/// The pallet's configuration trait.
pub trait Trait: system::Trait + timestamp::Trait{
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type Currency:Currency<Self::AccountId>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as PoeModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> =>
			(T::AccountId, T::BlockNumber, Option<Vec<u8>>, T::Moment);
		Prices get(fn price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;

		Notes get(fn note): map hasher(identity) T::AccountId =>
			Vec<(Vec<u8>, T::BlockNumber, Option<Vec<u8>>, T::Moment)>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId,Balance = BalanceOf<T> {
		ClaimCreated(AccountId, Vec<u8>),

		ClaimRevoked(AccountId, Vec<u8>),
		ClaimTransfered(AccountId, Vec<u8>),
		PriceSet(AccountId, Vec<u8>,Balance),
		ClaimBought(AccountId, Vec<u8>,Balance),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		BuyOwnClaim,
		PriceTooLow,
		NoteTooLong,
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

		// #[weight = 0]
		// pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
		// 	let sender = ensure_signed(origin)?;
		// 	ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
		// 	// 附加题答案
		// 	ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
		// 	Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));
		//
		// 	let price: BalanceOf<T> = 0.into();
		// 	Prices::<T>::insert(&claim, &price);
		//
		// 	Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
		//
		// 	Ok(())
		// }

		#[weight = 0]
		pub fn create_claim_with_note(origin, claim: Vec<u8>, note: Option<Vec<u8>>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);
			// ensure!(T::MaxClaimLength::get() >= note.len() as u32, Error::<T>::NoteTooLong);


			let current_time = <timestamp::Module<T>>::get();
			let block_number = system::Module::<T>::block_number();
			let proof_value = (sender.clone(), &block_number, &note, &current_time);

			//Proofs::<T>::insert(&claim, proof_value);
			// if Notes::<T>::contains_key(sender.clone()) {
			// 	let claims: Vec<(Vec<u8>, T::BlockNumber, Option<Vec<u8>>, T::Moment)> = Notes::<T>::get(sender.clone());
			// 	let mut new_claims = vec![];
			// 	for cla in claims {
			// 		if claim != cla.0 {
			// 			new_claims.push(cla);
			// 		}
			// 	}
			// 	new_claims.push((claim, block_number, note, current_time));
			// 	Notes::<T>::insert(sender.clone(), new_claims);
			// } else {
			// 	Notes::<T>::insert(sender.clone(), vec![(claim, block_number, note, current_time)]);
			// }

			Proofs::<T>::insert(claim.clone(), proof_value);
			Notes::<T>::insert(sender.clone(), vec![(claim.clone(), block_number, note, current_time)]);


			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		// #[weight = 0]
		// pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
		// 	let sender = ensure_signed(origin)?;
		//
		// 	ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
		//
		// 	let (owner, _block_number) = Proofs::<T>::get(&claim);
		//
		// 	ensure!(owner == sender, Error::<T>::NotClaimOwner);
		//
		// 	Proofs::<T>::remove(&claim);
		//
		// 	Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
		//
		// 	Ok(())
		// }

		// 第二题答案
		// #[weight = 0]
		// pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
		// 	let sender = ensure_signed(origin)?;
		// 	ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
		//
		// 	let (owner, _block_number) = Proofs::<T>::get(&claim);
		// 	ensure!(owner == sender, Error::<T>::NotClaimOwner);
		//
		// 	let dest = T::Lookup::lookup(dest)?;
		// 	Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));
		//
		// 	Self::deposit_event(RawEvent::ClaimRevoked(sender,claim));
		// 	Ok(())
		// }

		// #[weight =0]
		// pub fn set_price(origin, claim:Vec<u8>, price:BalanceOf<T> )-> dispatch::DispatchResult{
		// let sender = ensure_signed(origin)?;
		// 	ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
		//
		// 	let (owner, _block_number) = Proofs::<T>::get(&claim);
		// 	ensure!(owner == sender, Error::<T>::NotClaimOwner);
		//
		// 	Prices::<T>::insert(&claim, &price);
		//
		// 	Self::deposit_event(RawEvent::PriceSet(sender,claim,price));
		// 	Ok(())
		// }
		//
		// #[weight=0]
		// pub fn buy_claim(origin,claim:Vec<u8>,bid_price:BalanceOf<T>)->dispatch::DispatchResult{
		// let sender = ensure_signed(origin)?;
		// 	ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
		//
		// 	let (owner, _block_number) = Proofs::<T>::get(&claim);
		// 	ensure!(owner != sender, Error::<T>::BuyOwnClaim);
		//
		// 	let price= Prices::<T>::get(&claim);
		// 	ensure!(bid_price > price, Error::<T>::PriceTooLow);
		//
		// 	T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::AllowDeath)?;
		//
		// 	Proofs::<T>::insert(&claim,(&sender, system::Module::<T>::block_number()));
		// 	Prices::<T>::insert(&claim, &bid_price);
		//
		// 	Self::deposit_event(RawEvent::ClaimBought(sender, claim, price));
		// 	Ok(())
		// }
	}
}

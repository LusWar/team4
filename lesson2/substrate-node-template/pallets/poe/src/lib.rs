#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, StorageMap};
use frame_system::{self as system, ensure_signed}; 
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as PoeModule {
		//Proofs: get(fn proofs) map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlcokNumber);		
		Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event! {
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
			/// Event emitted when a proof has been claimed.
			ClaimCreated(AccountId, Vec<u8>),
			/// Event emitted when a claim is revoked by the owner.
			ClaimRevoked(AccountId, Vec<u8>),
	}
}

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
			/// This proof has already been claimed
			ProofAlreadyClaimed,
			/// The proof does not exist, so it cannot be revoked
			NoSuchProof,
			/// The proof is claimed by another account, so caller can't revoke it
			NotProofOwner,
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

			// A default function for depositing events
			fn deposit_event() = default;

			/// Allow a user to claim ownership of an unclaimed proof
			#[weight = 10_000]
			fn create_claim(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
					// Verify that the incoming transaction is signed and store who the
					// caller of this function is.
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has not been claimed yet or error with the message
					ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

					// Call the `system` pallet to get the current block number
					let current_block = <system::Module<T>>::block_number();

					// Store the proof with the sender and the current block number
					Proofs::<T>::insert(&proof, (&sender, current_block));

					// Emit an event that the claim was created
					Self::deposit_event(RawEvent::ClaimCreated(sender, proof));

					Ok(())
			}

			/// Allow the owner to revoke their claim
			#[weight = 10_000]
			fn revoke_claim(origin, proof: Vec<u8>) -> dispatch::DispatchResult {
					// Determine who is calling the function
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has been claimed
					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					// Get owner of the claim
					let (owner, _) = Proofs::<T>::get(&proof);

					// Verify that sender of the current call is the claim owner
					ensure!(sender == owner, Error::<T>::NotProofOwner);

					// Remove claim from storage
					Proofs::<T>::remove(&proof);

					// Emit an event that the claim was erased
					Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));

					Ok(())
			}

			/// Allow the owner to transfer their claim
			#[weight = 10_000]
			fn transfer_claim(origin, target: T::AccountId, proof: Vec<u8>) -> dispatch::DispatchResult {
					// Determine who is calling the function
					let sender = ensure_signed(origin)?;

					// Verify that the specified proof has been claimed
					ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

					// Get owner of the claim
					let (owner, _) = Proofs::<T>::get(&proof);

					// Verify that sender of the current call is the claim owner
					ensure!(sender == owner, Error::<T>::NotProofOwner);

					// Call the `system` pallet to get the current block number
					let current_block = <system::Module<T>>::block_number();

					// Store the proof with the sender and the current block number
					Proofs::<T>::insert(&proof, (&target, current_block));

					// Emit an event that the claim was created
					Self::deposit_event(RawEvent::ClaimCreated(target, proof));

					Ok(())
			}
	}
}
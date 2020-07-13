#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use core::convert::TryInto;

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{
	self as system, ensure_signed,
	offchain::{Signer, AppCrypto, CreateSignedTransaction, SendSignedTransaction}
};

use sp_std::prelude::*;
use sp_core::crypto::KeyTypeId;

use sp_runtime::{offchain::storage::StorageValueRef};

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use crate::KEY_TYPE;

	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

	type Call: From<Call<Self>>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {

		Numbers get(fn numbers): map hasher(blake2_128_concat) u64 => u64;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {

		NumberAppended(AccountId, u64, u64),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
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

		#[weight = 10_000]
		pub fn save_number(origin, number: u64) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			let block_number = <system::Module<T>>::block_number();
			let index: u64 = block_number.try_into().ok().unwrap() as u64 - 1;
			debug::info!("Numbers of {} is {}", index, number);

			Numbers::insert(index, number);

			Self::deposit_event(RawEvent::NumberAppended(who, index, number));

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			let numbers = StorageValueRef::persistent(b"offchain-numbers::numbers");
			let index = block_number.try_into().ok().unwrap() as u64;
			let num_val: u64 = index.saturating_pow(2);

			let latest = if let Some(Some(val)) = numbers.get::<u64>() {
				val.saturating_add(num_val)
			} else {
				num_val
			};

			numbers.set(&latest);

			Self::submit_number(latest);
		}

	}
}

impl <T: Trait>  Module<T> {
	fn submit_number(number: u64) {


		let signer = Signer::<T, T::AuthorityId>::all_accounts();

		if !signer.can_sign() {
			debug::error!("Not found local account, can't submit number to chain");

			return;
		}

		let res = signer.send_signed_transaction(|_acc| {
			Call::save_number(number)
		});

		for (_acc, r) in &res {
			match r {
				Ok(()) => {
					debug::native::info!("off-chain transaction number: {}  successfully. ", number);
				}
				Err(_) => {
					debug::error!("off-chain transaction number: {} failed. ", number);
				}
			}
		}
	}
}
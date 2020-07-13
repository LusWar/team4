#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer,
	},
};
use core::{convert::TryInto};
use sp_runtime::{offchain::storage::StorageValueRef};
use sp_core::crypto::KeyTypeId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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


/// This is the pallet's configuration trait
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>>  {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Sum get(fn sum): map hasher(blake2_128_concat) u64 => u128;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		ResultStored(u64, u128, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
		SignedSaveNumberError,
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
		pub fn save_number(origin, number: u128) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			let current_block = <system::Module<T>>::block_number();
			let index: u64 = current_block.try_into().ok().unwrap() as u64 - 1;
			debug::info!("Sum({}) = {}", index, number);
			Sum::insert(index, number); // start from the 3rd block
			Self::deposit_event(RawEvent::ResultStored(index, number, who));

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

			/*******
			 * 学员们在这里追加逻辑
			 *******/
			let sum = StorageValueRef::persistent(b"offchain-worker::sum");
			let block = TryInto::<u64>::try_into(block_number).ok().unwrap();
			let block_square = block.saturating_pow(2);
			let new_value;
			if let Some(Some(value)) = sum.get::<u128>() {
				debug::info!("Got value {}", value);
				new_value = value.saturating_add(block_square as u128);
				sum.set(&new_value);
			} else {
				new_value = block_square as u128;
				sum.set(&new_value);
			}
			let result = Self::signed_save_number(new_value);
			if let Err(e) = result { debug::error!("Error: {:?}", e); }
		}

	}
}

impl<T: Trait> Module<T> {
	fn signed_save_number(number: u128) -> Result<(), Error<T>> {
		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			debug::error!("No local account available");
			return Err(<Error<T>>::SignedSaveNumberError);
		}

		let results = signer.send_signed_transaction(|_acct| {
			Call::save_number(number)
		});

		for (acc, res) in &results {
			match res {
				Ok(()) => {
					debug::native::info!(
						"off-chain send_signed: acc: {:?}| number: {}",
						acc.id,
						number
					);
				}
				Err(e) => {
					debug::error!("[{:?}] Failed in signed_submit_number: {:?}", acc.id, e);
					return Err(<Error<T>>::SignedSaveNumberError);
				}
			};
		}
		Ok(())
	}
}
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_error, ensure, StorageValue, StorageMap, traits:: {Randomness, Currency, ExistenceRequirement,}};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchError, DispatchResult};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
	type Currency: Currency<Self::AccountId>;
}

type AccountIdOf<T> = <T as frame_system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) u32 => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(fn kitties_count): u32;

		/// Get kitty IDs by account ID
		pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat) T::AccountId => Vec<u32>;
		/// Get Owner by kitty ID
		pub KittyOwner get(fn kitty_owner): map hasher(blake2_128_concat) u32 => T::AccountId;
		/// Get price by kitty ID
		pub Prices get(fn prices): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		KittyNotOwned,
		KittyAlreadyOwned,
		KittyNotForSale,
		BidPriceTooLow
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;

			// Generate a random 128bit value
			let dna = Self::random_value(&sender);

			// Create and store kitty
			let kitty = Kitty(dna);

			// 作业：补完剩下的部分
			Self::insert_kitty(sender, kitty_id, kitty);
		}

		/// Breed kitties
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: u32, kitty_id_2: u32) {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}

		#[weight = 0]
		pub fn transfer(origin, kitty_id: u32, dest: <T::Lookup as StaticLookup>::Source) {
			let sender = ensure_signed(origin)?;
			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::InvalidKittyId);
			ensure!(sender == KittyOwner::<T>::get(&kitty_id), Error::<T>::KittyNotOwned);
			Self::remove_kitty_from_owner(sender, kitty_id);
			let dest = T::Lookup::lookup(dest)?;
			Self::add_kitty_to_owner(dest, kitty_id);
		}

		#[weight = 0]
		pub fn sell_kitty(origin, kitty_id: u32, ask_price: BalanceOf<T>) {
			let sender = ensure_signed(origin)?;
			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::InvalidKittyId);
			ensure!(sender == KittyOwner::<T>::get(&kitty_id), Error::<T>::KittyNotOwned);
			Prices::<T>::insert(kitty_id, ask_price);
		}

		#[weight = 0]
		pub fn buy_kitty(origin, kitty_id: u32, bid_price: BalanceOf<T>) {
			let sender = ensure_signed(origin)?;
			ensure!(Kitties::contains_key(&kitty_id), Error::<T>::InvalidKittyId);
			let owner = KittyOwner::<T>::get(&kitty_id);
			ensure!(sender != owner, Error::<T>::KittyAlreadyOwned);
			ensure!(Prices::<T>::contains_key(&kitty_id), Error::<T>::KittyNotForSale);
			let ask_price = Prices::<T>::get(&kitty_id);
			ensure!(bid_price >= ask_price, Error::<T>::BidPriceTooLow);
			T::Currency::transfer(&sender, &owner, ask_price, ExistenceRequirement::KeepAlive)?;
			Self::remove_kitty_from_owner(owner, kitty_id.clone());
			Self::add_kitty_to_owner(sender, kitty_id.clone());
			Prices::<T>::remove(kitty_id);
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		// 作业：完成方法
		let payload = (
			<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			sender,
			<frame_system::Module<T>>::extrinsic_index()
		);
		return payload.using_encoded(blake2_128);
	}

	fn next_kitty_id() -> sp_std::result::Result<u32, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == u32::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: u32, kitty: Kitty) {
		// 作业：完成方法
		Kitties::insert(kitty_id, kitty);
		KittiesCount::put(kitty_id + 1);

		Self::add_kitty_to_owner(owner, kitty_id);
	}

	fn add_kitty_to_owner(owner: T::AccountId, kitty_id: u32) {
		if <OwnedKitties<T>>::contains_key(&owner) {
			let mut kitty_ids = <OwnedKitties<T>>::get(&owner);
			match kitty_ids.binary_search(&kitty_id) {
				Ok(_) => (),
				Err(index) => kitty_ids.insert(index, kitty_id.clone()),
			}
		} else {
			let mut kitty_ids = Vec::<u32>::new();
			kitty_ids.push(kitty_id);
			<OwnedKitties<T>>::insert(&owner, kitty_ids);
		}
		<KittyOwner<T>>::insert(kitty_id, owner);
	}

	fn remove_kitty_from_owner(owner: T::AccountId, kitty_id: u32) {
		let mut kitty_ids = <OwnedKitties<T>>::get(&owner);
		match kitty_ids.binary_search(&kitty_id) {
			Ok(index) => kitty_ids.remove(index),
			Err(_) => 0,
		};
		OwnedKitties::<T>::insert(&owner, kitty_ids);
		<KittyOwner<T>>::remove(kitty_id);
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: u32, kitty_id_2: u32) -> DispatchResult {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}
}

#![cfg_attr(not(feature = "std"),no_std)]


use codec::{Encode,Decode};
use frame_support::{ decl_module, decl_storage, decl_error, StorageValue, StorageMap, ensure, traits::Randomness, Parameter };
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchResult, DispatchError, traits::{AtLeast32Bit, Bounded} };


#[derive(Encode,Decode)]
pub struct Kitty(pub [u8; 16]) ;

pub trait Trait: frame_system::Trait {
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
}


decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;

		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		pub OwnerKitties get(fn owner_kitties): map hasher(blake2_128_concat) (T::AccountId, T::KittyIndex) => T::KittyIndex;

		pub OwnerKittiesCount get(fn owner_kitties_count): map hasher(blake2_128_concat) T::AccountId => T::KittyIndex;
	}
}


decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
	}

}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{

		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)? ;


			let kitty_id = Self::next_kitty_id()?;

			let dna = Self::random_value(&sender);


			let kitty = Kitty(dna);

			//作业

			Self::insert_kitty(sender, kitty_id, kitty);

		}


		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)? ;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;
		}


	}
}


fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}


impl<T: Trait> Module<T> {



	fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex,DispatchError> {

		let kitty_id = Self::kitties_count();

		ensure!(kitty_id <= T::KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);

		Ok(kitty_id)

	}


	fn random_value(sender: &T::AccountId) -> [u8; 16]{

		//作业

		let payload = (
			<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index()
			);

		payload.using_encoded(blake2_128)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {

		//作业

		Kitties::<T>::insert(kitty_id,kitty);
		KittiesCount::<T>::put(kitty_id+1.into());


		let owner_id = Self::owner_kitties_count(&owner);

		<OwnerKitties<T>>::insert((&owner,owner_id),kitty_id);
		<OwnerKittiesCount<T>>::insert(&owner,owner_id +1.into());

	}

	
	fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) ->DispatchResult {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		let kitty_id = Self::next_kitty_id()?;

		let selector = Self::random_value(&sender);

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;

		let mut new_dna = [0u8; 16];

		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		let kitty = Kitty(new_dna);

		Self::insert_kitty(sender, kitty_id, kitty);

		Ok(())

	}


}
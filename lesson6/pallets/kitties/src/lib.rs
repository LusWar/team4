#![cfg_attr(not(feature = "std"),no_std)]


use codec::{Encode,Decode};
use frame_support::{ decl_module, decl_storage, decl_error, StorageValue, StorageMap, ensure, traits::Randomness, Parameter };
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::{DispatchResult, DispatchError, traits::{AtLeast32Bit, Bounded, Member} };


#[derive(Encode,Decode)]
pub struct Kitty(pub [u8; 16]) ;

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode,Decode)]
pub struct KittyLinkedItem<T: Trait> {
	pub prev: Option<T::KittyIndex>,
	pub next: Option<T::KittyIndex>,
}



pub trait Trait: frame_system::Trait {
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Member + Default + Copy;
}


decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;

		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		pub OwnerKitties get(fn owner_kitties): map hasher(blake2_128_concat) (T::AccountId,  Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;

		// pub OwnerKitties get(fn owner_kitties): map hasher(blake2_128_concat) (T::AccountId, T::KittyIndex) => T::KittyIndex;

		// pub OwnerKittiesCount get(fn owner_kitties_count): map hasher(blake2_128_concat) T::AccountId => T::KittyIndex;
	}
}


decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
	}

}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{

		type Error = Error<T>;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)? ;


			let kitty_id = Self::next_kitty_id()?;

			let dna = Self::random_value(&sender);


			let kitty = Kitty(dna);

			Self::insert_kitty(&sender, kitty_id, kitty);

		}


		/// Breed kitties
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)? ;

			Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
		}

		/// Transfer a kitty to new owner
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			// 作业
		}


	}
}




fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	(selector & dna1) | (!selector & dna2)
}


impl<T: Trait> OwnerKitties<T> {
	fn read_head(account: &T::AccountId) -> KittyLinkedItem<T> {
		Self::read(&account,None)
	}

	fn write_head(account: &T::AccountId, item: KittyLinkedItem<T>){
		Self::write(&account, None, item);
	}


	fn read(account: &T::AccountId, key: Option<T::KittyIndex>) -> KittyLinkedItem<T> {
		<OwnerKitties<T>>::get((&account, key)).unwrap_or_else(|| KittyLinkedItem{
			prev: None,
			next: None,
		})
		
	}


	fn write(account: &T::AccountId, key: Option<T::KittyIndex>, item: KittyLinkedItem<T>) {
		<OwnerKitties<T>>::insert((&account,key),item);
	}


	pub fn append(account: &T::AccountId, kitty_id: T::KittyIndex) {
		let head = Self::read_head(account);
		let new_head = KittyLinkedItem{
			prev: Some(kitty_id),
			next: head.next,
		};

		Self::write_head(account, new_head);

		let last = Self::read(account, head.prev);
		let new_last = KittyLinkedItem{
			prev: last.prev,
			next: Some(kitty_id),
		};

		Self::write(account,head.prev,new_last);

		let item = KittyLinkedItem {
			prev:head.prev,
			next: None,
		};

		Self::write(account,Some(kitty_id),item);
		
	}

	pub fn remove(account: &T::AccountId, kitty_id: T::KittyIndex) {
		if let Some(item) = <OwnerKitties<T>>::take((&account, Some(kitty_id))) {
			let prev = Self::read(account,item.prev);
			let new_prev = KittyLinkedItem{
				prev: prev.prev,
				next: item.next,
			};

			Self::write(account, item.prev, new_prev);

			let next = Self::read(account,item.next);
			let new_next = KittyLinkedItem{
				prev:item.prev,
				next:next.next,
			};

			Self::write(account, item.next, new_next);

		}


	}

}



impl<T: Trait> Module<T> {



	fn next_kitty_id() -> sp_std::result::Result<T::KittyIndex,DispatchError> {

		let kitty_id = Self::kitties_count();

		ensure!(kitty_id <= T::KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);

		Ok(kitty_id)

	}


	fn random_value(sender: &T::AccountId) -> [u8; 16]{

		let payload = (
			<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index()
			);

		payload.using_encoded(blake2_128)
	}


	fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex){

		//作业

		<OwnerKitties<T>>::append(owner,kitty_id);

	}


	fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {

		Kitties::<T>::insert(kitty_id,kitty);
		KittiesCount::<T>::put(kitty_id+1.into());

		Self::insert_owned_kitty(owner, kitty_id);

	}

	
	fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) ->DispatchResult {
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

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

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use sp_core::H256;
	use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};
	use frame_system as system;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type DbWeight = ();
		type BlockExecutionWeight = ();
		type ExtrinsicBaseWeight = ();
		type MaximumExtrinsicWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
		type AccountData = ();
		type OnNewAccount = ();
		type OnKilledAccount = ();
	}
	impl Trait for Test {
		type KittyIndex = u32;
	}
	type OwnedKittiesTest = OwnerKitties<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn owned_kitties_can_append_values() {
		new_test_ext().execute_with(|| {
			OwnedKittiesTest::append(&0, 1);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: None,
			}));


			OwnedKittiesTest::append(&0, 2);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(2),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
				prev: Some(1),
				next: None,
			}));

			OwnedKittiesTest::append(&0, 3);

			assert_eq!(OwnedKittiesTest::get(&(0, None)), Some(KittyLinkedItem {
				prev: Some(3),
				next: Some(1),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(1))), Some(KittyLinkedItem {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(3),
			}));

			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), Some(KittyLinkedItem {
				prev: Some(2),
				next: None,
			}));
		});
	}

	#[test]
	fn owned_kitties_can_remove_values() {
		// 作业

		new_test_ext().execute_with(|| {
			OwnedKittiesTest::append(&0, 1);
			OwnedKittiesTest::append(&0, 2);
			OwnedKittiesTest::append(&0, 3);
			OwnedKittiesTest::append(&0, 4);
			OwnedKittiesTest::append(&0, 5);


			OwnedKittiesTest::remove(&0, 3);
			
			assert_eq!(OwnedKittiesTest::get(&(0, Some(3))), None);

			assert_eq!(OwnedKittiesTest::get(&(0, Some(2))), Some(KittyLinkedItem {
				prev: Some(1),
				next: Some(4),
			}));
			
			assert_eq!(OwnedKittiesTest::get(&(0, Some(4))), Some(KittyLinkedItem {
				prev: Some(2),
				next: Some(5),
			}));
			
		});
	}
}
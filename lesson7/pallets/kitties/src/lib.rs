#![cfg_attr(not(feature = "std"),no_std)]


use codec::{Encode,Decode};
use frame_support::{ decl_module, decl_storage, decl_error, decl_event, StorageValue, StorageMap, ensure, Parameter, traits::{Randomness, Currency, ExistenceRequirement} };
use sp_io::hashing::blake2_128;
use frame_system::{self as system,ensure_signed};
use sp_runtime::{DispatchError, traits::{AtLeast32Bit, Bounded, Member} };
use crate::linked_item::{LinkedList, LinkedItem};

mod linked_item;


#[derive(Encode,Decode)]
pub struct Kitty(pub [u8; 16]) ;


pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type KittyIndex: Parameter + AtLeast32Bit + Bounded + Member + Default + Copy;
	type Currency: Currency<Self::AccountId>;
	type Randomness: Randomness<Self::Hash>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type KittyLinkedItem<T> = LinkedItem<<T as Trait>::KittyIndex>;
type OwnerKittiesList<T> = LinkedList<OwnerKitties<T>, <T as system::Trait>::AccountId, <T as Trait>::KittyIndex>;





decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		pub Kitties get(fn kitties): map hasher(blake2_128_concat) T::KittyIndex => Option<Kitty>;

		pub KittiesCount get(fn kitties_count): T::KittyIndex;

		pub OwnerKitties get(fn owner_kitties): map hasher(blake2_128_concat) (T::AccountId,  Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;

		pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) T::KittyIndex => Option<T::AccountId>;

		pub KittyPrices get(fn kitty_price): map hasher(blake2_128_concat) T::KittyIndex => Option<BalanceOf<T>>;


	}
}


decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
		RequireOwner,
		KittyIndexNotExist,
		NotForSale,
		PriceTooLow,
	}

}


decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::KittyIndex,
		Balance = BalanceOf<T>,
	{
		/// A kitty is created. (owner, kitty_id)
		Created(AccountId, KittyIndex),
		/// A kitty is transferred. (from, to, kitty_id)
		Transferred(AccountId, AccountId, KittyIndex),
		/// A kitty is available for sale. (owner, kitty_id, price)
		Ask(AccountId, KittyIndex, Option<Balance>),
		/// A kitty is sold. (from, to, kitty_id, price)
		Sold(AccountId, AccountId, KittyIndex, Balance),
	}
);


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{

		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)? ;


			let kitty_id = Self::next_kitty_id()?;

			let dna = Self::random_value(&sender);


			let kitty = Kitty(dna);

			Self::insert_kitty(&sender, kitty_id, kitty);

			Self::deposit_event(RawEvent::Created(sender, kitty_id));

		}


		/// Breed kitties
		#[weight = 0]
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) {
			let sender = ensure_signed(origin)? ;

			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;

			Self::deposit_event(RawEvent::Created(sender, new_kitty_id));

		}

		/// Transfer a kitty to new owner
		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: T::KittyIndex) {
			// 作业

			//确认用户签名
			let owner = ensure_signed(origin)?;

			//检查存证是否存在
			ensure!(Kitties::<T>::contains_key(&kitty_id),Error::<T>::KittyIndexNotExist );

			//检查AccountId下是否有相应的KittyId
			ensure!(OwnerKitties::<T>::contains_key((&owner, Some(kitty_id))),Error::<T>::RequireOwner );

			Self::do_transfer(&owner, &to, kitty_id);

			Self::deposit_event(RawEvent::Transferred(owner, to, kitty_id));

		}


		#[weight = 0]
		pub fn ask(origin, kitty_id: T::KittyIndex, price: Option<BalanceOf<T>>) {

			let owner = ensure_signed(origin)?;

			//检查存证是否存在
			ensure!(Kitties::<T>::contains_key(&kitty_id),Error::<T>::KittyIndexNotExist );

			//检查AccountId下是否有相应的KittyId
			ensure!(OwnerKitties::<T>::contains_key((&owner, Some(kitty_id))),Error::<T>::RequireOwner );

			KittyPrices::<T>::mutate_exists(kitty_id, |old_price|  *old_price = price);

			Self::deposit_event(RawEvent::Ask(owner, kitty_id, price));

		}


		#[weight = 0]
		pub fn buy(origin, kitty_id: T::KittyIndex, price: BalanceOf<T>) {

			let sender = ensure_signed(origin)?;

			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_price = Self::kitty_price(kitty_id).ok_or(Error::<T>::NotForSale)?;


			ensure!(kitty_price <= price,Error::<T>::PriceTooLow);

			//转账
			T::Currency::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive)?;

			KittyPrices::<T>::mutate_exists(kitty_id, |old_price|  *old_price = Some(price));

			Self::do_transfer(&owner, &sender, kitty_id);

			Self::deposit_event(RawEvent::Sold(owner, sender, kitty_id, kitty_price));

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

		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>>::extrinsic_index()
			);

		payload.using_encoded(blake2_128)
	}


	fn insert_owned_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex){

		//作业

		<OwnerKittiesList<T>>::append(owner,kitty_id);

		KittyOwners::<T>::insert(kitty_id, &owner);

	}


	fn insert_kitty(owner: &T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {

		Kitties::<T>::insert(kitty_id,kitty);
		KittiesCount::<T>::put(kitty_id+1.into());

		Self::insert_owned_kitty(owner, kitty_id);

	}

	fn do_transfer(owner: &T::AccountId, to: &T::AccountId, kitty_id: T::KittyIndex) {
		OwnerKittiesList::<T>::remove(owner,kitty_id);
		Self::insert_owned_kitty(to,kitty_id);
	}

	
	fn do_breed(sender: &T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> sp_std::result::Result<T::KittyIndex, DispatchError> {
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

		Ok(kitty_id)

	}


}


/// tests for this module
#[cfg(test)]
mod tests {

	use super::*;


	use crate::{Module, Trait};
	use sp_io::TestExternalities;
	use sp_core::H256;
	use frame_support::{impl_outer_origin, parameter_types, weights::Weight};


	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};
	use frame_system as system;

	

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the pallet, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of pallets we want to use.
	#[derive(Clone, Eq, PartialEq)]
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
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}


	impl pallet_balances::Trait for Test {
		type Balance = u64;
		type DustRemoval = ();
		type Event = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
	}


	impl Trait for Test {
		type Event = ();
		type Currency = Balances;
	 	type KittyIndex = u32;
  		type Randomness = Randomness;
 	}

 	pub type KittyModule = Module<Test>;

	pub type System = system::Module<Test>;
	pub type Balances = pallet_balances::Module<Test>;
	pub type Randomness = pallet_randomness_collective_flip::Module<Test>;


	type OwnerKittiesTest = OwnerKitties<Test>;

	type KittyLinkedItemTest = LinkedItem<u32>;

	type OwnerKittiesListTest = LinkedList<OwnerKittiesTest, u64, u32>;


	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {

		let mut storage = system::GenesisConfig::default()
				.build_storage::<Test>()
				.unwrap();

			pallet_balances::GenesisConfig::<Test> {
				balances: vec![(1, 10000), (2, 10000), (3, 10000), (4, 10000)],
			}
			.assimilate_storage(&mut storage).unwrap();

			let mut ext = TestExternalities::from(storage);
			ext.execute_with(|| System::set_block_number(1));
			ext.into()
	}

		#[test]
	fn owned_kitties_can_append_values() {
		new_test_ext().execute_with(|| {
			OwnerKittiesListTest::append(&1, 1);

			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: Some(1),
				next: Some(1),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(1))), Some(KittyLinkedItemTest {
				prev: None,
				next: None,
			}));


			OwnerKittiesListTest::append(&1, 2);

			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: Some(2),
				next: Some(1),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(1))), Some(KittyLinkedItemTest {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(2))), Some(KittyLinkedItemTest {
				prev: Some(1),
				next: None,
			}));

			OwnerKittiesListTest::append(&1, 3);

			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: Some(3),
				next: Some(1),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(1))), Some(KittyLinkedItemTest {
				prev: None,
				next: Some(2),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(2))), Some(KittyLinkedItemTest {
				prev: Some(1),
				next: Some(3),
			}));

			assert_eq!(OwnerKittiesTest::get(&(1, Some(3))), Some(KittyLinkedItemTest {
				prev: Some(2),
				next: None,
			}));
		});
	}

	#[test]
	fn owned_kitties_can_remove_values() {
		// 作业

		new_test_ext().execute_with(|| {
			OwnerKittiesListTest::append(&1, 1);
			OwnerKittiesListTest::append(&1, 2);
			OwnerKittiesListTest::append(&1, 3);
			OwnerKittiesListTest::append(&1, 4);
			OwnerKittiesListTest::append(&1, 5);


			OwnerKittiesListTest::remove(&1, 3);
			
			assert_eq!(OwnerKittiesTest::get(&(1, Some(3))), None);

			assert_eq!(OwnerKittiesTest::get(&(1, Some(2))), Some(KittyLinkedItemTest {
				prev: Some(1),
				next: Some(4),
			}));
			
			assert_eq!(OwnerKittiesTest::get(&(1, Some(4))), Some(KittyLinkedItemTest {
				prev: Some(2),
				next: Some(5),
			}));
			
		});
	}


	#[test]
	fn create_kitty_works(){
		new_test_ext().execute_with(||{

			// 生成Kitty
			let mut _result = KittyModule::create(Origin::signed(1));

			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),1);

			// 账户1的OwnerKitties Head
			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: Some(0),
				next: Some(0),
			}));

			// 判断0＃ Kitty拥有者
			assert_eq!(KittyOwners::<Test>::get(0),Some(1));

			// 生成Kitty
			_result = KittyModule::create(Origin::signed(2));


			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),2);


			// 账户2的OwnerKitties Head
			assert_eq!(OwnerKittiesTest::get(&(2, None)), Some(KittyLinkedItemTest {
				prev: Some(1),
				next: Some(1),
			}));


			// 判断1＃ Kitty拥有者
			assert_eq!(KittyOwners::<Test>::get(1),Some(2));

		});
	}


	#[test]
	fn breed_kitty_works(){
		new_test_ext().execute_with(||{

			// 生成3只Kitty
			let mut _result = KittyModule::create(Origin::signed(1));
			_result = KittyModule::create(Origin::signed(1));
			_result = KittyModule::create(Origin::signed(1));

			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),3);


			// 合成Kitty
			_result = KittyModule::breed(Origin::signed(1), 0, 1);


			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),4);


			// 账户1的OwnerKitties链表
			assert_eq!(OwnerKittiesTest::get(&(1, Some(3))), Some(KittyLinkedItemTest {
				prev: Some(2),
				next: None,
			}));

			// 判断合成Kitty拥有者
			assert_eq!(KittyOwners::<Test>::get(3),Some(1));

		});
	}

	#[test]
	fn transfer_kitty_works(){
		new_test_ext().execute_with(||{

			// 生成1只Kitty
			let mut _result = KittyModule::create(Origin::signed(1));

			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),1);

			// 账户1的OwnerKitties链表
			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: Some(0),
				next: Some(0),
			}));

			// 账户2的OwnerKitties链表
			assert_eq!(OwnerKittiesTest::get(&(2, None)), None);


			// 判断0# Kitty拥有者
			assert_eq!(KittyOwners::<Test>::get(0),Some(1));


			// 账户1转让0＃ Kitty 给账户2
			_result = KittyModule::transfer(Origin::signed(1), 2, 0);


			// 账户1的OwnerKitties链表
			assert_eq!(OwnerKittiesTest::get(&(1, None)), Some(KittyLinkedItemTest {
				prev: None,
				next: None,
			}));

			// 账户2的OwnerKitties链表
			assert_eq!(OwnerKittiesTest::get(&(2, None)), Some(KittyLinkedItemTest {
				prev: Some(0),
				next: Some(0),
			}));


			// 判断0# Kitty拥有者
			assert_eq!(KittyOwners::<Test>::get(0),Some(2));

		});
	}



	#[test]
	fn ask_kitty_price_works(){
		new_test_ext().execute_with(||{

			// 生成1只Kitty
			let mut _result = KittyModule::create(Origin::signed(1));

			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),1);

			// 0＃ Kitty 价格
			assert_eq!(KittyPrices::<Test>::get(0),None);

			// 账户1修改0＃ Kitty 价格
			_result = KittyModule::ask(Origin::signed(1), 0, Some(100));

			// 修改后的0＃ Kitty 价格
			assert_eq!(KittyPrices::<Test>::get(0),Some(100));

		});
	}



	#[test]
	fn buy_kitty_works(){
		new_test_ext().execute_with(||{

			// 生成1只Kitty
			let mut _result = KittyModule::create(Origin::signed(1));

			// Kitty总数量
			assert_eq!(KittiesCount::<Test>::get(),1);

			// 0＃ Kitty 价格
			assert_eq!(KittyPrices::<Test>::get(0),None);

			// 账户1修改0＃ Kitty 价格
			_result = KittyModule::ask(Origin::signed(1), 0, Some(1000));

			// 修改后的0＃ Kitty 价格
			assert_eq!(KittyPrices::<Test>::get(0),Some(1000));

			// 0＃ Kitty 拥有者
			assert_eq!(KittyOwners::<Test>::get(0),Some(1));

			// 账户2花费1000购买账户1的0＃ Kitty
			_result = KittyModule::buy(Origin::signed(2), 0, 1000);

			// 0＃ Kitty 拥有者
			assert_eq!(KittyOwners::<Test>::get(0),Some(2));

			// 账户1当前余额
			assert_eq!(Balances::total_balance(&1), 11000);

			// 账户2当前余额
			assert_eq!(Balances::total_balance(&2), 9000);

		});
	}




}

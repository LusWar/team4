// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok};
use codec::{Decode};

#[test]
fn test_onchain() {
	let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
	t.execute_with(|| {
		let value = 1;
		let acct: <Test as system::Trait>::AccountId = Default::default();

		// when `save_number` is being called
		assert_ok!(TemplateModule::save_number(Origin::signed(acct), value));

		// added to storage
		assert_eq!(<Sum>::get(0), value);

		// an event is emitted
		let expected_event = TestEvent::template(RawEvent::ResultStored(0, value, acct));
		assert!( System::events().iter().any(|er| er.event == expected_event) );

	});
}

#[test]
fn test_offchain() {
	let (mut t, pool_state, _offchain_state) = ExtBuilder::build();

	let acct: <Test as system::Trait>::AccountId = Default::default();

	t.execute_with(|| {
		// 4 submit_number being called
		assert_ok!(TemplateModule::signed_save_number(0));
		assert_ok!(TemplateModule::save_number(Origin::signed(acct), 0));

		assert_ok!(TemplateModule::signed_save_number(1));
		assert_ok!(TemplateModule::save_number(Origin::signed(acct), 1));

		assert_ok!(TemplateModule::signed_save_number(2));
		assert_ok!(TemplateModule::save_number(Origin::signed(acct), 2));

		// check proper calls are being added to the transaction pools
		let tx3 = pool_state.write().transactions.pop().unwrap();
		let tx2 = pool_state.write().transactions.pop().unwrap();
		let tx1 = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());

		let tx1decoded = TestExtrinsic::decode(&mut &*tx1).unwrap();
		assert_eq!(tx1decoded.call, Call::save_number(0));

		let tx2decoded = TestExtrinsic::decode(&mut &*tx2).unwrap();
		assert_eq!(tx2decoded.call, Call::save_number(1));

		let tx3decoded = TestExtrinsic::decode(&mut &*tx3).unwrap();
		assert_eq!(tx3decoded.call, Call::save_number(2));
	});
}
// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok};
use codec::Decode;

#[test]
fn test_onchain() {
	new_test_ext().execute_with(|| {
		// Test onchain logic here
		let (mut t, _, _) = ExtBuilder::build();
		t.execute_with(|| {
			let val = 1;
			let account_id: <Test as system::Trait>::AccountId = Default::default();

			assert_ok!(TemplateModule::save_number(Origin::signed(account_id), val));

			assert_eq!(<Numbers>::get(0), val);

			let expected_event = TestEvent::template(RawEvent::NumberAppended(account_id, 0, val));
			assert!(<system::Module<Test>>::events().iter().any(|er| er.event == expected_event));
		})
	});
}

#[test]
fn test_offchain() {
	new_test_ext().execute_with(|| {
		// Test offchain worker here
		let (mut t, pool_state, _) = ExtBuilder::build();

		let account_id: <Test as system::Trait>::AccountId = Default::default();

		t.execute_with(|| {
			TemplateModule::submit_number(0);
			assert_ok!(TemplateModule::save_number(Origin::signed(account_id), 0));

			TemplateModule::submit_number(1);
			assert_ok!(TemplateModule::save_number(Origin::signed(account_id), 1));

			TemplateModule::submit_number(2);
			assert_ok!(TemplateModule::save_number(Origin::signed(account_id), 2));

			// check proper calls are being added to the transaction pools
			let tx3 = pool_state.write().transactions.pop().unwrap();
			let tx2 = pool_state.write().transactions.pop().unwrap();
			let tx1 = pool_state.write().transactions.pop().unwrap();
			assert!(pool_state.read().transactions.is_empty());

			let tx1_decoded = TestExtrinsic::decode(&mut &*tx1).unwrap();
			assert_eq!(tx1_decoded.call, Call::save_number(0));

			let tx2_decoded = TestExtrinsic::decode(&mut &*tx2).unwrap();
			assert_eq!(tx2_decoded.call, Call::save_number(1));

			let tx3_decoded = TestExtrinsic::decode(&mut &*tx3).unwrap();
			assert_eq!(tx3_decoded.call, Call::save_number(2));
		})
	});
}

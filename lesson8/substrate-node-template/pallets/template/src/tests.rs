// Tests to be written here

use crate::{*, mock::*};
use frame_support::{assert_ok};
use codec::{Decode};

#[test]
fn test_onchain() {
    let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
    t.execute_with(|| {
        let index = 1;
        let value = 2;
        let acct: <Test as system::Trait>::AccountId = Default::default();

        // when `save_number` is being called
        assert_ok!(TemplateModule::save_number(Origin::signed(acct), index , value));

        // added to storage
        assert_eq!(<Numbers>::get(index), value);

        // an event is emitted
        let expected_event = TestEvent::template(RawEvent::NumberAppended(acct, index, value));
        assert!(System::events().iter().any(|er| er.event == expected_event));
    });
}

#[test]
fn test_offchain() {
    let (mut t, pool_state, _offchain_state) = ExtBuilder::build();

    let acct: <Test as system::Trait>::AccountId = Default::default();

    t.execute_with(|| {
        TemplateModule::submit_number(0);
        let tx = pool_state.write().transactions.pop().unwrap();
        let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.call, crate::Call::save_number(0, 1));
        match tx.call {
            crate::Call::save_number(b, r) => {
                assert_ok!(TemplateModule::save_number(Origin::signed(acct), b, r));
            }
            _ => {}
        }

        TemplateModule::submit_number(1);
        let tx = pool_state.write().transactions.pop().unwrap();
        let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.call, crate::Call::save_number(1, 5));
        match tx.call {
            crate::Call::save_number(b, r) => {
                assert_ok!(TemplateModule::save_number(Origin::signed(acct), b, r));
            }
            _ => {}
        }

        TemplateModule::submit_number(2);
        let tx = pool_state.write().transactions.pop().unwrap();
        let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.call, crate::Call::save_number(2, 14));
        match tx.call {
            crate::Call::save_number(b, r) => {
                assert_ok!(TemplateModule::save_number(Origin::signed(acct), b, r));
            }
            _ => {}
        }

        TemplateModule::submit_number(3);
        let tx = pool_state.write().transactions.pop().unwrap();
        let tx = TestExtrinsic::decode(&mut &*tx).unwrap();
        assert_eq!(tx.call, crate::Call::save_number(3, 30));
        match tx.call {
            crate::Call::save_number(b, r) => {
                assert_ok!(TemplateModule::save_number(Origin::signed(acct), b, r));
            }
            _ => {}
        }
    });
}

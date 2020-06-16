use crate::{Error, RawEvent, mock::*};
use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};
use frame_system::RawOrigin;

#[test]
fn create_and_revoke() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(Poe::create_claim(Origin::signed(1), b"claim_1".to_vec(), b"memo_1".to_vec()));
        assert_ok!(Poe::create_claim(Origin::signed(1), b"claim_2".to_vec(), b"memo_2".to_vec()));
        assert_ok!(Poe::create_claim(Origin::signed(1), b"claim_3".to_vec(), b"memo_3".to_vec()));
        let a = Poe::get_account_proofs(1);
        assert_eq!(3, a.len());
    });
}



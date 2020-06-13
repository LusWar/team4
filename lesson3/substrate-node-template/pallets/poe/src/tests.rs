// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_create_claim_succcess_value() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        // asserting that the stored value is equal to what we stored
        assert_eq!(PoeModule::proofs(claim), (1, 0));
    });
}

#[test]
fn it_works_for_create_claim_exist() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // asserting that the stored value is equal to what we stored
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn it_works_for_create_claim_too_long() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4, 5, 6, 7];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    });
}


#[test]
fn it_works_for_revoke_claim() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    });
}


#[test]
fn it_works_for_revoke_claim_not_claim_owner() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    });
}

#[test]
fn it_works_for_transfer_claim() {
    new_test_ext().execute_with(|| {
        // Just a dummy test for the dummy function `do_something`
        // calling the `do_something` function with a value 42
        let claim = vec![1,2,3,4];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(PoeModule::proofs(claim), (2, 0));
    });
}
// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop, StorageMap};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_eq!(Proofs::<Test>::get(hash), (1, system::Module::<Test>::block_number()));
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), hash));
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), hash.clone(), 2));
		assert_eq!(Proofs::<Test>::get(hash), (2, system::Module::<Test>::block_number()));
	});
}

#[test]
fn create_claim_error_proof_too_long() {
    new_test_ext().execute_with(|| {
        let hash: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), hash),
            Error::<Test>::ProofTooLong
        );
    });
}

#[test]
fn create_claim_error_proof_already_exists() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), hash),
			Error::<Test>::ProofAlreadyExist
		);
	});
}

#[test]
fn revoke_claim_error_claim_not_exists() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		let another_hash: Vec<u8> = [4,5,6].to_vec();
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), another_hash),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn revoke_claim_error_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), hash),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_error_claim_not_exists() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = vec![1, 2, 3];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), hash, 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_error_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let hash: Vec<u8> = [0,1,2].to_vec();
		assert_ok!(PoeModule::create_claim(Origin::signed(1), hash.clone()));
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), hash, 3),
			Error::<Test>::NotClaimOwner
		);
	});
}

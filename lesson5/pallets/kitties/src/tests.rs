#![allow(unused_imports)]

use crate::{Error, RawEvent, mock::*, KittiesCount};
use frame_support::{
    assert_ok,
    assert_noop,
    dispatch::DispatchError,
    storage::StorageValue,
};
use frame_system::{RawOrigin};

#[test]
fn create_kitty() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(Kitty::create(Origin::signed(1)));
        assert_eq!(Kitty::kitties_count(), 1);
        assert_eq!(Kitty::owned_kitties_count(1), 1);
        KittiesCount::put(u32::max_value());
        assert_noop!(
			Kitty::create(Origin::signed(1)),
			Error::<Test>::KittiesCountOverflow,
		);
    });
}

#[test]
fn breed_kitty() {
    ExtBuilder::build().execute_with(|| {
        assert_ok!(Kitty::create(Origin::signed(4)));
        assert_ok!(Kitty::create(Origin::signed(1)));
        assert_ok!(Kitty::create(Origin::signed(1)));
        assert_ok!(Kitty::create(Origin::signed(2)));
        assert_ok!(Kitty::create(Origin::signed(2)));
        assert_eq!(Kitty::kitties_count(), 5);
        assert_ok!(Kitty::breed(Origin::signed(1), 1, 2));
        assert_eq!(Kitty::owned_kitties_count(1), 3);
        assert_eq!(Kitty::owned_kitties_count(2), 2);
        assert_noop!(
            Kitty::breed(Origin::signed(3), 1, 6),
            Error::<Test>::InvalidKittyId,
        );
        assert_noop!(
            Kitty::breed(Origin::signed(3), 1, 1),
            Error::<Test>::RequireDifferentParent,
        );
        for i in 0u32..Kitty::kitties_count() {
            assert!(Kitty::kitties(i).is_some());
        }

        for j in 0u64..2 {
            for i in 0u32..Kitty::owned_kitties_count(j) {
                assert!(Kitty::owned_kitties((j, i)) > 0);
            }
        }
    });
}


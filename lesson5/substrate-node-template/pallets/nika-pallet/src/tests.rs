use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_register() {
	new_test_ext().execute_with(|| {
		let cer = vec![6, 6, 6];
		// Dispatch a signed extrinsic.
		assert_ok!(NikaModule::register(Origin::signed(1), cer.clone()));
		// Read pallet storage and assert an expected result.
		assert!(NikaModule::proofs(cer.clone()).is_some());
	});
}

#[test]
fn it_fails_for_register_again() {
	new_test_ext().execute_with(|| {
		let cer = vec![6, 6, 6];
		assert_ok!(NikaModule::register(Origin::signed(1), cer.clone()));
		assert_noop!(NikaModule::register(Origin::signed(1), cer.clone()), Error::<Test>::AlreadyExist);
	});
}

#[test]
fn it_works_for_unregister() {
	new_test_ext().execute_with(|| {
		let cer = vec![6, 6, 6];
		// register first
		assert_ok!(NikaModule::register(Origin::signed(1), cer.clone()));
		// unregister
		assert_ok!(NikaModule::unregister(Origin::signed(1), cer.clone()));
	});
}

#[test]
fn it_fails_for_unregister_when_not_exist() {
	new_test_ext().execute_with(|| {
		let cer = vec![6, 6, 6];
		// unregister
		assert_noop!(NikaModule::unregister(Origin::signed(1), cer.clone()), Error::<Test>::NotExist);
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let cer = vec![6, 6, 6];
		// register first
		assert_ok!(NikaModule::register(Origin::signed(1), cer.clone()));

		// failed when transferring without owner 
		assert_noop!(NikaModule::transfer(Origin::signed(2), 3, cer.clone()), Error::<Test>::OwnerNeeded);

		assert_eq!(NikaModule::proofs(cer.clone()), Some((1, 0)));

		// transfer normally
		assert_ok!(NikaModule::transfer(Origin::signed(1), 2, cer.clone()));

		assert_eq!(NikaModule::proofs(cer.clone()), Some((2, 0)));
	});
}

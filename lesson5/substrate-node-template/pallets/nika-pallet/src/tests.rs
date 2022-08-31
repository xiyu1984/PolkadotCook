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


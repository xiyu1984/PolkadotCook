//! Benchmarking setup for pallet-poe

use super::*;

#[allow(unused)]
use crate::Pallet as Poe;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_std::vec;
use frame_support::traits::Get;

benchmarks! {
	register {
		let d in 0 .. T::MaxClaimLength::get();
        let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), claim.clone())
	verify {
		assert_eq!(StarKeeper::<T>::get(&claim).is_some(), true);
	}

	unregister {
		let d in 0 .. T::MaxClaimLength::get();
        let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
		Poe::<T>::register(RawOrigin::Signed(caller.clone()).into(), claim.clone());
	}: _(RawOrigin::Signed(caller), claim.clone())
	verify {
		assert_eq!(StarKeeper::<T>::get(&claim).is_none(), true);
	}
	
	transfer {
		let d in 0 .. T::MaxClaimLength::get();
        let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
		let target: T::AccountId = account("target", 0, 0);
		Poe::<T>::register(RawOrigin::Signed(caller.clone()).into(), claim.clone());
	}: _(RawOrigin::Signed(caller.clone()), target, claim.clone())
	verify {
		assert_eq!(StarKeeper::<T>::get(&claim).is_some(), true);
	}

	impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test);
}

#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks};
use frame_support::storage::{StorageDoubleMap, StorageMap};
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_system::{EventRecord, RawOrigin};
use sp_arithmetic::traits::{One, Zero};
use sp_runtime::traits::{Hash, SaturatedConversion};
use sp_std::boxed::Box;
use sp_std::vec;
use sp_std::vec::Vec;

use crate::Module as Bounty;
use balances::Module as Balances;
use common::council::CouncilBudgetManager;
use frame_system::Module as System;
use membership::Module as Membership;

use crate::{
    BalanceOf, Bounties, BountyActor, BountyCreationParameters, BountyMilestone, Call, Event,
    Module, Trait, WorkEntries,
};

pub fn run_to_block<T: Trait>(target_block: T::BlockNumber) {
    let mut current_block = System::<T>::block_number();
    while current_block < target_block {
        System::<T>::on_finalize(current_block);
        Bounty::<T>::on_finalize(current_block);

        current_block += One::one();
        System::<T>::set_block_number(current_block);

        System::<T>::on_initialize(current_block);
        Bounty::<T>::on_initialize(current_block);
    }
}

fn assert_last_event<T: Trait>(generic_event: <T as Trait>::Event) {
    let events = System::<T>::events();
    let system_event: <T as frame_system::Trait>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn get_byte(num: u32, byte_number: u8) -> u8 {
    ((num & (0xff << (8 * byte_number))) >> 8 * byte_number) as u8
}

// Method to generate a distintic valid handle
// for a membership. For each index.
fn handle_from_id<T: Trait + membership::Trait>(id: u32) -> Vec<u8> {
    let mut handle = vec![];

    for i in 0..4 {
        handle.push(get_byte(id, i));
    }

    handle
}

//defines initial balance
fn initial_balance<T: Trait>() -> T::Balance {
    1000000.into()
}

fn member_funded_account<T: Trait + membership::Trait>(
    name: &'static str,
    id: u32,
) -> (T::AccountId, T::MemberId) {
    let account_id = account::<T::AccountId>(name, id, SEED);
    let handle = handle_from_id::<T>(id);

    // Give balance for buying membership
    let _ = Balances::<T>::make_free_balance_be(&account_id, initial_balance::<T>());

    let params = membership::BuyMembershipParameters {
        root_account: account_id.clone(),
        controller_account: account_id.clone(),
        name: None,
        handle: Some(handle),
        avatar_uri: None,
        about: None,
        referrer_id: None,
    };

    Membership::<T>::buy_membership(RawOrigin::Signed(account_id.clone()).into(), params).unwrap();

    let _ = Balances::<T>::make_free_balance_be(&account_id, initial_balance::<T>());

    (account_id, T::MemberId::from(id.saturated_into()))
}

const MAX_BYTES: u32 = 50000;
const SEED: u32 = 0;

benchmarks! {
    where_clause {
        where T: council::Trait,
              T: balances::Trait,
              T: membership::Trait,
    }
    _{ }

    create_bounty_by_council {
        let i in 1 .. MAX_BYTES;
        let metadata = vec![0u8].repeat(i as usize);
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            ..Default::default()
        };

    }: create_bounty (RawOrigin::Root, params.clone(), metadata)
    verify {
        let bounty_id: T::BountyId = 1u32.into();

        assert!(Bounties::<T>::contains_key(bounty_id));
        assert_eq!(Bounty::<T>::bounty_count(), 1); // Bounty counter was updated.
        assert_last_event::<T>(Event::<T>::BountyCreated(bounty_id, params).into());
    }

    create_bounty_by_member {
        let i in 1 .. MAX_BYTES;
        let metadata = vec![0u8].repeat(i as usize);
        let cherry: BalanceOf<T> = 100.into();

        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        T::CouncilBudgetManager::set_budget(cherry);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            creator: BountyActor::Member(member_id),
            ..Default::default()
        };

    }: create_bounty (RawOrigin::Signed(account_id), params.clone(), metadata)
    verify {
        let bounty_id: T::BountyId = 1u32.into();

        assert!(Bounties::<T>::contains_key(bounty_id));
        assert_eq!(Bounty::<T>::bounty_count(), 1); // Bounty counter was updated.
        assert_last_event::<T>(Event::<T>::BountyCreated(bounty_id, params).into());
    }

    cancel_bounty_by_council {
        let cherry: BalanceOf<T> = 100.into();
        let max_amount: BalanceOf<T> = 1000.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let creator = BountyActor::Council;
        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            creator: creator.clone(),
            max_amount,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));
    }: cancel_bounty(RawOrigin::Root, creator.clone(), bounty_id)
    verify {
        let bounty = Bounty::<T>::bounties(bounty_id);

        assert!(matches!(bounty.milestone, BountyMilestone::Canceled));
        assert_last_event::<T>(Event::<T>::BountyCanceled(bounty_id, creator).into());
    }

    cancel_bounty_by_member {
        let cherry: BalanceOf<T> = 100.into();
        let max_amount: BalanceOf<T> = 1000.into();
        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        T::CouncilBudgetManager::set_budget(cherry);

        let creator = BountyActor::Member(member_id);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            creator: creator.clone(),
            max_amount,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            params,
            Vec::new()
        ).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));
    }: cancel_bounty(RawOrigin::Signed(account_id), creator.clone(), bounty_id)
    verify {
        let bounty = Bounty::<T>::bounties(bounty_id);

        assert!(matches!(bounty.milestone, BountyMilestone::Canceled));
        assert_last_event::<T>(Event::<T>::BountyCanceled(bounty_id, creator).into());
    }

    veto_bounty {
        let max_amount: BalanceOf<T> = 1000.into();
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            max_amount,
            cherry,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));
    }: _ (RawOrigin::Root, bounty_id)
    verify {
        let bounty = Bounty::<T>::bounties(bounty_id);

        assert!(matches!(bounty.milestone, BountyMilestone::Canceled));
        assert_last_event::<T>(Event::<T>::BountyVetoed(bounty_id).into());
    }

    fund_bounty_by_member {
        let max_amount: BalanceOf<T> = 100.into();
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            max_amount,
            cherry,
            ..Default::default()
        };
        // should reach default max bounty funding amount
        let amount: BalanceOf<T> = 100.into();

        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));
    }: fund_bounty (RawOrigin::Signed(account_id.clone()), BountyActor::Member(member_id), bounty_id, amount)
    verify {
        assert_eq!(Balances::<T>::usable_balance(&account_id), initial_balance::<T>() - amount);
        assert_last_event::<T>(Event::<T>::BountyMaxFundingReached(bounty_id).into());
    }

    fund_bounty_by_council {
        let max_amount: BalanceOf<T> = 100.into();
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry + max_amount);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            max_amount,
            cherry,
            ..Default::default()
        };
        // should reach default max bounty funding amount
        let amount: BalanceOf<T> = 100.into();

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));
    }: fund_bounty (RawOrigin::Root, BountyActor::Council, bounty_id, amount)
    verify {
        assert_eq!(T::CouncilBudgetManager::get_budget(), Zero::zero());
        assert_last_event::<T>(Event::<T>::BountyMaxFundingReached(bounty_id).into());
    }

    withdraw_funding_by_member {
        let funding_period = 1;
        let bounty_amount = 200;
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let params = BountyCreationParameters::<T>{
            funding_period: Some(funding_period.into()),
            work_period: One::one(),
            judging_period: One::one(),
            max_amount: bounty_amount.into(),
            min_amount: bounty_amount.into(),
            cherry,
            ..Default::default()
        };
        // should reach default max bounty funding amount
        let amount: BalanceOf<T> = 100.into();

        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        let funder = BountyActor::Member(member_id);

        Bounty::<T>::fund_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            funder.clone(),
            bounty_id,
            amount
        ).unwrap();

        run_to_block::<T>((funding_period + 1).into());

    }: withdraw_funding (RawOrigin::Signed(account_id.clone()), funder, bounty_id)
    verify {
        assert_eq!(Balances::<T>::usable_balance(&account_id), initial_balance::<T>() + cherry);
        assert_last_event::<T>(Event::<T>::BountyRemoved(bounty_id).into());
    }

    withdraw_funding_by_council {
        let funding_period = 1;
        let bounty_amount = 200;
        let cherry: BalanceOf<T> = 100.into();
        let funding_amount: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry + funding_amount);

        let params = BountyCreationParameters::<T>{
            funding_period: Some(funding_period.into()),
            work_period: One::one(),
            judging_period: One::one(),
            max_amount: bounty_amount.into(),
            min_amount: bounty_amount.into(),
            cherry,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        let funder = BountyActor::Council;

        Bounty::<T>::fund_bounty(
            RawOrigin::Root.into(),
            funder.clone(),
            bounty_id,
            funding_amount
        ).unwrap();

        run_to_block::<T>((funding_period + 1).into());

    }: withdraw_funding(RawOrigin::Root, funder, bounty_id)
    verify {
        assert_eq!(T::CouncilBudgetManager::get_budget(), cherry + funding_amount);
        assert_last_event::<T>(Event::<T>::BountyRemoved(bounty_id).into());
    }

    withdraw_creator_cherry_by_council {
        let max_amount: BalanceOf<T> = 1000.into();
        let cherry: BalanceOf<T> = 100.into();

        T::CouncilBudgetManager::set_budget(cherry);

        let creator = BountyActor::Council;
        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            max_amount,
            creator: creator.clone(),
            ..Default::default()
        };

        Bounty::<T>::create_bounty(RawOrigin::Root.into(), params, Vec::new()).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        Bounty::<T>::cancel_bounty(RawOrigin::Root.into(), creator.clone(), bounty_id).unwrap();

    }: withdraw_creator_cherry(RawOrigin::Root, creator.clone(), bounty_id)
    verify {
        assert!(!Bounties::<T>::contains_key(bounty_id));
        assert_last_event::<T>(Event::<T>::BountyRemoved(bounty_id).into());
    }

    withdraw_creator_cherry_by_member {
        let max_amount: BalanceOf<T> = 1000.into();
        let cherry: BalanceOf<T> = 100.into();
        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        let creator = BountyActor::Member(member_id);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            cherry,
            creator: creator.clone(),
            max_amount,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            params,
            Vec::new()
        ).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        Bounty::<T>::cancel_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            creator.clone(),
            bounty_id
        ).unwrap();

    }: withdraw_creator_cherry(RawOrigin::Signed(account_id), creator.clone(), bounty_id)
    verify {
        assert!(!Bounties::<T>::contains_key(bounty_id));
        assert_last_event::<T>(Event::<T>::BountyRemoved(bounty_id).into());
    }

    announce_work_entry {
        let cherry: BalanceOf<T> = 100.into();
        let funding_amount: BalanceOf<T> = 100.into();
        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        let creator = BountyActor::Member(member_id);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            creator: creator.clone(),
            cherry,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            params,
            Vec::new()
        ).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        Bounty::<T>::fund_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            creator.clone(),
            bounty_id,
            funding_amount
        ).unwrap();

        let (account_id, member_id) = member_funded_account::<T>("member2", 1);

    }: _(RawOrigin::Signed(account_id.clone()), member_id, bounty_id, Some(account_id.clone()))
    verify {
        let entry_id: T::WorkEntryId = Bounty::<T>::work_entry_count().into();

        assert!(WorkEntries::<T>::contains_key(bounty_id, entry_id));
        assert_last_event::<T>(
            Event::<T>::WorkEntryAnnounced(bounty_id, entry_id, member_id, Some(account_id)).into()
        );
    }

    withdraw_work_entry {
        let cherry: BalanceOf<T> = 100.into();
        let funding_amount: BalanceOf<T> = 100.into();
        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        let creator = BountyActor::Member(member_id);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            creator: creator.clone(),
            cherry,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            params,
            Vec::new()
        ).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        Bounty::<T>::fund_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            creator.clone(),
            bounty_id,
            funding_amount
        ).unwrap();

        let (account_id, member_id) = member_funded_account::<T>("member2", 1);

        Bounty::<T>::announce_work_entry(
            RawOrigin::Signed(account_id.clone()).into(),
            member_id,
            bounty_id,
            Some(account_id.clone())
        ).unwrap();

        let entry_id: T::WorkEntryId = Bounty::<T>::work_entry_count().into();

    }: _(RawOrigin::Signed(account_id.clone()), member_id, bounty_id, entry_id)
    verify {
        assert!(!WorkEntries::<T>::contains_key(bounty_id, entry_id));
        assert_last_event::<T>(
            Event::<T>::WorkEntryWithdrawn(bounty_id, entry_id, member_id).into()
        );
    }

    submit_work {
        let i in 0 .. MAX_BYTES;
        let work_data = vec![0u8].repeat(i as usize);

        let cherry: BalanceOf<T> = 100.into();
        let funding_amount: BalanceOf<T> = 100.into();
        let (account_id, member_id) = member_funded_account::<T>("member1", 0);

        let creator = BountyActor::Member(member_id);

        let params = BountyCreationParameters::<T>{
            work_period: One::one(),
            judging_period: One::one(),
            creator: creator.clone(),
            cherry,
            ..Default::default()
        };

        Bounty::<T>::create_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            params,
            Vec::new()
        ).unwrap();

        let bounty_id: T::BountyId = Bounty::<T>::bounty_count().into();

        assert!(Bounties::<T>::contains_key(bounty_id));

        Bounty::<T>::fund_bounty(
            RawOrigin::Signed(account_id.clone()).into(),
            creator.clone(),
            bounty_id,
            funding_amount
        ).unwrap();

        let (account_id, member_id) = member_funded_account::<T>("member2", 1);

        Bounty::<T>::announce_work_entry(
            RawOrigin::Signed(account_id.clone()).into(),
            member_id,
            bounty_id,
            Some(account_id.clone())
        ).unwrap();

        let entry_id: T::WorkEntryId = Bounty::<T>::work_entry_count().into();

    }: _(RawOrigin::Signed(account_id.clone()), member_id, bounty_id, entry_id, work_data.clone())
    verify {
        let entry = Bounty::<T>::work_entries(bounty_id, entry_id);
        let hashed = T::Hashing::hash(&work_data);
        let work_data_hash = hashed.as_ref().to_vec();

        assert_eq!(entry.last_submitted_work, Some(work_data_hash));
        assert_last_event::<T>(
            Event::<T>::WorkSubmitted(bounty_id, entry_id, member_id, work_data).into()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mocks::{build_test_externalities, Test};
    use frame_support::assert_ok;

    #[test]
    fn create_bounty_by_council() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_create_bounty_by_council::<Test>());
        });
    }

    #[test]
    fn create_bounty_by_member() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_create_bounty_by_member::<Test>());
        });
    }

    #[test]
    fn cancel_bounty_by_council() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_cancel_bounty_by_council::<Test>());
        });
    }

    #[test]
    fn cancel_bounty_by_member() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_cancel_bounty_by_member::<Test>());
        });
    }

    #[test]
    fn veto_bounty() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_veto_bounty::<Test>());
        });
    }

    #[test]
    fn fund_bounty_by_member() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_fund_bounty_by_member::<Test>());
        });
    }

    #[test]
    fn fund_bounty_by_council() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_fund_bounty_by_council::<Test>());
        });
    }

    #[test]
    fn withdraw_funding_by_member() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_withdraw_funding_by_member::<Test>());
        });
    }

    #[test]
    fn withdraw_funding_by_council() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_withdraw_funding_by_council::<Test>());
        });
    }

    #[test]
    fn withdraw_creator_cherry_by_council() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_withdraw_creator_cherry_by_council::<Test>());
        });
    }

    #[test]
    fn withdraw_creator_cherry_by_member() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_withdraw_creator_cherry_by_member::<Test>());
        });
    }

    #[test]
    fn announce_work_entry() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_announce_work_entry::<Test>());
        });
    }

    #[test]
    fn withdraw_work_entry() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_withdraw_work_entry::<Test>());
        });
    }

    #[test]
    fn submit_work() {
        build_test_externalities().execute_with(|| {
            assert_ok!(test_benchmark_submit_work::<Test>());
        });
    }
}

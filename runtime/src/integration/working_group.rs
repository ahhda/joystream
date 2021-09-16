use frame_support::StorageMap;
use sp_std::marker::PhantomData;

use crate::{
    ContentDirectoryWorkingGroupInstance, GatewayWorkingGroupInstance,
    OperationsWorkingGroupInstanceAlpha, OperationsWorkingGroupInstanceBeta,
    OperationsWorkingGroupInstanceGamma, StorageWorkingGroupInstance,
};
use stake::{BalanceOf, NegativeImbalance};

// Will be removed in the next releases.
#[allow(clippy::upper_case_acronyms)]
pub struct ContentDirectoryWgStakingEventsHandler<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<ContentDirectoryWorkingGroupInstance>>
    stake::StakingEventsHandler<T> for ContentDirectoryWgStakingEventsHandler<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, ContentDirectoryWorkingGroupInstance>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, ContentDirectoryWorkingGroupInstance>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

pub struct StorageWgStakingEventsHandler<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<StorageWorkingGroupInstance>>
    stake::StakingEventsHandler<T> for StorageWgStakingEventsHandler<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, StorageWorkingGroupInstance>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, StorageWorkingGroupInstance>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

pub struct OperationsWgStakingEventsHandlerAlpha<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<OperationsWorkingGroupInstanceAlpha>>
    stake::StakingEventsHandler<T> for OperationsWgStakingEventsHandlerAlpha<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, OperationsWorkingGroupInstanceAlpha>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, OperationsWorkingGroupInstanceAlpha>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

pub struct GatewayWgStakingEventsHandler<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<GatewayWorkingGroupInstance>>
    stake::StakingEventsHandler<T> for GatewayWgStakingEventsHandler<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, GatewayWorkingGroupInstance>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, GatewayWorkingGroupInstance>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

pub struct OperationsWgStakingEventsHandlerBeta<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<OperationsWorkingGroupInstanceBeta>>
    stake::StakingEventsHandler<T> for OperationsWgStakingEventsHandlerBeta<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, OperationsWorkingGroupInstanceBeta>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, OperationsWorkingGroupInstanceBeta>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

pub struct OperationsWgStakingEventsHandlerGamma<T> {
    pub marker: PhantomData<T>,
}

impl<T: stake::Trait + working_group::Trait<OperationsWorkingGroupInstanceGamma>>
    stake::StakingEventsHandler<T> for OperationsWgStakingEventsHandlerGamma<T>
{
    /// Unstake remaining sum back to the source_account_id
    fn unstaked(
        stake_id: &<T as stake::Trait>::StakeId,
        _unstaked_amount: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        // Stake not related to a staked role managed by the hiring module.
        if !hiring::ApplicationIdByStakingId::<T>::contains_key(*stake_id) {
            return remaining_imbalance;
        }

        let hiring_application_id = hiring::ApplicationIdByStakingId::<T>::get(*stake_id);

        if working_group::MemberIdByHiringApplicationId::<T, OperationsWorkingGroupInstanceGamma>::contains_key(
            hiring_application_id,
        ) {
            return <working_group::Module<T, OperationsWorkingGroupInstanceGamma>>::refund_working_group_stake(
				*stake_id,
				remaining_imbalance,
			);
        }

        remaining_imbalance
    }

    /// Empty handler for the slashing.
    fn slashed(
        _: &<T as stake::Trait>::StakeId,
        _: Option<<T as stake::Trait>::SlashId>,
        _: BalanceOf<T>,
        _: BalanceOf<T>,
        remaining_imbalance: NegativeImbalance<T>,
    ) -> NegativeImbalance<T> {
        remaining_imbalance
    }
}

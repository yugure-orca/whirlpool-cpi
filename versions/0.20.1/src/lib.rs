#![allow(unused)]

use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub mod state;
use crate::state::*;

pub mod context;
use crate::context::*;

pub mod util;

#[program]
pub mod whirlpool {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        fee_authority: Pubkey,
        collect_protocol_fees_authority: Pubkey,
        reward_emissions_super_authority: Pubkey,
        default_protocol_fee_rate: u16,
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        bumps: WhirlpoolBumps,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_tick_array(
        ctx: Context<InitializeTickArray>,
        start_tick_index: i32,
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_fee_tier(
        ctx: Context<InitializeFeeTier>,
        tick_spacing: u16,
        default_fee_rate: u16,
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_reward(
        ctx: Context<InitializeReward>,
        reward_index: u8
    ) -> ProgramResult { Ok(()) }

    pub fn set_reward_emissions(
        ctx: Context<SetRewardEmissions>,
        reward_index: u8,
        emissions_per_second_x64: u128,
    ) -> ProgramResult { Ok(()) }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        bumps: OpenPositionBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> ProgramResult { Ok(()) }

    pub fn open_position_with_metadata(
        ctx: Context<OpenPositionWithMetadata>,
        bumps: OpenPositionWithMetadataBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> ProgramResult { Ok(()) }

    pub fn increase_liquidity(
        ctx: Context<ModifyLiquidity>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    ) -> ProgramResult { Ok(()) }

    pub fn decrease_liquidity(
        ctx: Context<ModifyLiquidity>,
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
    ) -> ProgramResult { Ok(()) }

    pub fn update_fees_and_rewards(
        ctx: Context<UpdateFeesAndRewards>
    ) -> ProgramResult { Ok(()) }

    pub fn collect_fees(
        ctx: Context<CollectFees>
    ) -> ProgramResult { Ok(()) }

    pub fn collect_reward(
        ctx: Context<CollectReward>,
        reward_index: u8
    ) -> ProgramResult { Ok(()) }

    pub fn collect_protocol_fees(
        ctx: Context<CollectProtocolFees>
    ) -> ProgramResult { Ok(()) }

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> ProgramResult { Ok(()) }

    pub fn close_position(
        ctx: Context<ClosePosition>
    ) -> ProgramResult { Ok(()) }

    pub fn set_default_fee_rate(
        ctx: Context<SetDefaultFeeRate>,
        default_fee_rate: u16,
    ) -> ProgramResult { Ok(()) }

    pub fn set_default_protocol_fee_rate(
        ctx: Context<SetDefaultProtocolFeeRate>,
        default_protocol_fee_rate: u16,
    ) -> ProgramResult { Ok(()) }

    pub fn set_fee_rate(
        ctx: Context<SetFeeRate>,
        fee_rate: u16
    ) -> ProgramResult { Ok(()) }

    pub fn set_protocol_fee_rate(
        ctx: Context<SetProtocolFeeRate>,
        protocol_fee_rate: u16,
    ) -> ProgramResult { Ok(()) }

    pub fn set_fee_authority(
        ctx: Context<SetFeeAuthority>
    ) -> ProgramResult { Ok(()) }

    pub fn set_collect_protocol_fees_authority(
        ctx: Context<SetCollectProtocolFeesAuthority>,
    ) -> ProgramResult { Ok(()) }

    pub fn set_reward_authority(
        ctx: Context<SetRewardAuthority>,
        reward_index: u8
    ) -> ProgramResult { Ok(()) }

    pub fn set_reward_authority_by_super_authority(
        ctx: Context<SetRewardAuthorityBySuperAuthority>,
        reward_index: u8,
    ) -> ProgramResult { Ok(()) }

    pub fn set_reward_emissions_super_authority(
        ctx: Context<SetRewardEmissionsSuperAuthority>,
    ) -> ProgramResult { Ok(()) }

    pub fn two_hop_swap(
        ctx: Context<TwoHopSwap>,
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128,
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_position_bundle(
        ctx: Context<InitializePositionBundle>
    ) -> ProgramResult { Ok(()) }

    pub fn initialize_position_bundle_with_metadata(
        ctx: Context<InitializePositionBundleWithMetadata>,
    ) -> ProgramResult { Ok(()) }

    pub fn delete_position_bundle(
        ctx: Context<DeletePositionBundle>
    ) -> ProgramResult { Ok(()) }

    pub fn open_bundled_position(
        ctx: Context<OpenBundledPosition>,
        bundle_index: u16,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> ProgramResult { Ok(()) }

    pub fn close_bundled_position(
        ctx: Context<CloseBundledPosition>,
        bundle_index: u16,
    ) -> ProgramResult { Ok(()) }

}
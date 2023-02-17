use crate::impls::pool::data_struct::*;
use ink_env::DefaultEnvironment;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp22::{PSP22Error, PSP22Ref},
    storage::Mapping,
    traits::{AccountId, Balance},
};

pub mod liquidity_num {
    use primitive_types::U256;
    pub const MIN_TICK: i32 = -887272;
    pub const MAX_TICK: i32 = -MIN_TICK;
    pub const MIN_SQRT_RATIO: u128 = 4295128739;
    pub const MAX_SQRT_RATIO: U256 =
        U256::from("1461446703485210103287273052203988822378723970342");
}

#[inline]
pub fn get_sqrt_ratio_at_tick(tick: i32) -> u128 {
    0
}

#[inline]
pub fn get_tick_at_sqrt_ratio(sqrt_ratio_x96: u128) -> i32 {
    0
}

#[inline]
pub fn _compute_swap_step(
    sqrt_ratio_current_x96: u128,
    sqrt_ratio_target_x96: u128,
    liquidity: u128,
    amount_remaining: i128,
    fee_pips: u32,
) -> (u128, u128, u128, u128) {
    (0, 0, 0, 0)
}
#[inline]
pub fn tick_bitmap(tick: i32) -> Result<u128, LiquidityHelperError> {
    Ok(0)
}

#[inline]
pub fn mul_div(a: u128, b: u128, denominator: u128) -> u128 {
    0
}

#[inline]
pub fn next_initialized_tick_within_oneword(
    tick: i32,
    tick_spacing: i32,
    lte: bool,
) -> (u128, bool) {
    (0, false)
}

#[inline]
pub fn cross(
    // obs: Mapping<i32, TickInfo>,
    tick: i32,
    fee_growth_global_0x128: u128,
    fee_growth_global_1x128: u128,
    seconds_per_liquidity_cumulative_x128: u128,
    tick_cumulative: i64,
    time: u32,
) -> i128 {
    // Tick.Info storage info = self[tick];
    // info.feeGrowthOutside0X128 = feeGrowthGlobal0X128 - info.feeGrowthOutside0X128;
    // info.feeGrowthOutside1X128 = feeGrowthGlobal1X128 - info.feeGrowthOutside1X128;
    // info.secondsPerLiquidityOutsideX128 = secondsPerLiquidityCumulativeX128 - info.secondsPerLiquidityOutsideX128;
    // info.tickCumulativeOutside = tickCumulative - info.tickCumulativeOutside;
    // info.secondsOutside = time - info.secondsOutside;
    // liquidityNet = info.liquidityNet;
    0
}
#[inline]
pub fn write(
    index: u16,
    block_timestamp: u32,
    tick: i32,
    liquidity: u128,
    cardinality: u16,
    cardinality_next: u16,
) -> (u16, u16) {
    (0, 0)
}

#[inline]
pub fn add_delta(x: u128, y: i128) -> u128 {
    0
}

#[inline]
pub fn observe_single(
    time: u32,
    seconds_ago: u32,
    tick: i32,
    index: u16,
    liquidity: u128,
    cardinality: u16,
) -> (i64, u128) {
    (0, 0)
}
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LiquidityHelperError {
    TickError,
    AddOverflowBalance0,
    AddOverflowBalance1,
    M0,
    M1,
    CheckedNeg0,
    CheckedNeg1,
    BurningInsuficientBalance,
}

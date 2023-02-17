use ink_env::DefaultEnvironment;
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::psp22::{PSP22Error, PSP22Ref},
    traits::{AccountId, Balance},
};
use primitive_types::U256;

pub mod liquidity_num {
    pub const MIN_TICK: i32 = -887272;
    pub const MAX_TICK: i32 = -MIN_TICK;
    pub const MIN_SQRT_RATIO: u128 = 4295128739;
    pub const MAX_SQRT_RATIO: u128 = 1461446703485210103287273052203988822378723970342;
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

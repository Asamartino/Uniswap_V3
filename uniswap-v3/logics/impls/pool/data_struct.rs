use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Balance, Timestamp},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct TickInfo {
    // the total position liquidity that references this tick
    pub liquidity_gross: u128,
    // amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left),
    pub liquidity_net: i128,
    // fee growth per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub fee_Growth_outside_0x128: u128,
    pub fee_growth_outside_1x128: u128,
    // the cumulative tick value on the other side of the tick
    pub tick_cumulative_outside: i64,
    // the seconds per unit of liquidity on the _other_ side of this tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub seconds_per_liquidity_outside_x128: u128,
    // the seconds spent on the other side of the tick (relative to the current tick)
    // only has relative meaning, not absolute — the value depends on when the tick is initialized
    pub seconds_outside: u32,
    // true iff the tick is initialized, i.e. the value is exactly equivalent to the expression liquidityGross != 0
    // these 8 bits are set to prevent fresh sstores when crossing newly initialized ticks
    pub initialized: bool,
}

#[derive(Debug, Clone, SpreadLayout, PackedLayout, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct PositionInfo {
    // the amount of liquidity owned by this position
    pub liquidity: u128,
    // fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub fee_growth_inside_0_last_x128: u128,
    pub fee_growth_inside_1_last_x128: u128,
    // the fees owed to the position owner in token0/token1
    pub tokens_owed_0: Balance,
    pub tokens_owed_1: Balance,
}

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Slot {
    // the current price
    pub sqrt_price_x96: u128,
    // the current tick
    pub tick: i32,
    // the most-recently updated index of the observations array
    pub observation_index: u16,
    // the current maximum number of observations that are being stored
    pub observation_cardinality: u16,
    // the next maximum number of observations to store, triggered in observations.write
    pub observation_cardinality_next: u16,
    // the current protocol fee as a percentage of the swap fee taken on withdrawal
    // represented as an integer denominator (1/x)%
    pub fee_protocol: u8,
    // whether the pool is locked
    pub unlocked: bool,
}

// #[derive(Default, Debug)]
// #[openbrush::upgradeable_storage(STORAGE_KEY)]
// pub struct ProtocolFees {
//     pub token0: Balance,
//     pub token1: Balance,
// }

#[derive(Default, Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct Observation {
    // the block timestamp of the observation
    pub block_timestamp: u32,
    // the tick accumulator, i.e. tick * time elapsed since the pool was first initialized
    pub tick_cumulative: i64,
    // the seconds per liquidity, i.e. seconds elapsed / max(1, liquidity) since the pool was first initialized
    pub seconds_per_liquidity_cumulative_x128: u128,
    // whether or not the observation is initialized
    pub initialized: bool,
}

use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout, StorageLayout};
use openbrush::traits::Balance;

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
pub struct SwapCache {
    // the protocol fee for the input token
    pub fee_protocol: u8,
    // liquidity at the beginning of the swap
    pub liquidity_start: u128,
    // the timestamp of the current block
    pub block_timestamp: u32,
    // the current value of the tick accumulator, computed only if we cross an initialized tick
    pub tick_cumulative: i64,
    // the current value of seconds per liquidity accumulator, computed only if we cross an initialized tick
    pub seconds_per_liquidity_cumulative_x128: u128,
    // whether we've computed and cached the above two accumulators
    pub computed_latest_observations: bool,
}

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct SwapState {
    // the amount remaining to be swapped in/out of the input/output asset
    pub amount_specified_remaining: i128,
    // the amount already swapped out/in of the output/input asset
    pub amount_calculated: i128,
    // current sqrt(price)
    pub sqrt_price_x96: u128,
    // the tick associated with the current price
    pub tick: i32,
    // the global fee growth of the input token
    pub fee_growth_global_x128: u128, //U258
    // amount of input token paid as protocol fee
    pub protocol_fee: u128,
    // the current liquidity in range
    pub liquidity: u128,
}
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct StepComputations {
    pub sqrt_price_start_x96: u128,
    pub tick_next: i32,
    pub initialized: bool,
    pub sqrt_price_next_x96: u128, //U256
    pub amount_in: u128,           //U256
    pub amount_out: u128,          //U256
    pub fee_amount: u128,          //U256
}
#[derive(
    Default, Debug, Clone, Copy, SpreadLayout, SpreadAllocate, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
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

#[derive(Default, Debug, Copy, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(SpreadLayout, PackedLayout, scale_info::TypeInfo)
)]
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

// #[derive(Debug, SpreadLayout, SpreadAllocate,  scale::Encode, scale::Decode )]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
// pub struct ObservationArray{
//     pub array: [Observation; 65535]
// }

// impl Default for ObservationArray {
//     fn default() -> ObservationArray {
//         ObservationArray {
//             array: [  Observation {
//                               block_timestamp: 0,
//                               tick_cumulative: 0,
//                               seconds_per_liquidity_cumulative_x128: 0,
//                               initialized: false,
//                             }; 65535],
//         }
//     }
// }

// impl StorageLayout for ObservationArray{
//     fn layout(key: &mut ink_primitives::KeyPtr) ->  ink_metadata::layout::Layout{Layout::Struct(Observation
//         {
//         block_timestamp: 0,
//         tick_cumulative: 0,
//         seconds_per_liquidity_cumulative_x128: 0,
//         initialized: false,
//       })
//     }
// }
// impl Default for Observation {
//         fn default() -> Observation {
//             Observation {
//               block_timestamp: 0,
//               tick_cumulative: 0,
//               seconds_per_liquidity_cumulative_x128: 0,
//               initialized: false,
//             }
//         }
// }

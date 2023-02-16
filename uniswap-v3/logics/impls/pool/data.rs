use crate::impls::pool::data_struct::*;

use ink_storage::traits::StorageLayout;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Balance, Timestamp},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    pub token_0: AccountId,
    pub token_1: AccountId,
    pub fee: u32,
    pub tick_spacing: i32,
    pub max_liquidity_per_tick: u128,
    pub slot_0: Slot,
    pub fee_growth_global_0x128: u128,
    pub fee_growth_global_1x128: u128,
    pub liquidity: u128,
    pub ticks: Mapping<i32, TickInfo>,
    pub tick_bitmap: Mapping<i32, u128>,
    pub positions: Mapping<[u8; 32], PositionInfo>,
    pub fee0: Balance,
    pub fee1: Balance,
    pub swap_cache: SwapCache,
    pub swap_state: SwapState,
    pub step_computations: StepComputations,
    // pub observation_array: [Observation;65535],
}

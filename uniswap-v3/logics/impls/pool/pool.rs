use crate::impls::pool::data_struct::*;
use crate::traits::factory::FactoryRef;
use ink_env::hash::Blake2x256;
use ink_prelude::vec::Vec;

use crate::{ensure, helpers::transfer_helper::safe_transfer};
use crate::{impls::pool::*, traits::pool::*};
use openbrush::{
    contracts::{ownable::*, psp22::*, reentrancy_guard::*, traits::psp22::PSP22Ref},
    traits::{AccountId, Balance, Storage, Timestamp},
};

pub trait Internal {
    fn _emit_initialize_event(&self, sqrt_price_x96: u128, tick: i32);
    fn _emit_mint_event(
        &self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount: Balance,
    );
    fn _emit_collect_event(
        &self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Balance,
        amount1_requested: Balance,
    );
    fn _emit_burn_event(&self, tick_lower: i32, tick_upper: i32, amount: Balance);
    fn _emit_swap_event(
        &self,
        recipient: AccountId,
        zero_for_one: bool,
        amount_specified: i128,
        sqrt_price_limit_x96: u128,
    );
    fn _emit_flash_event(
        &self,
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        paid0: u128,
        paid1: u128,
    );
    fn _emit_set_fee_protocol_event(
        &self,
        fee_protocol0_old: u8,
        fee_protocol1_old: u8,
        fee_protocol0_new: u8,
        fee_protocol1_new: u8,
    );
    fn _emit_collect_protocol_event(
        &self,
        sender: AccountId,
        recipient: AccountId,
        amount0_requested: Balance,
        amount1_requested: Balance,
    );
}
impl<T: Storage<data::Data> + Internal> Pool for T {
    fn initialize(
        &mut self,
        token_0: AccountId,
        token_1: AccountId,
        fee: u32,
    ) -> Result<(), PoolError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        self.data::<data::Data>().fee = fee;
        Ok(())
    }
    fn collect(
        &mut self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError> {
        let mut amount_0: Balance;
        let mut amount_1: Balance;
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        let caller = Self::env().caller();
        let mut owned_amount_0 = 0;
        let mut owned_amount_1 = 0;
        let position_info = self.get_position(caller, tick_lower, tick_upper);
        if let Some(info) = position_info {
            owned_amount_0 = info.tokens_owed_0;
            owned_amount_1 = info.tokens_owed_1;
        }

        if amount0_requested > owned_amount_0 {
            amount_0 = owned_amount_0;
        } else {
            amount_0 = amount0_requested;
        }
        if amount1_requested > owned_amount_1 {
            amount_1 = owned_amount_1;
        } else {
            amount_1 = amount1_requested;
        }
        if amount_0 > 0 {
            owned_amount_0 -= amount_0;
            safe_transfer(token_0, recipient, amount_0);
        } else {
            // ensure!(amount_0 == 0, PoolError);
        }
        if amount_1 > 0 {
            owned_amount_1 -= amount_1;
            safe_transfer(token_1, recipient, amount_1);
        } else {
            // ensure!(amount_1 == 0, PoolError);
        }
        self._emit_collect_event(recipient, tick_lower, tick_upper, amount_0, amount_1);
        Ok((amount_0, amount_1))
    }

    fn burn(
        &mut self,
        tick_lower: i32,
        tick_upper: i32,
        amount: i128,
    ) -> Result<(u128, u128), PoolError> {
        Ok((0, 0))
    }

    fn swap(
        &mut self,
        recipient: AccountId,
        zero_for_one: bool,
        amount_specified: i128,
        sqrt_price_limit_x96: u128,
    ) -> Result<(u128, u128), PoolError> {
        Ok((0, 0))
    }

    fn flash(
        &mut self,
        recipient: AccountId,
        amount0: u128,
        amount1: u128,
        data: Vec<u8>,
    ) -> Result<(), PoolError> {
        Ok(())
    }

    // ProtocolOwnerActions
    // TODO: add fee protocol
    fn collect_protocol(
        &mut self,
        sender: AccountId,
        recipient: AccountId,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError> {
        let caller = Self::env().caller();
        let mut amount_0: Balance;
        let mut amount_1: Balance;
        let mut fee_0 = self.data::<data::Data>().fee0;
        let mut fee_1 = self.data::<data::Data>().fee1;
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        if amount0_requested > fee_0 {
            amount_0 = fee_0;
        } else {
            amount_0 = amount0_requested;
        }
        if amount1_requested > fee_1 {
            amount_1 = fee_1;
        } else {
            amount_1 = amount1_requested;
        }
        if amount_0 > 0 {
            if amount_0 == fee_0 {
                amount_0 -= 1;
            }
            fee_0 -= amount_0;
            safe_transfer(token_0, recipient, amount_0);
        }
        if amount_1 > 0 {
            if amount_1 == fee_1 {
                amount_1 -= 1;
            }
            fee_1 -= amount_1;
            safe_transfer(token_1, recipient, amount_1);
        }
        self._emit_collect_protocol_event(caller, recipient, amount_0, amount_1);
        Ok((amount_0, amount_1))
    }

    fn protocol_fees(&self) -> Result<(Balance, Balance), PoolError> {
        let fee_0 = self.data::<data::Data>().fee0;
        let fee_1 = self.data::<data::Data>().fee1;
        Ok((fee_0, fee_1))
    }

    fn get_token_0(&self) -> AccountId {
        self.data::<data::Data>().token_0
    }

    fn get_token_1(&self) -> AccountId {
        self.data::<data::Data>().token_1
    }

    fn get_fee(&self) -> u32 {
        self.data::<data::Data>().fee
    }

    fn get_tick_spacing(&self) -> i32 {
        self.data::<data::Data>().tick_spacing
    }

    fn get_max_liquidity_per_tick(&self) -> u128 {
        self.data::<data::Data>().max_liquidity_per_tick
    }

    // fn get_slot0(&self) -> Slot{
    //     self.data::<data::Data>().slot0
    // }

    fn get_fee_growth_global_0x128(&self) -> u128 {
        self.data::<data::Data>().fee_growth_global_0x128
    }

    fn get_fee_growth_global_1x128(&self) -> u128 {
        self.data::<data::Data>().fee_growth_global_1x128
    }

    // fn get_protocol_fees(&self) -> ProtocolFees{
    //     self.data::<data::Data>().protocol_fees
    // }

    fn get_liquidity(&self) -> u128 {
        self.data::<data::Data>().liquidity
    }

    fn get_tick_bitmap(&self, entry: i32) -> Option<u128> {
        self.data::<data::Data>().tick_bitmap.get(&entry)
    }

    // fn get_tick_spacing(&self) -> i32 {
    //     self.data::<data::Data>().tick_spacing
    // }

    // fn get_max_liquidity_per_tick(&self) -> u123 {
    //     self.data::<data::Data>().max_liquidity_per_tick
    // }

    // fn get_slot_0(&self) -> Slot {
    //     self.data::<data::Data>().slot_0
    // }

    // fn get_fee_growth_global_0x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_0x128
    // }

    // fn get_fee_growth_global_1x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_1x128
    // }

    // fn get_liquidity(&self) -> u128 {
    //     self.data::<data::Data>().liquidity
    // }

    // fn mint(&mut self, recipient: AccountId, tick_lower: i32, tick_upper: i32, amount: u128, data:u128) -> Result<Balance, PoolError>{
    //     if amount < 1{
    //         return Err(PoolError::ZeroAmmount)
    //     }
    // }

    // fn _update_position(&self, onwer: AccountId, tick_lower: i32, tick_upper: i32, liquidity_delta: i128, tick: i32) -> Result<Balance, PoolError>{
    //     let position = self.get_position(owner,tick_lower, tick_upper);
    //     let flipped_lower = false;
    //     let flipped_upper = false;

    // }

    fn get_position(
        &self,
        owner: AccountId,
        tick_lower: i32,
        tick_upper: i32,
    ) -> Option<PositionInfo> {
        self.data::<data::Data>()
            .positions
            .get(&Self::env().hash_encoded::<Blake2x256, _>(&(owner, tick_lower, tick_upper)))
    }

    // fn _modify_position(&mut self, owner: AccountId, tick_lower: i32, tick_upper: i32, liqudiity_delta: i128) -> Result<Balance, PoolError>{
    //     if !self._check_ticks(tick_lower,tick_upper){
    //         return Err(PoolError::TickError)
    //     }

    // }

    fn _check_ticks(&self, tick_lower: i32, tick_upper: i32) -> bool {
        tick_lower < tick_upper && tick_lower >= i32::MIN && tick_upper <= i32::MAX
    }
}

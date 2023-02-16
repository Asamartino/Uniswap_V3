use crate::impls::pool::data_struct::*;
use crate::traits::factory::FactoryRef;
use ink_env::hash::Blake2x256;
use ink_prelude::vec::Vec;

pub use crate::{impls::pool::*, traits::pool::*};
use openbrush::{
    contracts::{ownable::*, psp22::*, reentrancy_guard::*, traits::psp22::PSP22Ref},
    traits::{AccountId, Balance, Storage, Timestamp},
};

pub struct  ModifyPositionParams {
    // the address that owns the position
    owner: AccountId,
    // the lower and upper tick of the position
    tick_lower : i32,
    tick_upper: i32,
    // any change in liquidity
    liquidity_delta: i128,
}

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
        amount0: Balance,
        amount1: Balance,
    );
}
impl<
        T: Storage<data::Data>, // + Storage<psp22::Data> + Storage<ownable::Data>
    > Pool for T
{
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
        amount0_requested: i128,
        amount1_requested: i128,
    ) -> Result<(u128, u128), PoolError> {
        Ok((0, 0))
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
        amount0: i128,
        amount1: i128,
        data: Vec<u8>,
    ) -> Result<(), PoolError> {
        Ok(())
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

    fn get_slot_0(&self) -> Slot {
        self.data::<data::Data>().slot_0
    }

    // fn get_fee_growth_global_0x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_0x128
    // }

    // fn get_fee_growth_global_1x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_1x128
    // }

    // fn get_liquidity(&self) -> u128 {
    //     self.data::<data::Data>().liquidity
    // }



    fn mint(&mut self, recipient: AccountId, tick_lower: i32, tick_upper: i32, amount: u128, data:u128) -> Result<(u128,u128), PoolError>{ // need to convert to u256
        if amount == 0 {
            return Err(PoolError::ZeroAmmount)
        }
        let amount_i128 = amount as i128;
     
        let ( _ ,  amount_0_int, amount_1_int) = self._modify_position( recipient, tick_lower, tick_upper, amount_i128)?;

        let amount_0 = amount_0_int as u128;
        let amount_1 = amount_1_int as u128;

        let contract = Self::env().account_id();

        let balance_0_before: u128;
        let balance_1_before: u128;

        // need to implement: IUniswapV3MintCallback(msg.sender).uniswapV3MintCallback(amount0, amount1, data)
        if amount_0 > 0 {
            balance_0_before = PSP22Ref::balance_of(&self.data::<data::Data>().token_0, contract);
            let balance_0_before_plus_amount_0 = balance_0_before.checked_add(amount_0).ok_or(PoolError::AddOverflowBalance0)?;
            if balance_0_before_plus_amount_0 > balance_0_before{
                return Err(PoolError::M0)
            }
        }
        if amount_1 > 0{
            balance_1_before = PSP22Ref::balance_of(&self.data::<data::Data>().token_1, contract);
            let balance_1_before_plus_amount_1 = balance_1_before.checked_add(amount_1).ok_or(PoolError::AddOverflowBalance1)?;
            if balance_1_before_plus_amount_1 > balance_1_before{
                return Err(PoolError::M1)
            }
        }
        Ok((amount_0,amount_1))
    }

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

    fn _modify_position(&mut self, owner: AccountId, tick_lower: i32, tick_upper: i32, liqudiity_delta: i128) -> Result<(PositionInfo,i128,i128), PoolError>{
        if !self._check_ticks(tick_lower,tick_upper){
            return Err(PoolError::TickError)
        }
        let slot0 = self.get_slot_0();
        // need to implement _updatePosition

        let amount_0: i128 = 0;
        let amount_1: i128 = 0;

        // if liquidity_delta != 0{
        //     if slot0.tick < tick_lower{

        //     }
        // }

        let position = PositionInfo { liquidity: 0, fee_growth_inside_0_last_x128: 0, fee_growth_inside_1_last_x128:0, tokens_owed_0: 0, token_owed_1: 0};
        Ok((position, amount_0, amount_1))
    }

    fn _check_ticks(&self, tick_lower: i32, tick_upper: i32) -> bool {
        tick_lower < tick_upper && tick_lower >= i32::MIN && tick_upper <= i32::MAX
    }
}

use crate::traits::factory::FactoryRef;

pub use crate::{
    impls::pool::*,
    traits::pool::*,
};
use openbrush::{
    contracts::{
        ownable::*,
        psp22::*,
        reentrancy_guard::*,
        traits::psp22::PSP22Ref,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        Timestamp,
    }
};

impl<T: Storage<data::Data> // + Storage<psp22::Data> + Storage<ownable::Data>
> Pool for T {
    fn initialize(
        &mut self,
        token_0: AccountId,
        token_1: AccountId,
        fee: u32
    ) -> Result<(), PoolError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        self.data::<data::Data>().fee = fee;
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
    
    fn get_tick_spacing(&self) -> i32{
        self.data::<data::Data>().tick_spacing
    }
    
    fn get_max_liquidity_per_tick(&self) -> u128{
        self.data::<data::Data>().max_liquidity_per_tick
    }
    
    // fn get_slot0(&self) -> Slot{
    //     self.data::<data::Data>().slot0
    // }
    
    fn get_fee_growth_global_0x128(&self) -> u128{
        self.data::<data::Data>().fee_growth_global_0x128
    }
    
    fn get_fee_growth_global_1x128(&self) -> u128{
        self.data::<data::Data>().fee_growth_global_1x128
    }
    
    // fn get_protocol_fees(&self) -> ProtocolFees{
    //     self.data::<data::Data>().protocol_fees
    // }
    
    fn get_liquidity(&self) -> u128{
        self.data::<data::Data>().liquidity
    }
    
    fn get_tick_bitmap(&self, entry: i32) -> Option<u128>{
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

    // fn get_position(&self, owner: AccountId, tick_lower: i32, tick_upper: i32)  -> Option<PositionInfo> {
    //     self.data::<data::Data>().positions[Self::env().hash_encoded::<Blake2x256,_>(&(owner, tick_lower, tick_upper))]
    // }
    
    // fn _modify_position(&mut self, owner: AccountId, tick_lower: i32, tick_upper: i32, liqudiity_delta: i128) -> Result<Balance, PoolError>{
    //     if !self._check_ticks(tick_lower,tick_upper){
    //         return Err(PoolError::TickError)
    //     }

    // }

    fn _check_ticks(&self, tick_lower: i32, tick_upper: i32) -> bool{
        tick_lower < tick_upper && tick_lower >= i32::MIN && tick_upper <= i32::MAX
    }

}
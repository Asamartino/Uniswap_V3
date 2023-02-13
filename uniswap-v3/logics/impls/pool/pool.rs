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

impl<T: Storage<data::Data> //+ Storage<psp22::Data> + Storage<ownable::Data>
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

}
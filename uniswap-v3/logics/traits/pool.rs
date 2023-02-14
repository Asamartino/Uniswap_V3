use openbrush::{
    contracts::{
        reentrancy_guard::*,
        traits::{
            ownable::*,
            psp22::PSP22Error,
        },
    },
    traits::{
        AccountId,
    }
};

#[openbrush::wrapper]
pub type PoolRef = dyn Pool;

#[openbrush::trait_definition]
pub trait Pool {
    #[ink(message)]
    fn initialize(&mut self, token_0: AccountId, token_1: AccountId, fee: u32) -> Result<(), PoolError>;
    
    #[ink(message)]
    fn get_token_0(&self) -> AccountId;
    
    #[ink(message)]
    fn get_token_1(&self) -> AccountId;
    
    #[ink(message)]
    fn get_fee(&self) -> u32;
    
    #[ink(message)]
    fn get_tick_spacing(&self) -> i32;
    
    #[ink(message)]
    fn get_max_liquidity_per_tick(&self) -> u128;
    
    // #[ink(message)]
    // fn get_slot0(&self) -> Slot;
    
    #[ink(message)]
    fn get_fee_growth_global_0x128(&self) -> u128;
    
    #[ink(message)]
    fn get_fee_growth_global_1x128(&self) -> u128;
    
    // #[ink(message)]
    // fn get_protocol_fees(&self) -> ProtocolFees;
    
    #[ink(message)]
    fn get_liquidity(&self) -> u128;
    
    #[ink(message)]
    fn get_tick_bitmap(&self, entry: i32) -> Option<u128>;

    // #[ink(message)]
    // fn mint(&mut self, recipient: AccountId, tick_lower: i32, tick_upper: i32, amount: u128, data:u128) -> Result<Balance, PoolError>;

    // #[ink(message)]
    // fn _modify_position(&mut self, owner: AccountId, tick_lower: i32, tick_upper: i32, liqudiity_delta: i128) -> Result<Balance, PoolError>;

    // #[ink(message)]
    // fn get_position(&self, owner: AccountId, tick_lower: i32, tick_upper: i32)  -> Option<PositionInfo> {}

    #[ink(message)]
    fn _check_ticks(&self,tick_lower: i32, tick_upper: i32) -> bool;

    // #[ink(message)]
    // fn _update_position(&self, onwer: AccountId, tick_lower: i32, tick_upper: i32, liquidity_delta: i128, tick: i32) -> Result<Balance, PoolError>;


}

#[derive(Debug,PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolError{
    ZeroAmmount,
    TickError,
}
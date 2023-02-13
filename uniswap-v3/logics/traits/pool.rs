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
    
    fn get_token_0(&self) -> AccountId;
    fn get_token_1(&self) -> AccountId;
    fn get_fee(&self) -> u32;
}

#[derive(Debug,PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolError{

}
use openbrush::{
    contracts::{
        traits::{
            // ownable::*,
            // psp22::PSP22Error,
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
}

#[derive(Debug,PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolError{

}
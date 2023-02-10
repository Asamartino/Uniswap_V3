#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            psp22::{
                Internal,
                *,
            },
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PoolContract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for PoolContract {}

    impl PoolContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }
    }
}
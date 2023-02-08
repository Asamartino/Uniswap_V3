#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use ink_lang::{
        codegen::{EmitEvent, Env},
        ToAccountId,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
        }
        traits::{Storage, ZERO_ADDRESS},
    }
    use pool_contract::pool::PoolrContractRef;
    use uniswap_v2::{impls::factory::*, traits::factory::*};

    #[ink(event)]
    pub struct PoolCreated {
        #[ink(topic)]
        pub token_0: AccountId,
        #[ink(topic)]
        pub token_1: AccountId,
        pub pool: AccountId,
        pub pool_len: u64,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {
        default fn _fee_amount_enabled(&self, fee: u24, tick_spacing: i24){
            EmitEvent::<FactoryContract>::emit_event(
                self.env(),
                FeeEnabled {
                    fee,
                    tick_spacing
                },
            )
        }
    

        default fn _instantiate_pool(&mut self, _salt_bytes: &[u32]) -> Result<AccountId, FactoryError> {
            let pool_hash = self.factory.pool_contract_code_hash;
            let pool = PoolContractRef::new()
                .endowment(0)
                .code_hash(pool_hash)
                .salt_bytes(&salt_bytes[..4])
                .instantiate()
                .map_err(|_| FactoryError::PoolInstantiationFailed)?;
            Ok(pool.to_account_id())
        }
    }
   
    impl Ownable for PoolContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(fee_to_setter: AccountId, pool_code_hash: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.factory.pool_contract_code_hash = pool_code_hash;
                instance.factory.fee_to_setter = fee_to_setter;
                instance.factory.fee_to = ZERO_ADDRESS.into();
            })
        }
    }
}


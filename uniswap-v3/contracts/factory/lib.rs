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
        contracts::ownable::*,
        traits::{Storage, ZERO_ADDRESS},
    };
    use pool_contract::pool::PoolrContractRef;
    use uniswap_v2::{impls::factory::*, traits::factory::*};

    #[ink(event)]
    pub struct PoolCreated {
        #[ink(topic)]
        pub token_0: AccountId,
        #[ink(topic)]
        pub token_1: AccountId,
        pub fee: u32,
        pub tick_spacing: i32,
        pub pool_contract: AccountId,
    }

    #[ink(event)]
    pub struct OwnerChanged {
        #[ink(topic)]
        pub original_owner: AccountId,
        #[ink(topic)]
        pub new_owner: AccountId,
    }

    #[ink(event)]
    pub struct FeeAmountEnabled {
        #[ink(topic)]
        pub fee: u32,
        #[ink(topic)]
        pub tick_spacing: i32,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: data::Data,
    }

    impl Factory for FactoryContract {
        default fn _fee_amount_enabled(&self, fee: u24, tick_spacing: i24) {
            EmitEvent::<FactoryContract>::emit_event(self.env(), FeeEnabled { fee, tick_spacing })
        }

        fn _emit_create_pool_event(
            &self,
            token_0: AccountId,
            token_1: AccountId,
            fee: u32,
            tick_spacing: i32,
            pool_contract: AccountId,
        ) {
            EmitEvent::<FactoryContract>::emit_event(
                self.env(),
                PoolCreated {
                    token_0,
                    token_1,
                    fee,
                    tick_spacing,
                    pool_contract,
                },
            )
        }

        fn _emit_owner_changed_event(&self, original_owner: AccountId, new_owner: AccountId) {
            EmitEvent::<FactoryContract>::emit_event(
                self.env(),
                OwnerChanged {
                    original_owner,
                    new_owner,
                },
            )
        }

        fn _emit_fee_amount_enabled_event(&self, _fee: u32, _tick_spacing: i32) {
            EmitEvent::<FactoryContract>::emit_event(
                self.env(),
                FeeAmountEnabled { fee, tick_spacing },
            )
        }

        default fn _instantiate_pool(
            &mut self,
            _salt_bytes: &[u8],
        ) -> Result<AccountId, FactoryError> {
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
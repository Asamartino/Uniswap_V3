use crate::traits::pool::PoolRef;

pub use crate::{impls::factory::*, traits::factory::*};

use ink_env::hash::Blake2x256;

use openbrush::{
    contracts::ownable::*,
    modifiers, modifier_definition,
    traits::{AccountId, Storage, ZERO_ADDRESS},
};

impl<T: Storage<data::Data> + Storage<ownable::Data>> Factory for T {
    fn create_pool(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        fee: u32,
    ) -> Result<AccountId, FactoryError> {
        if token_a == token_b {
            return Err(FactoryError::IdenticalAddresses);
        }
        let token_pair = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        if token_pair.0 == ZERO_ADDRESS.into() {
            return Err(FactoryError::ZeroAddress);
        }
        let tick_spacing = self
            .get_fee_amount_tick_spacing(fee)
            .ok_or(FactoryError::NoTickSpacing)?;
        if tick_spacing == 0 {
            return Err(FactoryError::ZeroTickSpacing);
        }
        ///////////////////////////////////////////////////////////////////////////////////////
        // not sure how to translate UniswapV3PoolDeployer.sol and deploy() to ink!
        // will use a similar solution than the one use for UniswapV2Factory translation to ink!

        let salt = Self::env().hash_encoded::<Blake2x256, _>(&token_pair);
        let pool_contract = self._instantiate_pool(salt.as_ref())?;

        /////////////////////////////                             uncomment once pool contract is coded                            ///////////////////////////////////////////////////////
        // PoolRef::initialize(&pool_contract, token_pair.0, token_pair.1, fee)?;
        /////////////////////////////                                                                                              ///////////////////////////////////////////////////////

        //////////////////////////////////////////////////////////////////////////////////////

        self.data::<data::Data>()
            .get_pool
            .insert(&(token_pair.0, token_pair.1, fee), &pool_contract);
        self.data::<data::Data>()
            .get_pool
            .insert(&(token_pair.1, token_pair.0, fee), &pool_contract);

        self._emit_create_pool_event(token_a, token_b, fee, tick_spacing, pool_contract);
        Ok(pool_contract)
    }

    default fn _instantiate_pool(&mut self, _salt_bytes: &[u8]) -> Result<AccountId, FactoryError> {
        unimplemented!()
    }

    fn get_pool(&self, token_a: AccountId, token_b: AccountId, fee: u32) -> Option<AccountId> {
        self.data::<data::Data>()
            .get_pool
            .get(&(token_a, token_b, fee))
    }

    #[modifiers(only_owner)]
    fn set_owner(&mut self, new_owner: AccountId) -> Result<(), FactoryError> {
        let previous_owner = self.data::<data::Data>().owner;
        self._emit_owner_changed_event(previous_owner, new_owner);
        self.data::<data::Data>().owner = new_owner;
        Ok(())
    }

    #[modifiers(only_owner)]
    fn enable_fee_amount(&mut self, fee: u32, tick_spacing: i32) -> Result<(), FactoryError> {
        if fee < 1000000 {
            return Err(FactoryError::FeeTooBig);
        }

        let tick_spacing_range = 1..16384;
        if !tick_spacing_range.contains(&tick_spacing) {
            return Err(FactoryError::TickSpacingOutOfBonds);
        }

        if self.get_fee_amount_tick_spacing(fee).unwrap() != 0 {
            return Err(FactoryError::NonZeroTickSpacing);
        }
        self.data::<data::Data>().fee_amount_tick_spacing.insert(&fee, &tick_spacing);
        self._emit_fee_amount_enabled_event(fee, tick_spacing);
        Ok(())
    }

    fn get_fee_amount_tick_spacing(&self, fee: u32) -> Option<i32> {
        self.data::<data::Data>().fee_amount_tick_spacing.get(&fee)
    }

    default fn _emit_fee_amount_enabled_event(&self, _fee: u32, _tick_spacing: i32) {}

    default fn _emit_create_pool_event(
        &self,
        _token_a: AccountId,
        _token_b: AccountId,
        _fee: u32,
        _tick_spacing: i32,
        _pool: AccountId,
    ) {
    }

    default fn _emit_owner_changed_event(&self, _original_owner: AccountId, _new_owner: AccountId) {
    }
}

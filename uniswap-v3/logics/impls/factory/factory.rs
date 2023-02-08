use crate::traits::factory::FactoryRef;

pub use crate::{impls::factory::*, traits::factory::*};

use ink_env::hash::Blake2x256;

use openbrush::{
    contracts::ownable::*,
    modifiers,
    traits::{AccountId, Storage, ZERO_ADDRESS},
};

impl<T: Storage<data::Data>> Factory for T {
    #[modifiers(only_owner)]
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
        let tick_spacing = self.fee_amount_tick_spacing(fee)?;
        if tick_spacing == 0 {
            return Err(FactoryError::ZeroTickSpacing);
        }
        ///////////////////////////////////////////////////////////////////////////////////////
        // not sure how to translate UniswapV3PoolDeployer.sol and deploy() to ink!
        // will use a similar solution than UniswapV2Factory translation to ink!
        let salt = Self::env().hash_encoded::<Blake2x256, _>(&token_pair, fee);
        let pool_contract = self._intantiate_pool(salt.as_ref())?;
        FactoryRef::initialize(&pool_contract, token_pair.0, token_pair.1, fee)?;

        //////////////////////////////////////////////////////////////////////////////////////

        self.data::<data::Data>()
            .get_pool
            .insert(&(token_pair.0, token_pair.1, fee), &pool_contract);
        self.data::<data::Data>()
            .get_pool
            .insert(&(token_pair.1, token_pair.0, fee), &pool_contract);

        self._pool_created(&self, token_a, token_b, fee, tick_spacing, pool_contract);
        Ok(pool_contract)
    }
    ////////////// ???????????????????????????????????

    fn enable_fee_amount(&self, fee: u32, tick_spacing: i32) -> Result<(), FactoryError> {
        if fee < 1000000 {
            return Err(FactoryError::FeeTooBig);
        }

        let tick_spacing_range = 1..16384;
        if !tick_spacing_range.contains(&tick_spacing) {
            return Err(FactoryError::TickSpacingOutOfBonds);
        }

        if tick_spacing == 0 {
            return Err(FactoryError::ZeroTickSpacing);
        }
        // tick spacing is capped at 16384 to prevent the situation where tickSpacing is so large that
        // TickBitmap#nextInitializedTickWithinOneWord overflows int24 container from a valid tick
        // 16384 ticks represents a >5x price change with ticks of 1 bips
        *self.fee_amount_tick_spacing.entry(fee).or_insert(0) = tick_spacing;
        self._fee_amount_enabled(fee, tick_spacing);
        Ok(())
    }

    fn fee_amount_tick_spacing(&self, fee: u32) -> Option<i32> {
        self.data::<data::Data>().fee_amount_tick_spacing.get(&fee)
    }

    default fn _fee_amount_enabled(&self, _fee: u32, _tick_spacing: i32) {}

    default fn _instantiate_pool(
        &mut self,
        _salt_bytes: &[u32],
    ) -> Result<AccountId, FactoryError> {
        unimplemented!()
    }

    fn get_pool(&self, token_a: AccountId, token_b: AccountId, fee: u32) -> Option<AccountId> {
        self.data::<data::Data>()
            .get_pool
            .get(&(token_a, token_b, fee))
    }
}

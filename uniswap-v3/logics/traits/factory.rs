use openbrush::{contracts::traits::ownable::*, traits::AccountId};

#[openbrush::wrapper]
pub type FactoryRef = dyn Factory;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn create_pool(
        &mut self,
        token_a: AccountId,
        token_b: AccountId,
        fee: u32,
    ) -> Result<AccountId, FactoryError>;

    fn _instantiate_pool(&mut self, salt_bytes: &[u8]) -> Result<AccountId, FactoryError>;

    #[ink(message)]
    fn set_owner(&mut self, new_owner: AccountId) -> Result<(), FactoryError>;

    #[ink(message)]
    fn enable_fee_amount(&mut self, fee: u32, tick_spacing: i32) -> Result<(), FactoryError>;

    #[ink(message)]
    fn get_fee_amount_tick_spacing(&self, fee: u32) -> Option<i32>;

    #[ink(message)]
    fn get_pool(&self, token_a: AccountId, token_b: AccountId, fee: u32) -> Option<AccountId>;

    // Events
    fn _emit_owner_changed_event(&self, _original_owner: AccountId, _new_owner: AccountId) {}

    fn _emit_fee_amount_enabled_event(&self, _fee: u32, _tick_spacing: i32);
    fn _emit_create_pool_event(
        &self,
        _token_a: AccountId,
        _token_b: AccountId,
        _fee: u32,
        _tick_spacing: i32,
        _pool: AccountId,
    );
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FactoryError {
    IdenticalAddresses,
    ZeroAddress,
    ZeroTickSpacing,
    NonZeroTickSpacing,
    FeeTooBig,
    TickSpacingOutOfBonds,
    NoTickSpacing,
    PoolInstantiationFailed,
    OwnableError(OwnableError),
}

impl From<OwnableError> for FactoryError {
    fn from(error: OwnableError) -> Self {
        FactoryError::OwnableError(error)
    }
}

use ink_env::Hash;
use openbrush::{storage::Mapping, traits::AccountId};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub owner: AccountId,
    pub fee_amount_tick_spacing: Mapping<u32, i32>,
    pub get_pool: Mapping<AccountId, Mapping<(AccountId, u32), AccountId>>,
    pub pool_contract_code_hash: Hash,
}

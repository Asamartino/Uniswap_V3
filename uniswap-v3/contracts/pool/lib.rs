#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use ink_lang::codegen::{EmitEvent, Env};
    use ink_prelude::vec::Vec;

    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::{
                Internal, 
                *
            },
            reentrancy_guard,
        },
        traits::Storage,
    };
    use uniswap_v3::{
        impls::pool::*,
        traits::pool::*,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PoolContract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pool: data::Data,
    }
    #[ink(event]
        pub struct Mint {
            #[ink(topic)]
            sender: AccountId,
            owner: AccountId,
            tick_lower: i32,
            tick_upper: i32,
            #[ink(topic)]
            amount: Balance,
            amount0: Balance,
            amount1: Balance,
        }
    #[ink(event)]
    pub struct Burn {
        owner: AccountId,
        #[ink(topic)]
        tick_lower: i32,
        tick_upper: i32,
        #[ink(topic)]
        amount: Balance,
        amount0: Balance,
        amount1: Balance,
    }
    #[ink(event)]
    pub struct Collect {
        owner: AccountId,
        #[ink(topic)]
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0: Balance,
        amount1: Balance,
    }
    #[ink(event)]
    pub struct Swap {
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        sqrt_price_x96: u128,
        liquidity: u128,
        tick: i32,
    }
    #[ink(event)]
    pub struct Flash{
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        paid0: Balance,
        paid1: Balance,
    }
    #[ink(event)]
    pub struct IncreaseObservationCardinalityNext{
        observation_cardinality_next_old: u16,
        observation_cardinality_next_new: u16,
    }
    #[ink(event)]
    pub struct SetFeeProtocol{
        fee_protocol0_old: u8,
        fee_protocol1_old: u8,
        fee_protocol0_new: u8,
        fee_protocol1_new: u8,
    }
    #[ink(event)]
    pub struct CollectProtocol{
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
    }
    // #[ink(event)]
    // pub struct Transfer {
    //     #[ink(topic)]
    //     from: Option<AccountId>,
    //     #[ink(topic)]
    //     to: Option<AccountId>,
    //     value: Balance,
    // }

    // #[ink(event)]
    // pub struct Approval {
    //     #[ink(topic)]
    //     owner: AccountId,
    //     #[ink(topic)]
    //     spender: AccountId,
    //     value: Balance,
    // }

    impl PSP22 for PoolContract {
            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                let caller = self.env().caller();
                let allowance = self._allowance(&from, &caller);
        
                // In uniswapv2 max allowance never decrease
                if allowance != u128::MAX {
                    if allowance < value {
                        return Err(PSP22Error::InsufficientAllowance)
                    }
        
                    self._approve_from_to(from, caller, allowance - value)?;
                }
                self._transfer_from_to(from, to, value, data)?;
                Ok(())
            }
        }
    

    impl PoolContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }
    }

    // impl Pool for PoolContract {
        // fn _emit_transfer_event(
        //     &self,
        //     from: Option<AccountId>,
        //     to: Option<AccountId>,
        //     amount: Balance,
        // ) {
        //     self.env().emit_event(Transfer {
        //         from,
        //         to,
        //         value: amount,
        //     });
        // }

        // fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
        //     self.env().emit_event(Approval {
        //         owner,
        //         spender,
        //         value: amount,
        //     });
        // }

        // fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        //     let mut new_balance = self._balance_of(&account);
        //     new_balance += amount;
        //     self.psp22.balances.insert(&account, &new_balance);
        //     self.psp22.supply += amount;
        //     self._emit_transfer_event(None, Some(account), amount);
        //     Ok(())
        // }

        // fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        //     let mut from_balance = self._balance_of(&account);

        //     if from_balance < amount {
        //         return Err(PSP22Error::InsufficientBalance);
        //     }

        //     from_balance -= amount;
        //     self.psp22.balances.insert(&account, &from_balance);
        //     self.psp22.supply -= amount;
        //     self._emit_transfer_event(Some(account), None, amount);
        //     Ok(())
        // }

        // fn _approve_from_to(
        //     &mut self,
        //     owner: AccountId,
        //     spender: AccountId,
        //     amount: Balance,
        // ) -> Result<(), PSP22Error> {
        //     self.psp22.allowances.insert(&(&owner, &spender), &amount);
        //     self._emit_approval_event(owner, spender, amount);
        //     Ok(())
        // }

        // fn _transfer_from_to(
        //     &mut self,
        //     from: AccountId,
        //     to: AccountId,
        //     amount: Balance,
        //     _data: Vec<u8>,
        // ) -> Result<(), PSP22Error> {
        //     let from_balance = self._balance_of(&from);

        //     if from_balance < amount {
        //         return Err(PSP22Error::InsufficientBalance);
        //     }

        //     self.psp22.balances.insert(&from, &(from_balance - amount));
        //     let to_balance = self._balance_of(&to);
        //     self.psp22.balances.insert(&to, &(to_balance + amount));

        //     self._emit_transfer_event(Some(from), Some(to), amount);
        //     Ok(())
        // }
    }      

}
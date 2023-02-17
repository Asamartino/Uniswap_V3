use crate::impls::pool::data_struct::*;
use crate::traits::factory::FactoryRef;
use ink_env::hash::Blake2x256;
use ink_prelude::vec::Vec;
use primitive_types::U256;

use crate::helpers::liquidity_helper::liquidity_num::{
    MAX_SQRT_RATIO, MAX_TICK, MIN_SQRT_RATIO, MIN_TICK,
};
use crate::helpers::liquidity_helper::{
    _compute_swap_step, add_delta, cross, get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio, mul_div,
    next_initialized_tick_within_oneword, observe_single, tick_bitmap, write,
};
use crate::{ensure, helpers::transfer_helper::safe_transfer};
// use crate::helpers::liquidity_helper::{_compute_swap_step, get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio};
// use crate::helpers::liquidity_helper::liquidity_num;
// use crate::{ensure, helpers::transfer_helper::safe_transfer};
use crate::{impls::pool::*, traits::pool::*};
use openbrush::{
    contracts::{ownable::*, psp22::*, reentrancy_guard::*, traits::psp22::PSP22Ref},
    traits::{AccountId, Balance, Storage, Timestamp},
};

pub struct ModifyPositionParams {
    // the address that owns the position
    owner: AccountId,
    // the lower and upper tick of the position
    tick_lower: i32,
    tick_upper: i32,
    // any change in liquidity
    liquidity_delta: i128,
}

pub trait Internal {
    fn _emit_initialize_event(&self, sqrt_price_x96: u128, tick: i32);
    fn _emit_mint_event(
        &self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount: Balance,
    );
    fn _emit_collect_event(
        &self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Balance,
        amount1_requested: Balance,
    );

    fn _emit_swap_event(
        &self,
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        sqrt_price_x96: u128,
        liquidity: u128,
        tick: i32,
    );
    fn _emit_flash_event(
        &self,
        sender: AccountId,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        paid0: u128,
        paid1: u128,
    );
    fn _emit_set_fee_protocol_event(
        &self,
        fee_protocol0_old: u8,
        fee_protocol1_old: u8,
        fee_protocol0_new: u8,
        fee_protocol1_new: u8,
    );
    fn _emit_collect_protocol_event(
        &self,
        sender: AccountId,
        recipient: AccountId,
        amount0_requested: Balance,
        amount1_requested: Balance,
    );
}
impl<T: Storage<data::Data> + Internal> Pool for T {
    fn initialize(
        &mut self,
        token_0: AccountId,
        token_1: AccountId,
        fee: u32,
    ) -> Result<(), PoolError> {
        self.data::<data::Data>().token_0 = token_0;
        self.data::<data::Data>().token_1 = token_1;
        self.data::<data::Data>().fee = fee;
        Ok(())
    }
    fn collect(
        &mut self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError> {
        let mut amount_0: Balance;
        let mut amount_1: Balance;
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        let caller = Self::env().caller();
        let mut owned_amount_0 = 0;
        let mut owned_amount_1 = 0;
        let position_info = self.get_position(caller, tick_lower, tick_upper);
        if let Some(info) = position_info {
            owned_amount_0 = info.tokens_owed_0;
            owned_amount_1 = info.tokens_owed_1;
        }

        if amount0_requested > owned_amount_0 {
            amount_0 = owned_amount_0;
        } else {
            amount_0 = amount0_requested;
        }
        if amount1_requested > owned_amount_1 {
            amount_1 = owned_amount_1;
        } else {
            amount_1 = amount1_requested;
        }
        if amount_0 > 0 {
            owned_amount_0 -= amount_0;
            safe_transfer(token_0, recipient, amount_0);
        } else {
            // ensure!(amount_0 == 0, PoolError);
        }
        if amount_1 > 0 {
            owned_amount_1 -= amount_1;
            safe_transfer(token_1, recipient, amount_1);
        } else {
            // ensure!(amount_1 == 0, PoolError);
        }
        self._emit_collect_event(recipient, tick_lower, tick_upper, amount_0, amount_1);
        Ok((amount_0, amount_1))
    }

    fn swap(
        &mut self,
        recipient: AccountId,
        zero_for_one: bool,
        amount_specified: i128,
        sqrt_price_limit_x96: u128,
    ) -> Result<(Balance, Balance), PoolError> {
        let mut amount_0: Balance;
        let mut amount_1: Balance;
        let min_tick = MIN_TICK;
        let max_tick = MAX_TICK;
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        let min_sqrt_ratio = MIN_SQRT_RATIO;
        let max_sqrt_ratio = MAX_SQRT_RATIO;
        ensure!(amount_specified != 0, PoolError::AmountSpecifiedIsZero);
        let slot0_start = self.data::<data::Data>().slot_0;
        ensure!(slot0_start.unlocked, PoolError::PoolIsLocked);
        let caller = Self::env().caller();
        // TODO: implement tick_math
        if zero_for_one {
            ensure!(
                sqrt_price_limit_x96 < slot0_start.sqrt_price_x96
                    && sqrt_price_limit_x96 > min_sqrt_ratio,
                PoolError::SqrtPriceLimitX96IsInvalid
            );
        } else {
            ensure!(
                // TODO: FIX TYPE ERROR MAX_SQRT_RATIO
                sqrt_price_limit_x96 > slot0_start.sqrt_price_x96
                    && sqrt_price_limit_x96 < max_sqrt_ratio,
                PoolError::SqrtPriceLimitX96IsInvalid
            );
        }
        slot0_start.unlocked = false;
        // refer https://github.com/Uniswap/v3-core/blob/05c10bf6d547d6121622ac51c457f93775e1df09/contracts/UniswapV3Pool.sol#L622
        let mut cache = SwapCache {
            liquidity_start: self.data::<data::Data>().liquidity,
            block_timestamp: 0,
            fee_protocol: 0,
            seconds_per_liquidity_cumulative_x128: 0,
            tick_cumulative: 0,
            computed_latest_observations: false,
        };

        let exact_input = amount_specified > 0;
        let state = SwapState {
            amount_specified_remaining: amount_specified,
            amount_calculated: 0,
            sqrt_price_x96: slot0_start.sqrt_price_x96,
            tick: slot0_start.tick,
            fee_growth_global_x128: if zero_for_one {
                self.data::<data::Data>().fee_growth_global_0x128
            } else {
                self.data::<data::Data>().fee_growth_global_1x128
            },
            protocol_fee: 0,
            liquidity: cache.liquidity_start,
        };
        // continue swapping as long as we haven't used the entire input/output and haven't reached the price limit
        while state.amount_specified_remaining != 0 && state.sqrt_price_x96 != sqrt_price_limit_x96
        {
            let step = self.data::<data::Data>().step_computations;
            step.sqrt_price_start_x96 = state.sqrt_price_x96;
            // TODO: implement tick_bitmap
            // TODO: implement next_initialized_tick_within_one_word
            (step.sqrt_price_next_x96, step.initialized) = next_initialized_tick_within_oneword(
                state.tick,
                self.data::<data::Data>().tick_spacing,
                zero_for_one,
            );
            // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
            if step.tick_next < min_tick {
                step.tick_next = min_tick;
            } else if step.tick_next > max_tick {
                step.tick_next = max_tick;
            }
            //get the price for the next tick
            step.sqrt_price_next_x96 = get_sqrt_ratio_at_tick(step.tick_next);

            // compute values to swap to the target tick, price limit, or point where input/output amount is exhausted
            //  TODO implement _compute_swap_step
            (
                state.sqrt_price_x96,
                step.amount_in,
                step.amount_out,
                step.fee_amount,
            ) = if zero_for_one {
                if step.sqrt_price_next_x96 < sqrt_price_limit_x96 {
                    _compute_swap_step(
                        state.sqrt_price_x96,
                        sqrt_price_limit_x96,
                        state.liquidity,
                        state.amount_specified_remaining,
                        self.data::<data::Data>().fee,
                    )
                } else {
                    _compute_swap_step(
                        state.sqrt_price_x96,
                        step.sqrt_price_next_x96,
                        state.liquidity,
                        state.amount_specified_remaining,
                        self.data::<data::Data>().fee,
                    )
                }
            } else {
                if step.sqrt_price_next_x96 > sqrt_price_limit_x96 {
                    _compute_swap_step(
                        state.sqrt_price_x96,
                        sqrt_price_limit_x96,
                        state.liquidity,
                        state.amount_specified_remaining,
                        self.data::<data::Data>().fee,
                    )
                } else {
                    _compute_swap_step(
                        state.sqrt_price_x96,
                        step.sqrt_price_next_x96,
                        state.liquidity,
                        state.amount_specified_remaining,
                        self.data::<data::Data>().fee,
                    )
                }
            };
            // TODO implement converter(openzeppelin)
            // refer https://github.com/Uniswap/v3-core/blob/05c10bf6d547d6121622ac51c457f93775e1df09/contracts/UniswapV3Pool.sol#L676
            if exact_input {
                state.amount_specified_remaining -= (step.amount_in / step.fee_amount) as i128;
                state.amount_calculated = state.amount_calculated;
            } else {
                state.amount_specified_remaining += step.amount_out as i128;
                state.amount_calculated = state.amount_calculated;
            }
            // if the protocol fee is on, calculate how much is owed, decrement feeAmount, and increment protocolFee
            if cache.fee_protocol > 0 {
                let delta = step.fee_amount / cache.fee_protocol as u128;
                step.fee_amount -= delta;
                state.protocol_fee += delta;
            }
            // update global fee tracker
            // TODO: implement full_math
            // TODO: implement fixed_point_128
            if state.liquidity > 0 {
                state.fee_growth_global_x128 +=
                    mul_div(step.fee_amount, u128::MAX, state.liquidity);
            }
            // shift tick if we reached the next price
            // TODO: implement observe_single
            // TODO: implement cross
            if state.sqrt_price_x96 == step.sqrt_price_next_x96 {
                // check for the placeholder value, which we replace with the actual value the first time the swap
                // crosses an initialized tick
                if !cache.computed_latest_observations {
                    (
                        cache.tick_cumulative,
                        cache.seconds_per_liquidity_cumulative_x128,
                    ) = observe_single(
                        cache.block_timestamp,
                        0,
                        slot0_start.tick,
                        slot0_start.observation_index,
                        cache.liquidity_start,
                        slot0_start.observation_cardinality,
                    );
                    cache.computed_latest_observations = true;
                }
                let liquidity_net: i128 = if zero_for_one {
                    cross(
                        step.tick_next,
                        state.fee_growth_global_x128,
                        self.data::<data::Data>().fee_growth_global_1x128,
                        cache.seconds_per_liquidity_cumulative_x128,
                        cache.tick_cumulative,
                        cache.block_timestamp,
                    )
                } else {
                    cross(
                        step.tick_next,
                        self.data::<data::Data>().fee_growth_global_0x128,
                        state.fee_growth_global_x128,
                        cache.seconds_per_liquidity_cumulative_x128,
                        cache.tick_cumulative,
                        cache.block_timestamp,
                    )
                };
                // if we're moving leftward, we interpret liquidityNet as the opposite sign
                // safe because liquidityNet cannot be type(int128).min
                if zero_for_one {
                    liquidity_net = -liquidity_net
                };
                //TODO implement add_delta
                state.liquidity = add_delta(state.liquidity, liquidity_net);
            } else if state.sqrt_price_x96 != step.sqrt_price_start_x96 {
                state.tick = get_tick_at_sqrt_ratio(state.sqrt_price_x96);
            }
        }
        // update tick and write an oracle entry if the tick change
        if state.tick != slot0_start.tick {
            let mut observation_index: u16;
            let mut observation_cardinality: u16;
            (observation_index, observation_cardinality) = write(
                slot0_start.observation_index,
                cache.block_timestamp,
                slot0_start.tick,
                cache.liquidity_start,
                slot0_start.observation_cardinality,
                slot0_start.observation_cardinality_next,
            );
            (
                self.data::<data::Data>().slot_0.sqrt_price_x96,
                self.data::<data::Data>().slot_0.tick,
                self.data::<data::Data>().slot_0.observation_index,
                self.data::<data::Data>().slot_0.observation_cardinality,
            ) = (
                state.sqrt_price_x96,
                state.tick,
                observation_index,
                observation_cardinality,
            );
        } else {
            // otherwise, just update the sqrt price
            self.data::<data::Data>().slot_0.sqrt_price_x96 = state.sqrt_price_x96;
        }
        // update liquidity if it changed
        if cache.liquidity_start != state.liquidity {
            self.data::<data::Data>().liquidity = state.liquidity;
        }
        // update fee growth global and, if necessary, protocol fees
        // overflow is acceptable, protocol has to withdraw before it hits type(uint128).max fees
        if zero_for_one {
            self.data::<data::Data>().fee_growth_global_0x128 = state.fee_growth_global_x128;
            if state.protocol_fee > 0 {
                // TODO data::Data::fee0 is Balance type
                self.data::<data::Data>().fee0 += state.protocol_fee;
            }
        } else {
            self.data::<data::Data>().fee_growth_global_1x128 = state.fee_growth_global_x128;
            if state.protocol_fee > 0 {
                //TODO data::Data::fee1 is Balance type
                self.data::<data::Data>().fee1 += state.protocol_fee;
            }
        }
        let amount_specified_i128 = amount_specified as i128;
        let amount_specified_remaining_i128 = state.amount_specified_remaining as i128;

        if zero_for_one == exact_input {
            let amount_1_i128 = -amount_specified_remaining_i128;
            amount_0 = (amount_specified_i128 - state.amount_specified_remaining) as Balance;
            amount_1 = state.amount_calculated as Balance;
        } else {
            let amount_0_i128 = -amount_specified_remaining_i128;
            amount_0 = state.amount_calculated as Balance;
            amount_1 = (amount_specified_i128 - state.amount_specified_remaining) as Balance;
        }

        // do the transfers and collect payment
        if zero_for_one {
            if amount_1 < 0 {
                safe_transfer(token_1, recipient, amount_1);
            }
        } else {
            if amount_0 < 0 {
                safe_transfer(token_0, recipient, amount_0);
            }
        }

        self._emit_swap_event(
            caller,
            recipient,
            amount_0,
            amount_1,
            state.sqrt_price_x96,
            state.liquidity,
            state.tick,
        );
        self.data::<data::Data>().slot_0.unlocked = true;
        Ok((amount_0, amount_1))
    }

    fn flash(
        &mut self,
        recipient: AccountId,
        amount0: u128,
        amount1: u128,
        data: Vec<u8>,
    ) -> Result<(), PoolError> {
        Ok(())
    }

    // ProtocolOwnerActions
    // TODO: add fee protocol
    fn collect_protocol(
        &mut self,
        sender: AccountId,
        recipient: AccountId,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError> {
        let caller = Self::env().caller();
        let mut amount_0: Balance;
        let mut amount_1: Balance;
        let mut fee_0 = self.data::<data::Data>().fee0;
        let mut fee_1 = self.data::<data::Data>().fee1;
        let token_0 = self.data::<data::Data>().token_0;
        let token_1 = self.data::<data::Data>().token_1;
        if amount0_requested > fee_0 {
            amount_0 = fee_0;
        } else {
            amount_0 = amount0_requested;
        }
        if amount1_requested > fee_1 {
            amount_1 = fee_1;
        } else {
            amount_1 = amount1_requested;
        }
        if amount_0 > 0 {
            if amount_0 == fee_0 {
                amount_0 -= 1;
            }
            fee_0 -= amount_0;
            safe_transfer(token_0, recipient, amount_0);
        }
        if amount_1 > 0 {
            if amount_1 == fee_1 {
                amount_1 -= 1;
            }
            fee_1 -= amount_1;
            safe_transfer(token_1, recipient, amount_1);
        }
        self._emit_collect_protocol_event(caller, recipient, amount_0, amount_1);
        Ok((amount_0, amount_1))
    }

    fn protocol_fees(&self) -> Result<(Balance, Balance), PoolError> {
        let fee_0 = self.data::<data::Data>().fee0;
        let fee_1 = self.data::<data::Data>().fee1;
        Ok((fee_0, fee_1))
    }

    fn get_token_0(&self) -> AccountId {
        self.data::<data::Data>().token_0
    }

    fn get_token_1(&self) -> AccountId {
        self.data::<data::Data>().token_1
    }

    fn get_fee(&self) -> u32 {
        self.data::<data::Data>().fee
    }

    fn get_tick_spacing(&self) -> i32 {
        self.data::<data::Data>().tick_spacing
    }

    fn get_max_liquidity_per_tick(&self) -> u128 {
        self.data::<data::Data>().max_liquidity_per_tick
    }

    // fn get_slot0(&self) -> Slot{
    //     self.data::<data::Data>().slot0
    // }

    fn get_fee_growth_global_0x128(&self) -> u128 {
        self.data::<data::Data>().fee_growth_global_0x128
    }

    fn get_fee_growth_global_1x128(&self) -> u128 {
        self.data::<data::Data>().fee_growth_global_1x128
    }

    // fn get_protocol_fees(&self) -> ProtocolFees{
    //     self.data::<data::Data>().protocol_fees
    // }

    fn get_liquidity(&self) -> u128 {
        self.data::<data::Data>().liquidity
    }

    fn get_tick_bitmap(&self, entry: i32) -> Option<u128> {
        self.data::<data::Data>().tick_bitmap.get(&entry)
    }

    // fn get_tick_spacing(&self) -> i32 {
    //     self.data::<data::Data>().tick_spacing
    // }

    // fn get_max_liquidity_per_tick(&self) -> u128 {
    //     self.data::<data::Data>().max_liquidity_per_tick
    // }

    fn get_slot_0(&self) -> Slot {
        self.data::<data::Data>().slot_0
    }

    // fn get_fee_growth_global_0x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_0x128
    // }

    // fn get_fee_growth_global_1x128(&self) -> u128 {
    //     self.data::<data::Data>().fee_growth_global_1x128
    // }

    // fn get_liquidity(&self) -> u128 {
    //     self.data::<data::Data>().liquidity
    // }

    fn mint(
        &mut self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount: u128,
        data: u128,
    ) -> Result<(u128, u128), PoolError> {
        // need to convert to u256
        if amount == 0 {
            return Err(PoolError::ZeroAmmount);
        }
        //need to implement a better solution for conversion
        let amount_i128 = -(amount as i128);

        let (_, amount_0_int, amount_1_int) =
            self._modify_position(recipient, tick_lower, tick_upper, amount_i128)?;

        let amount_0 = amount_0_int as u128;
        let amount_1 = amount_1_int as u128;

        let contract = Self::env().account_id();

        let balance_0_before: u128;
        let balance_1_before: u128;

        // need to implement: IUniswapV3MintCallback(msg.sender).uniswapV3MintCallback(amount0, amount1, data)
        if amount_0 > 0 {
            balance_0_before = PSP22Ref::balance_of(&self.data::<data::Data>().token_0, contract);
            let balance_0_before_plus_amount_0 = balance_0_before
                .checked_add(amount_0)
                .ok_or(PoolError::AddOverflowBalance0)?;
            if balance_0_before_plus_amount_0 > balance_0_before {
                return Err(PoolError::M0);
            }
        }
        if amount_1 > 0 {
            balance_1_before = PSP22Ref::balance_of(&self.data::<data::Data>().token_1, contract);
            let balance_1_before_plus_amount_1 = balance_1_before
                .checked_add(amount_1)
                .ok_or(PoolError::AddOverflowBalance1)?;
            if balance_1_before_plus_amount_1 > balance_1_before {
                return Err(PoolError::M1);
            }
        }
        Ok((amount_0, amount_1))
    }

    // fn _update_position(&self, onwer: AccountId, tick_lower: i32, tick_upper: i32, liquidity_delta: i128, tick: i32) -> Result<Balance, PoolError>{
    //     let position = self.get_position(owner,tick_lower, tick_upper);
    //     let flipped_lower = false;
    //     let flipped_upper = false;

    // }

    fn get_position(
        &self,
        owner: AccountId,
        tick_lower: i32,
        tick_upper: i32,
    ) -> Option<PositionInfo> {
        self.data::<data::Data>()
            .positions
            .get(&Self::env().hash_encoded::<Blake2x256, _>(&(owner, tick_lower, tick_upper)))
    }

    fn _modify_position(
        &mut self,
        owner: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        liqudiity_delta: i128,
    ) -> Result<(PositionInfo, Balance, Balance), PoolError> {
        if !self._check_ticks(tick_lower, tick_upper) {
            return Err(PoolError::TickError);
        }
        let slot0 = self.get_slot_0();
        // need to implement _updatePosition

        let amount_0: u128 = 0;
        let amount_1: u128 = 0;

        // if liquidity_delta != 0{
        //     if slot0.tick < tick_lower{

        //     }
        // }

        let position = PositionInfo {
            liquidity: 0,
            fee_growth_inside_0_last_x128: 0,
            fee_growth_inside_1_last_x128: 0,
            tokens_owed_0: 0,
            tokens_owed_1: 0,
        };
        Ok((position, amount_0, amount_1))
    }

    fn _check_ticks(&self, tick_lower: i32, tick_upper: i32) -> bool {
        tick_lower < tick_upper && tick_lower >= i32::MIN && tick_upper <= i32::MAX
    }

    fn burn(
        &mut self,
        tick_lower: i32,
        tick_upper: i32,
        amount: u128,
    ) -> Result<(Balance, Balance), PoolError> {
        let amount_i128 = amount as i128;

        let (position, amount_0_int, amount_1_int) =
            self._modify_position(Self::env().caller(), tick_lower, tick_upper, amount_i128)?;

        let amount_0: u128 = amount_0_int.checked_neg().ok_or(PoolError::CheckedNeg0)?;
        let amount_1: u128 = amount_1_int.checked_neg().ok_or(PoolError::CheckedNeg1)?;

        if amount_0 > 0 || amount_1 > 0 {
            self.data::<data::Data>()
                .positions
                .get(&Self::env().hash_encoded::<Blake2x256, _>(&(
                    Self::env().caller(),
                    tick_lower,
                    tick_upper,
                )))
                .unwrap()
                .tokens_owed_0 = position.tokens_owed_0 + amount_0;

            self.data::<data::Data>()
                .positions
                .get(&Self::env().hash_encoded::<Blake2x256, _>(&(
                    Self::env().caller(),
                    tick_lower,
                    tick_upper,
                )))
                .unwrap()
                .tokens_owed_1 = position.tokens_owed_1 + amount_1;
        } else {
            return Err(PoolError::BurningInsuficientBalance);
        }

        self._emit_burn_event(
            Self::env().caller(),
            tick_lower,
            tick_upper,
            amount,
            amount_0,
            amount_1,
        );
        Ok((amount_0, amount_1))
    }

    default fn _emit_burn_event(
        &self,
        _sender: AccountId,
        _tick_lower: i32,
        _tick_upper: i32,
        _amount: Balance,
        _amount_0: Balance,
        _amount_1: Balance,
    ) {
    }
}

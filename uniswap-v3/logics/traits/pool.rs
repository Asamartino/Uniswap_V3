use openbrush::{
    contracts::{
        reentrancy_guard::*,
        traits::{ownable::*, pausable::*, psp22::PSP22Error},
    },
    traits::{AccountId, Balance, Timestamp},
};

use super::types::WrappedU256;

#[openbrush::wrapper]
pub type PoolRef = dyn Pool;

#[openbrush::trait_definition]
pub trait Pool {
    //from interface/pool/IUniswapV3PoolActions.sol
    /// @title Permissionless pool actions
    /// @notice Contains pool methods that can be called by anyone
    /// @notice Sets the initial price for the pool
    /// @dev Price is represented as a sqrt(amountToken1/amountToken0) Q64.96 value
    /// @param sqrtPriceX96 the initial sqrt price of the pool as a Q64.96
    #[ink(message)]
    fn initialize(&mut self, sqrt_price_x96: u128) -> Result<(), PoolError>;
    /// @notice Adds liquidity for the given recipient/tickLower/tickUpper position
    /// @dev The caller of this method receives a callback in the form of IUniswapV3MintCallback#uniswapV3MintCallback
    /// in which they must pay any token0 or token1 owed for the liquidity. The amount of token0/token1 due depends
    /// on tickLower, tickUpper, the amount of liquidity, and the current price.
    /// @param recipient The address for which the liquidity will be created
    /// @param tickLower The lower tick of the position in which to add liquidity
    /// @param tickUpper The upper tick of the position in which to add liquidity
    /// @param amount The amount of liquidity to mint
    /// @param data Any data that should be passed through to the callback
    /// @return amount0 The amount of token0 that was paid to mint the given amount of liquidity. Matches the value in the callback
    /// @return amount1 The amount of token1 that was paid to mint the given amount of liquidity. Matches the value in the callback
    #[ink(message)]
    fn mint(
        &mut self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount: i128,
    ) -> Result<(Balance, Balance), PSP22Error>;
    /// @notice Collects tokens owed to a position
    /// @dev Does not recompute fees earned, which must be done either via mint or burn of any amount of liquidity.
    /// Collect must be called by the position owner. To withdraw only token0 or only token1, amount0Requested or
    /// amount1Requested may be set to zero. To withdraw all tokens owed, caller may pass any value greater than the
    /// actual tokens owed, e.g. type(uint128).max. Tokens owed may be from accumulated swap fees or burned liquidity.
    /// @param recipient The address which should receive the fees collected
    /// @param tickLower The lower tick of the position for which to collect fees
    /// @param tickUpper The upper tick of the position for which to collect fees
    /// @param amount0Requested How much token0 should be withdrawn from the fees owed
    /// @param amount1Requested How much token1 should be withdrawn from the fees owed
    /// @return amount0 The amount of fees collected in token0
    /// @return amount1 The amount of fees collected in token1
    #[ink(message)]
    fn collect(
        &mut self,
        recipient: AccountId,
        tick_lower: i32,
        tick_upper: i32,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError>;

    /// @notice Burn liquidity from the sender and account tokens owed for the liquidity to the position
    /// @dev Can be used to trigger a recalculation of fees owed to a position by calling with an amount of 0
    /// @dev Fees must be collected separately via a call to #collect
    /// @param tickLower The lower tick of the position for which to burn liquidity
    /// @param tickUpper The upper tick of the position for which to burn liquidity
    /// @param amount How much liquidity to burn
    /// @return amount0 The amount of token0 sent to the recipient
    /// @return amount1 The amount of token1 sent to the recipient
    #[ink(message)]
    fn burn(
        &mut self,
        tick_lower: i32,
        tick_upper: i32,
        amount: i128,
    ) -> Result<(Balance, Balance), PoolError>;

    /// @notice Swap token0 for token1, or token1 for token0
    /// @dev The caller of this method receives a callback in the form of IUniswapV3SwapCallback#uniswapV3SwapCallback
    /// @param recipient The address to receive the output of the swap
    /// @param zeroForOne The direction of the swap, true for token0 to token1, false for token1 to token0
    /// @param amountSpecified The amount of the swap, which implicitly configures the swap as exact input (positive), or exact output (negative)
    /// @param sqrtPriceLimitX96 The Q64.96 sqrt price limit. If zero for one, the price cannot be less than this
    /// value after the swap. If one for zero, the price cannot be greater than this value after the swap
    /// @param data Any data to be passed through to the callback
    /// @return amount0 The delta of the balance of token0 of the pool, exact when negative, minimum when positive
    /// @return amount1 The delta of the balance of token1 of the pool, exact when negative, minimum when positive
    #[ink(message)]
    fn swap(
        &mut self,
        recipient: AccountId,
        zero_for_one: bool,
        amount_specified: Balance,
        sqrt_price_limit_x96: u128,
    ) -> Result<(Balance, Balance), PoolError>;
    /// @notice Receive token0 and/or token1 and pay it back, plus a fee, in the callback
    /// @dev The caller of this method receives a callback in the form of IUniswapV3FlashCallback#uniswapV3FlashCallback
    /// @dev Can be used to donate underlying tokens pro-rata to currently in-range liquidity providers by calling
    /// with 0 amount{0,1} and sending the donation amount(s) from the callback
    /// @param recipient The address which will receive the token0 and token1 amounts
    /// @param amount0 The amount of token0 to send
    /// @param amount1 The amount of token1 to send
    /// @param data Any data to be passed through to the callback
    #[ink(message)]
    fn flash(
        &mut self,
        recipient: AccountId,
        amount0: Balance,
        amount1: Balance,
        data: Vec<u8>,
    ) -> Result<(), PoolError>;
    /// @notice Increase the maximum number of price and liquidity observations that this pool will store
    /// @dev This method is no-op if the pool already has an observationCardinalityNext greater than or equal to
    /// the input observationCardinalityNext.
    /// @param observationCardinalityNext The desired minimum number of observations for the pool to store
    #[ink(message)]
    fn increase_observation_cardinality_next(
        &mut self,
        observation_cardinality_next: u16,
    ) -> Result<(), PoolError>;

    // interface/pool/IUniswapV3PoolState.sol
    /// @title Pool state that can change
    /// @notice These methods compose the pool's state, and can change with any frequency including multiple times
    /// per transaction

    #[ink(message)]
    fn slot0(&self) -> (Slot);
    /// @notice The fee growth as a Q128.128 fees of token0 collected per unit of liquidity for the entire life of the pool
    /// @dev This value can overflow the uint256
    #[ink(message)]
    fn fee_growth_global_0x128(&self) -> WrappedU256;

    /// @notice The fee growth as a Q128.128 fees of token1 collected per unit of liquidity for the entire life of the pool
    /// @dev This value can overflow the uint256
    #[ink(message)]
    fn fee_growth_global_1x128(&self) -> WrappedU256;

    /// @notice The amounts of token0 and token1 that are owed to the protocol
    /// @dev Protocol fees will never exceed uint128 max in either token
    #[ink(message)]
    fn protocol_fees(&self) -> ProtocolFees;

    /// @notice The currently in range liquidity available to the pool
    /// @dev This value has no relationship to the total liquidity across all ticks
    #[ink(message)]
    fn liquidity(&self) -> i128;

    #[ink(message)]
    fn ticks(&self, tick: i32) -> TickInfo;

    /// @notice Returns 256 packed tick initialized boolean values. See TickBitmap for more information
    #[ink(message)]
    fn tick_bitmap(&self, word_position: u16) -> WrappedU256;

    #[ink(message)]
    fn positions(&self, key: Hash) -> PositionInfo;

    #[ink(message)]
    fn observations(&self, index: u16) -> Observation;

    // interface/pool/IUniswapV3PoolOwnerActions.sol
    #[ink(message)]
    fn set_fee_protocol(&mut self, fee_protocol0: u8, fee_protocol1: u8) -> Result<(), PoolError>;

    #[ink(message)]
    fn collect_protocol(
        &mut self,
        recipient: AccountId,
        amount0_requested: Balance,
        amount1_requested: Balance,
    ) -> Result<(Balance, Balance), PoolError>;

    // interface/pool/IuniswapV3PoolPoolImmutables.sol
    /// @title Pool state that never changes
    /// @notice These parameters are fixed for a pool forever, i.e., the methods will always return the same values

    /// @notice The contract that deployed the pool, which must adhere to the IUniswapV3Factory interface
    /// @return The contract address
    #[ink(message)]
    fn factory(&self) -> AccountId;

    /// @notice The first of the two tokens of the pool, sorted by address
    /// @return The token contract address
    #[ink(message)]
    fn token0(&self) -> AccountId;

    /// @notice The second of the two tokens of the pool, sorted by address
    /// @return The token contract address
    #[ink(message)]
    fn token_1(&self) -> AccountId;
    /// @notice The pool's fee in hundredths of a bip, i.e. 1e-6
    /// @return The fee
    #[ink(message)]
    fn fee(&self) -> u8;
    /// @notice The pool tick spacing
    /// @dev Ticks can only be used at multiples of this value, minimum of 1 and always positive
    /// e.g.: a tickSpacing of 3 means ticks can be initialized every 3rd tick, i.e., ..., -6, -3, 0, 3, 6, ...
    /// This value is an int24 to avoid casting even though it is always positive.
    /// @return The tick spacing
    #[ink(message)]
    fn tick_spacing(&self) -> i8;
    /// @notice The maximum amount of position liquidity that can use any tick in the range
    /// @dev This parameter is enforced per tick to prevent liquidity from overflowing a uint128 at any point, and
    /// also prevents out-of-range liquidity from being used to prevent adding in-range liquidity to a pool
    /// @return The max amount of liquidity per tick
    #[ink(message)]
    fn max_liquidity_per_tick(&self) -> u8;

    // #[ink(message)]
    // fn get_reserves(&self) -> (Balance, Balance, Timestamp);

    // #[ink(message)]
    // fn price_0_cumulative_last(&self) -> WrappedU256;

    // #[ink(message)]
    // fn price_1_cumulative_last(&self) -> WrappedU256;

    // #[ink(message)]
    // fn skim(&mut self, to: AccountId) -> Result<(), PoolError>;

    // #[ink(message)]
    // fn sync(&mut self) -> Result<(), PoolError>;

    // #[ink(message)]
    // fn get_token_0(&self) -> AccountId;

    // #[ink(message)]
    // fn get_token_1(&self) -> AccountId;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PoolError {}

# Uniswap V3
For this [Polkadot Hackathon](https://www.polkadotglobalseries.com/?utm_source=Discord&utm_medium=socials&utm_campaign=launch), we converted the Uniswap V3 contracts to ink!. <br />
We relied on [tutorials](https://docs.astar.network/docs/build/wasm/from-zero-to-ink-hero/dex/) provided by Astar. <br />
Specifically the following contracts have been translated: <br />
- [Factory Contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Factory.sol)
- [Pool Deployer](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3PoolDeployer.sol)
- [Pool contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Pool.sol)*

\*this conversion is currently incomplete as this contract is quite large and calls many functions from the library.

# Uniswap V3
For this [Polkadot Hackathon](https://www.polkadotglobalseries.com/?utm_source=Discord&utm_medium=socials&utm_campaign=launch), we relied on [tutorials](https://docs.astar.network/docs/build/wasm/from-zero-to-ink-hero/dex/) provided by Astar, and are competing for the **Bounty: Build a DeFi dApp with ink!** which entails: <br />
- Built a DEX with ink! based on our tutorial.
- Add a frontend to interact with your contract. <br />

We converted the Uniswap V3 contracts to ink!, specifically the following contracts have been translated: <br />
- [Factory Contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Factory.sol)
- [Pool Deployer](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3PoolDeployer.sol)
- [Pool contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Pool.sol)*

\*this translation is currently incomplete as this contract is quite large and calls many functions from the library.

# Uniswap V3
For this [Polkadot Hackathon](https://www.polkadotglobalseries.com/?utm_source=Discord&utm_medium=socials&utm_campaign=launch), we relied on [tutorials](https://docs.astar.network/docs/build/wasm/from-zero-to-ink-hero/dex/) provided by Astar, and are competing for the **Bounty: Build a DeFi dApp with ink!** which entails: <br />
- Build a DEX with ink! based on Astar's tutorial
- Add a frontend to interact with our contract <br />

We converted the following Uniswap V3 contracts to ink!: <br />
- [Factory Contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Factory.sol)
- [Pool Deployer](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3PoolDeployer.sol)
- [Pool Contract](https://github.com/Uniswap/v3-core/blob/main/contracts/UniswapV3Pool.sol)*

\*this conversion is currently incomplete as this contract is quite large and calls many other functions from the library.

### Why convert Uniswap V3?
Uniswap V3 comes with many features that makes it one of the most flexible and efficient Automated Market Maker (AMM), that makes previous versions obsoletes (f.i. Uniswap V2, Sushiswap,etc.).


## Front-end implementation
[VerselURL](https://dexfrontend-lilac.vercel.app/)

Implemented some simple Ii and interaction with ink! by using Polkadot{js} API.

*note: swap and add liquidity on front-end is not working because we should implement [additional contract](https://github.com/Uniswap/v3-periphery/tree/main/contracts) outside of core contract, like `Router` and `Position Management`
![Screenshot 2023-02-17 at 13 02 38](https://user-images.githubusercontent.com/67859510/219695799-4fc4b143-4317-4d25-a468-7e91f236a4d8.png)
![Screenshot 2023-02-17 at 13 03 05](https://user-images.githubusercontent.com/67859510/219695823-1654cf0e-6a38-4f66-ad14-c9e692308b23.png)
![Screenshot 2023-02-17 at 13](https://user-images.githubusercontent.com/67859510/219697023-8405ebba-920a-43bb-92e7-6bba4f907d72.png)
![Screenshot 2023-02-18 at 0 31 17](https://user-images.githubusercontent.com/67859510/219696884-0d9a5019-ce99-4a8f-9b33-88dde15bf1f7.png)

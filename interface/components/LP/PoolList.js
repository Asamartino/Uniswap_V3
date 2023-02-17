import React from 'react'
import Pool from './Pool'
const PoolList = () => {
  return (
    <div>
      <div className='flex font-medium px-5 pb-3'>
        <div className='w-1/5 '>Pair</div>
        <div className='w-1/5 '>Liquidity</div>
        <div className='w-1/5 '>Volume(24h)</div>
        <div className='w-1/5 '>Fee</div>
        <div className='w-1/5 '>APY</div>
      </div>
      <Pool
        token0_symbol={'/assets/usdt.svg'}
        token1_symbol={'/assets/astar.png'}
        token0={'USDT'}
        token1={'ASTR'}
        APY={'64.50'}
        Fee={'20.50'}
        TVL={'123.50'}
        DVL={'34.50'}
      />
      <Pool
        token0_symbol={'/assets/usdt.svg'}
        token1_symbol={'/assets/uniswap.png'}
        token0={'USDT'}
        token1={'UNI'}
        APY={'64.50'}
        Fee={'20.50'}
        TVL={'123.50'}
        DVL={'34.50'}
      />
      <Pool
        token0_symbol={'/assets/uniswap.png'}
        token1_symbol={'/assets/astar.png'}
        token0={'UNI'}
        token1={'ASTR'}
        APY={'64.50'}
        Fee={'20.50'}
        TVL={'123.50'}
        DVL={'34.50'}
      />
    </div>
  )
}

export default PoolList

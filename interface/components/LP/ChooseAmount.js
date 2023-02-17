import React, { useState } from 'react'
import Link from 'next/link'
import ChooseAmountContainer from './ChooseAmountContainer'
const ChooseAmount = () => {
  return (
    <div>
      <div className='w-1/3 mx-auto mb-5'>
        <Link href={'/addliquidity'}>
          <button className='bg-gray-700 hover:bg-gray-500 text-white font-normal py-2 px-4 rounded-xl'>
            ← Go back
          </button>
        </Link>
      </div>
      <ChooseAmountContainer token0={'ETH'} token1={'USDC'} />
    </div>
  )
}

export default ChooseAmount

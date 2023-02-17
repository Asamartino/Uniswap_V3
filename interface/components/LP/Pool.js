import React from 'react'
import Image from 'next/image'
import Link from 'next/link'
const Pool = (props) => {
  const { token0_symbol, token1_symbol, token0, token1, TVL, DVL, APY, Fee } = props

  return (
    <Link href={'/addliquidity'}>
      <div className='bg-gray-800  hover:bg-gray-700 p-5 rounded-2xl shadow-lg border border-gray-700 text-white cursor-pointer mb-4'>
        <div className='flex font-medium'>
          <div className='w-1/5 flex p-3'>
            <div>
              <span className='p-1 flex'>
                <Image src={token0_symbol} alt='token0' width={16} height={16} />
                <Image src={token1_symbol} alt='token1' width={16} height={16} />
              </span>
            </div>
            <div>
              {token0} / {token1}
            </div>
          </div>
          <div className='w-1/5 py-3'>
            <span className='text-center'>{TVL} K</span>
          </div>
          <div className='w-1/5 py-3'>
            <span className='text-center'>${DVL} K</span>
          </div>
          <div className='w-1/5 py-3'>
            <span className='text-center'>${Fee} K</span>
          </div>
          <div className='w-1/5 py-3'>
            <span className=' text-center'>${APY} %</span>
          </div>
        </div>
      </div>
    </Link>
  )
}

export default Pool

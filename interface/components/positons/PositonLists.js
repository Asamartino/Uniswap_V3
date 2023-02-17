import React, { useState } from 'react'
import Card from './PositonContainer'
import Link from 'next/link'
import PositionList from './Position'
import Empty_position from './EmptyPosition'

const PositonList = () => {
  const [positon, setPosition] = useState(true)
  return (
    <div>
      <Card
        button={
          <Link href='/pools'>
            <button className='bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-full font-medium'>
              New Position
            </button>
          </Link>
        }
      >
        {positon ? (
          <div>
            <PositionList
              token0_symbol={'/assets/usdt.svg'}
              token1_symbol={'/assets/astar.png'}
              token0={'USDT'}
              token1={'ASTR'}
              rangerow={'0.05'}
              rangehigh={'0.06'}
            />
            <PositionList
              token0_symbol={'/assets/usdt.svg'}
              token1_symbol={'/assets/uniswap.png'}
              token0={'USDT'}
              token1={'UNI'}
              rangerow={'5.6'}
              rangehigh={'6.4'}
            />
            <PositionList
              token0_symbol={'/assets/uniswap.png'}
              token1_symbol={'/assets/astar.png'}
              token0={'UNI'}
              token1={'ASTR'}
              rangerow={'10.0'}
              rangehigh={'12.3'}
            />
          </div>
        ) : (
          <Empty_position />
        )}
      </Card>
    </div>
  )
}

export default PositonList

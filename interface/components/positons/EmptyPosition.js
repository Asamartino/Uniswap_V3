import React from 'react'
import { SiDatabricks } from 'react-icons/si'
const Empty_position = () => {
  return (
    <div>
      <div>
        <div className='flex items-center justify-center'>
          <SiDatabricks className='text-white-600 h-12 w-12' />
        </div>
        <p className='text-center text-white-600 mt-5'>Your active liquidity positions will appear here.</p>
      </div>
    </div>
  )
}

export default Empty_position

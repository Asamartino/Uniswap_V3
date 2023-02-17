import React from 'react'

const PoolsContainer = ({ title, children }) => (
  <div className='w-3/4 mx-auto '>
    <div className='flex justify-between items-center'>
      <h2 className='text-white text-3xl '>{title}</h2>
    </div>
    <div className='text-center my-5 p-6 rounded-2xl shadow-md bg-gray-800 border border-gray-700'>{children}</div>
  </div>
)

export default PoolsContainer

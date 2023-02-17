import React from 'react'

const Card = ({ children, button }) => (
  <div className='w-1/2 mx-auto '>
    <div className='flex justify-between items-center'>
      <h2 className='text-white text-3xl '>Your Positions</h2>
      {button}
    </div>
    <div className='text-center my-5 p-6 rounded-2xl shadow-md bg-gray-800 border border-gray-700'>{children}</div>
  </div>
)

export default Card

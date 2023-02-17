import React from 'react'

const SelectStrategy = ({ title, description }) => (
  <div className=' rounded-2xl shadow-md bg-gray-800 border border-gray-700 mb-5 cursor-pointer hover:bg-gray-500'>
    <div className='text-left mt-2 px-4 text-2xl font-medium'>{title}</div>
    <div className='text-left mb-2 p-4 text-sm '>{description}</div>
  </div>
)

export default SelectStrategy

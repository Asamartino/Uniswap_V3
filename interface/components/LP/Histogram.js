import React from 'react'

const Histogram = ({ data }) => {
  return (
    <div className='w-1/6 mx-auto border rounded-2xl'>
      <div className='flex flex-row'>
        {data.map((value, index) => (
          <div key={index} className='bg-gray-300 w-4' style={{ height: `${value}px` }}></div>
        ))}
      </div>
    </div>
  )
}

export default Histogram

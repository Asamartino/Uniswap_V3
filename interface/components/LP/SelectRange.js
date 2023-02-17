import React, { useState } from 'react'

const NumberSelector = (props) => {
  // set a current price as  default value
  const defaultNumber = props.defaultNumber || 0
  // set a tick as default increment value
  const defaultIncrement = props.defaultIncrement || 1
  const [number, setNumber] = useState(defaultNumber)

  const handleIncrement = () => {
    setNumber((prevCount) => prevCount + defaultIncrement)
  }

  const handleDecrement = () => {
    setNumber((prevCount) => prevCount - defaultIncrement)
  }
  const handleInput = (e) => {
    setNumber(parseFloat(e.target.value))
  }
  return (
    <div className='flex rounded-2xl p-4'>
      <button
        className='bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full mr-2'
        onClick={handleDecrement}
      >
        -
      </button>
      <input
        type='text'
        value={number}
        onChange={(e) => handleInput(e)}
        className=' text-gray-700 font-bold py-2 px-4 rounded-2xl text-center w-24'
      />
      <button
        className='bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full ml-2'
        onClick={handleIncrement}
      >
        +
      </button>
    </div>
  )
}

export default NumberSelector

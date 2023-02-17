import React, { useState } from 'react'
import Button from '../Button'
function ChooseAmountContainer(props) {
  const { token0, token1 } = props
  const [token0amount, setToken0Amount] = useState()
  const [token1amount, setToken1Amount] = useState()
  const [approve0, setApprove0] = useState(false)
  const [approve1, setApprove1] = useState(true)

  const handleInput1 = (e) => {
    setToken0Amount(parseFloat(e.target.value))
  }
  const handleInput2 = (e) => {
    setToken1Amount(parseFloat(e.target.value))
  }
  return (
    <div className='w-1/3 mx-auto rounded-2xl shadow-md bg-gray-800 border p-5'>
      <div className='text-center text-xl font-medium	mb-3'>Select Amount</div>
      <div className='flex flex-col'>
        <div className='flex justify-between mb-5'>
          <div className='text-medium text-xl'>{token0}</div>
          <input
            type='text'
            value={token0amount}
            placeholder={0}
            pattern='^[0-9]*[.,]?[0-9]*$'
            onChange={(e) => handleInput1(e)}
            className='bg-gray-200 text-gray-700 font-bold py-2 px-4 rounded-2xl text-right'
          />
        </div>
        {approve0 ? (
          ''
        ) : (
          <button className='bg-gray-700  text-gray-100 font-medium py-2 px-4 rounded-xl mb-3 border-2 border-rose-600 '>
            You need to Approve {token0}
          </button>
        )}
        <div className='flex justify-between mb-5'>
          <div className=' text-medium text-xl'>{token1}</div>
          <input
            type='text'
            value={token1amount}
            placeholder={0}
            pattern='^[0-9]*[.,]?[0-9]*$'
            onChange={(e) => handleInput2(e)}
            className='bg-gray-200 text-gray-700 font-bold py-2 px-4 rounded-2xl text-right'
          />
        </div>
        {approve1 ? (
          ''
        ) : (
          <button className='bg-gray-700  text-gray-100 font-medium py-2 px-4 rounded-xl mb-3 border-2 border-rose-600 '>
            You need to Approve {token1}
          </button>
        )}

        <hr />
      </div>
      <div className='flex flex-col'>
        <div className='flex justify-between my-5'>
          <div className='text-medium text-2xl'>Total</div>
          <input
            type='text'
            value={token0amount}
            placeholder={0}
            pattern='^[0-9]*[.,]?[0-9]*$'
            onChange={(e) => handleInput1(e)}
            className='bg-gray-200 text-gray-700 font-bold py-2 px-4 rounded-2xl text-right'
          />
        </div>
      </div>
      <div className='flex flex-col'>
        <div className='text-center'>
          {approve0 && approve1 ? (
            <button className='bg-blue-600  hover:bg-blue-500 w-1/2 rounded-2xl py-4 px-4 '>Confirm</button>
          ) : (
            <button className='bg-gray-400 text-white w-1/2 rounded-2xl py-4 px-4'>Need to Approve</button>
          )}
        </div>
      </div>
    </div>
  )
}

export default ChooseAmountContainer

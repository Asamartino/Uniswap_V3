import react from 'react'
import SelectStrategy from './SelectStrategy'
import NumberSelector from './SelectRange'
import Link from 'next/link'
import Histogram from './Histogram'

const Strategy = (curerntprice) => {
  return (
    <div>
      <div className='w-4/5 mx-auto mb-5'>
        <Link href={'/pools'}>
          <button className='bg-gray-700 hover:bg-gray-500 text-white font-normal py-2 px-4 rounded-xl'>
            ← Go back
          </button>
        </Link>
      </div>

      <div className='w-4/5 mx-auto rounded-2xl shadow-md bg-gray-800 border flex p-5'>
        <div className='m-5 w-1/2'>
          <SelectStrategy
            title={'Pro'}
            description={
              'This strategy is designed for experienced traders who are familiar with the risks of impermanent loss. It is recommended to use this strategy if you are comfortable with the risks of impermanent loss.'
            }
          />
          <SelectStrategy
            title={'Normal'}
            description={
              'This strategy is designed for traders who are familiar with the risks of impermanent loss. It is recommended to use this strategy if you are comfortable with the risks of impermanent loss.'
            }
          />
          <SelectStrategy
            title={'Custom'}
            description={
              'This strategy is designed for traders who are familiar with the risks of impermanent loss. It is recommended to use this strategy if you are comfortable with the risks of impermanent loss.'
            }
          />
        </div>
        <div className='p-5 w-1/2'>
          <div className='text-center text-2xl font-medium	mb-3'>Select Price Range</div>
          <div className='border h-1/2'>{/* This is the histogram component */}</div>
          <div className='flex justify-between text-center cursor-pointer'>
            <div className='my-4'>
              <div className='font-medium'>Min price</div>
              <NumberSelector defaultNumber={1600} defaultIncrement={1} />
            </div>
            <div className='my-4'>
              <div className='font-medium '>Current price</div>
              <div className='m-4 py-2 rounded-2xl '>1650.25 USDC</div>
            </div>
            <div className='my-4'>
              <div className='font-medium '>Max price</div>
              <NumberSelector defaultNumber={1680} defaultIncrement={1} />
            </div>
          </div>
          <div className='my-5 text-right'>
            <Link href={'/selectamount'}>
              <button className='bg-gray-700 hover:bg-gray-500 text-white font-normal py-2 px-4 rounded-xl '>
                Select Amount →
              </button>
            </Link>
          </div>
        </div>
      </div>
    </div>
  )
}
export default Strategy

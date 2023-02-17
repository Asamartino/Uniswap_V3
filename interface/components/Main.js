//TODO: APPROVE WAITING STATE
//TODO: SELECT TOKEN
//TODO: AmountOut

import Image from 'next/image'
import { RiSettings3Fill } from 'react-icons/ri'
import { AiOutlineDown, AiOutlineUp, AiOutlineArrowDown } from 'react-icons/ai'
import { IoSwapVertical } from 'react-icons/io5'
import astar from '../assets/astar.png'
import usdt from '../assets/usdt.svg'
import shiden from '../assets/Shiden.png'
import uniswap from '../assets/uniswap.png'
import { useContext, useEffect, useState } from 'react'
import Button from './Button'
import { TransactionContext } from '../context/TransactionContext'
import { ContractPromise } from '@polkadot/api-contract'
import { BN } from 'bn.js'

import { router_address, address0, address1, address2, address3, ONE, Decimal } from '../util/RouterUtil'
//abi
import PAIR_CONTRACT from '../contract/abi/pair'
import FACTORY_CONTRACT from '../contract/abi/factory'
import { PSP22_ABI } from '../contract/abi/psp22'
import { WNATIVE_ABI } from '../contract/abi/wnative'
import ROUTER_CONTRACT from '../contract/abi/router'
import Modal from 'react-modal'
import { useRouter } from 'next/router'
import LoadingTransaction from './Modal/LoadingTransaction'
Modal.setAppElement('#__next')

const style = {
  wrapper: `w-screen flex items-center justify-center mt-14`,
  content: `bg-[#191B1F] w-[40rem] rounded-2xl p-4`,
  formHeader: `px-2 flex items-center justify-between font-semibold text-xl`,
  transferPropContainer: `bg-[#20242A] my-3 rounded-2xl p-6 border border-[#20242A] hover:border-[#41444F]`,
  transferPropInput: `bg-transparent placeholder:text-[#B2B9D2] outline-none w-full text-2xl`,
  currencySelector: `flex-col ml-auto`,
  inputnum: `w-3/4`,
  currencySelectorContent: `w-full h-min flex justify-between items-center bg-[#2D2F36] hover:bg-[#41444F] rounded-2xl text-xl font-medium cursor-pointer p-2 mt-[-0.2rem]`,
  currencySelectorIcon: `flex`,
  swapIcon: `flex items-center justify-center`,
  currencySelectorTicker: `mx-2`,
  currencySelectorArrow: `text-lg w-5`,
  copyarea: `mt-2 text-right text-[#B2B9D2]`,
}

const Main = () => {
  const [showList, setShowList] = useState(false)
  const [Currency2, setCurrency2] = useState(false)
  const { currentAccount, api, signer } = useContext(TransactionContext)
  const [Token1Contract, setToken1Contract] = useState(undefined)
  const [Token2Contract, setToken2Contract] = useState(undefined)
  const [Token1Balance, setToken1balance] = useState('')
  const [Token2Balance, setToken2balance] = useState('')
  const [inputAmount1, setInputAmount1] = useState(0)
  const [inputAmount2, setInputAmount2] = useState(0)
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)
  const [AmountOutMin, setAmountOutMin] = useState()
  const [AmountInMax, setAmountInMax] = useState()

  //const gasLimit = 18750000000;
  const proofSize = 100000000000
  const refTime = 100000000000

  // const gasLimit = 100000000000
  const storageDepositLimit = null

  const customStyles = {
    content: {
      top: '50%',
      left: '50%',
      right: 'auto',
      bottom: 'auto',
      transform: 'translate(-50%, -50%)',
      backgroundColor: '#0a0b0d',
      padding: 0,
      border: 'none',
    },
    overlay: {
      backgroundColor: 'rgba(10, 11, 13, 0.75)',
    },
  }
  useEffect(() => {
    if (isLoading) {
      router.push(`/?loading=${currentAccount}`)
    } else {
      router.push(`/`)
    }
  }, [isLoading])
  const handleInput1 = (e) => {
    setInputAmount1(e.target.value)
  }
  const handleInput2 = (e) => {
    setInputAmount2(e.target.value)
  }

  useEffect(() => {
    if (api && currentAccount) {
      setup()
    }
  }, [api, currentAccount])
  useEffect(() => {
    if (api && currentAccount && inputAmount1 > 0) {
      getAmountOut()
    }
    if (inputAmount1 == '') {
      setAmountOutMin(0)
    }
  }, [inputAmount1])
  useEffect(() => {
    if (api && currentAccount && inputAmount2 > 0) {
      getAmountIn()
    }
    if (inputAmount2 == '') {
      setAmountInMax(0)
    }
  }, [inputAmount2])
  const setup = async () => {
    const gasLimit = await api.registry.createType('WeightV2', {
      refTime,
      proofSize,
    })
    // initialize contracts
    const getToken1Contract = new ContractPromise(api, PSP22_ABI, address3) //USDT
    const getToken2Contract = new ContractPromise(api, WNATIVE_ABI, address1) //wsby
    setToken1Contract(getToken1Contract)
    setToken2Contract(getToken2Contract)
    const balance1 = await getToken1Contract.query['psp22::balanceOf'](
      currentAccount.address,
      { gasLimit: gasLimit, storageDepositLimit: storageDepositLimit },
      currentAccount.address,
    )
    if (balance1.result.isOk) {
      setToken1balance(balance1.output.value.toString() / 10 ** Decimal)
      console.log('Token1Balance: ', balance1.output.value.toString())
    } else {
      console.error('Error', balance1.result.asErr)
    }

    const balance2 = await getToken2Contract.query['psp22::balanceOf'](
      currentAccount.address,
      { gasLimit: gasLimit, storageDepositLimit: storageDepositLimit },
      currentAccount.address,
    )
    if (balance2.result.isOk) {
      setToken2balance(balance2.output.toString() / 10 ** Decimal)
      console.log('Token2Balance: ', balance2.output.toString())
    } else {
      console.error('Error', balance2.result.asErr)
    }
  }
  const getAmountOut = async () => {
    // const token0 = BigInt(inputAmount1 * 10 ** Decimal)
    // const token1 = BigInt(inputAmount2 * 10 ** Decimal)
    const router = new ContractPromise(api, ROUTER_CONTRACT, router_address)
    // return [AmountIn,AmountOut]
    const AmountOut = await router.query['router::getAmountsOut'](
      currentAccount.address,
      { gasLimit: gasLimit, storageDepositLimit },
      inputAmount1,
      [Token2Contract.address, Token1Contract.address],
    )
    if (AmountOut.result.isOk) {
      setAmountOutMin(BigInt(AmountOut.output.toJSON()['ok'][1]))
    } else {
      // console.error('Error', result.asErr)
    }
  }
  const getAmountIn = async () => {
    // const token0 = BigInt(inputAmount1 * 10 ** Decimal)
    // const token1 = BigInt(inputAmount2 * 10 ** Decimal)
    const router = new ContractPromise(api, ROUTER_CONTRACT, router_address)
    // return [AmountIn,AmountOut]
    const AmountIn = await router.query['router::getAmountsIn'](
      currentAccount.address,
      { gasLimit, storageDepositLimit },
      inputAmount2,
      [Token2Contract.address, Token1Contract.address],
    )
    if (AmountIn.result.isOk) {
      setAmountInMax(BigInt(AmountIn.output.toJSON()['ok'][0]))
      console.log(AmountInMax)
    } else {
      // console.error('Error', result.asErr)
    }
  }
  // TODO: Add other swap functions
  const runswap = async () => {
    setIsLoading(true)
    const deadline = '111111111111111111'
    const router = new ContractPromise(api, ROUTER_CONTRACT, router_address)
    await router.tx['router::swapExactTokensForTokens'](
      { gasLimit, storageDepositLimit },
      inputAmount1,
      0,
      [Token2Contract.address, Token1Contract.address], //path [input, output]
      currentAccount.address,
      deadline,
    ).signAndSend(currentAccount.address, { signer: signer.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`)
        setIsLoading(false)
        setup()
      } else {
        console.log(`Current status: ${status.type}`)
      }
    })
  }

  return (
    <div className={style.wrapper}>
      <div className={style.content}>
        <div className={style.formHeader}>
          <div>Swap</div>
          <div>
            <RiSettings3Fill />
          </div>
        </div>
        <div className={style.transferPropContainer}>
          <div className='flex'>
            <div className={style.inputnum}>
              <input
                type='text'
                className={style.transferPropInput}
                placeholder={AmountInMax}
                pattern='^[0-9]*[.,]?[0-9]*$'
                onChange={(e) => handleInput1(e, 'input1amount')}
              />
            </div>
            <div className={style.currencySelector}>
              <button className={style.currencySelectorContent} onClick={() => setShowList((prevState) => !prevState)}>
                <div className={style.currencySelectorIcon}>
                  <Image src={usdt} alt='USDT' height={20} width={20} />
                </div>
                <div className={style.currencySelectorTicker}>USDT</div>
                {showList ? (
                  <AiOutlineUp className={style.currencySelectorArrow} />
                ) : (
                  <AiOutlineDown className={style.currencySelectorArrow} />
                )}
              </button>
            </div>
          </div>
          <div>
            {Token1Balance !== '0' ? (
              <div>
                <div className={style.copyarea}>Balance: {Token1Balance}</div>
              </div>
            ) : (
              <div className={style.copyarea}>Balance: 0</div>
            )}
          </div>
        </div>
        <div className={style.swapIcon}>
          <AiOutlineArrowDown size={42} />
        </div>
        <div className={style.transferPropContainer}>
          <div className='flex'>
            <div className={style.inputnum}>
              <input
                type='text'
                className={style.transferPropInput}
                placeholder={AmountInMax}
                pattern='^[0-9]*[.,]?[0-9]*$'
                onChange={(e) => handleInput1(e, 'input1amount')}
              />
            </div>
            <div className={style.currencySelector}>
              <button className={style.currencySelectorContent} onClick={() => setShowList((prevState) => !prevState)}>
                <div className={style.currencySelectorIcon}>
                  <Image src={astar} alt='ASTR' height={20} width={20} />
                </div>
                <div className={style.currencySelectorTicker}>WSBY</div>
                {showList ? (
                  <AiOutlineUp className={style.currencySelectorArrow} />
                ) : (
                  <AiOutlineDown className={style.currencySelectorArrow} />
                )}
              </button>
            </div>
          </div>
          <div>
            {Token2Balance ? (
              <div>
                <div className={style.copyarea}>Balance: {Token2Balance}</div>
              </div>
            ) : (
              <div className={style.copyarea}>Balance: 0</div>
            )}
          </div>
        </div>

        <div onClick={() => runswap()}>
          <Button title='Swap' />
        </div>
      </div>
      <Modal isOpen={!!router.query.loading} style={customStyles}>
        <LoadingTransaction />
      </Modal>
    </div>
  )
}
export default Main

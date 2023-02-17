import Image from 'next/image'
import React, { useContext, useEffect, useState } from 'react'
import { RiSettings3Fill } from 'react-icons/ri'
import { AiOutlineDown, AiOutlinePlus } from 'react-icons/ai'
import astar from '../assets/astar.png'
import Shiden from '../assets/Shiden.png'
import uniswap from '../assets/uniswap.png'
import Button from '../Button'
import { TransactionContext } from '../../context/TransactionContext'

import ROUTER_CONTRACT from '../../contract/abi/router'
import { ContractPromise } from '@polkadot/api-contract'
import { PSP22_ABI } from '../../contract/abi/psp22'
import { WNATIVE_ABI } from '../../contract/abi/wnative'
import {
  factory_address,
  router_address,
  pair_address,
  address0,
  address1,
  address2,
  ONE,
  Decimal,
} from '../../util/RouterUtil'
import { BN } from 'bn.js'
import Modal from 'react-modal'
import { useRouter } from 'next/router'
import LoadingTransaction from '../Modal/LoadingTransaction'
Modal.setAppElement('#__next')

const style = {
  wrapper: `w-screen flex items-center justify-center mt-14`,
  content: `bg-[#191B1F] w-[40rem] rounded-2xl p-4`,
  formHeader: `px-2 flex items-center justify-between font-semibold text-xl`,
  transferPropContainer: `bg-[#20242A] my-3 rounded-2xl p-6 text-3xl  border border-[#20242A] hover:border-[#41444F]  flex justify-between`,
  transferPropInput: `bg-transparent placeholder:text-[#B2B9D2] outline-none mb-6 w-full text-2xl`,
  currencySelector: `flex w-1/4`,
  currencySelectorContent: `w-full h-min flex justify-between items-center bg-[#2D2F36] hover:bg-[#41444F] rounded-2xl text-xl font-medium cursor-pointer p-2 mt-[-0.2rem]`,
  currencySelectorIcon: `flex items-center`,
  currencySelectorTicker: `mx-2`,
  currencySelectorArrow: `text-lg`,
  plusIcon: `flex items-center justify-center`,
  confirmButton: `bg-[#2172E5] my-2 rounded-2xl py-6 px-8 text-xl font-semibold flex items-center justify-center cursor-pointer border border-[#2172E5] hover:border-[#234169]`,
}
const Liquidity = () => {
  const { currentAccount, api, signer } = useContext(TransactionContext)
  const [Token1Contract, setToken1Contract] = useState(undefined)
  const [token2Contract, setToken2Contract] = useState(undefined)

  const [inputAmount1, setInputAmount1] = useState('')
  const [inputAmount2, setInputAmount2] = useState('')
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)
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
  const [isapproving, setIsApproving] = useState(false)
  const gasLimit = 100000000000
  const storageDepositLimit = null
  const approve = async () => {
    setIsApproving(true)
    const getToken1Contract = new ContractPromise(api, PSP22_ABI, address0)
    const getToken2Contract = new ContractPromise(api, WNATIVE_ABI, address2)
    await getToken1Contract.tx['psp22::approve'](
      {
        gasLimit,
        storageDepositLimit,
      },
      router_address,
      inputAmount1,
    ).signAndSend(currentAccount.address, { signer: signer.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`)
        setIsApproving(false)
      } else {
        console.log(`Current status: ${status.type}`)
      }
    })
    await getToken2Contract.tx['psp22::approve'](
      {
        gasLimit,
        storageDepositLimit,
      },
      router_address,
      inputAmount2,
    ).signAndSend(currentAccount.address, { signer: signer.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`)
        setIsApproving(false)
      } else {
        console.log(`Current status: ${status.type}`)
      }
    })
  }
  const token0 = BigInt(inputAmount1 * 10 ** Decimal)
  const token1 = BigInt(inputAmount2 * 10 ** Decimal)
  console.log('WSBY:', token0, 'UNI2', token1)
  const add_liquidity = async () => {
    setIsLoading(true)
    const getToken1Contract = new ContractPromise(api, PSP22_ABI, address2)
    const getToken2Contract = new ContractPromise(api, WNATIVE_ABI, address1)
    //add liquidity via router
    const router = new ContractPromise(api, ROUTER_CONTRACT, router_address)
    const deadline = '111111111111111111'

    await router.tx['router::addLiquidity'](
      { gasLimit, storageDepositLimit },
      getToken1Contract.address,
      getToken2Contract.address,
      token0,
      token1,
      0,
      0,
      currentAccount.address,
      deadline,
    ).signAndSend(currentAccount.address, { signer: signer.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`)
        setIsLoading(false)
      } else {
        console.log(`Current status: ${status.type}`)
      }
    })
  }
  return (
    <div className={style.wrapper}>
      <div className={style.content}>
        <div className={style.formHeader}>
          <div>Add Liquidity</div>
          <div>
            <RiSettings3Fill />
          </div>
        </div>
        <div className={style.transferPropContainer}>
          <input
            type='text'
            className={style.transferPropInput}
            placeholder='0.0'
            pattern='^[0-9]*[.,]?[0-9]*$'
            onChange={(e) => handleInput1(e, 'inputAmount1')}
          />
          <div className={style.currencySelector}>
            <div className={style.currencySelectorContent}>
              <div className={style.currencySelectorIcon}>
                <Image src={astar} alt='astar' height={20} width={20} />
              </div>
              <div className={style.currencySelectorTicker}>WSBY</div>
              <AiOutlineDown className={style.currencySelectorArrow} />
            </div>
          </div>
        </div>
        <div className={style.plusIcon}>
          <AiOutlinePlus size={42} />
        </div>

        <div className={style.transferPropContainer}>
          <input
            type='text'
            className={style.transferPropInput}
            placeholder='0.0'
            pattern='^[0-9]*[.,]?[0-9]*$'
            onChange={(e) => handleInput2(e, 'InputAmount2')}
          />
          <div className={style.currencySelector}>
            <div className={style.currencySelectorContent}>
              <div className={style.currencySelectorIcon}>
                <Image src={uniswap} alt='uniswap' height={20} width={20} />
              </div>
              <div className={style.currencySelectorTicker}>UNI</div>
              <AiOutlineDown className={style.currencySelectorArrow} />
            </div>
          </div>
        </div>
        <div onClick={() => add_liquidity()}>
          <Button title='Add liquidity' />
        </div>
      </div>
      <Modal isOpen={!!router.query.loading} style={customStyles}>
        <LoadingTransaction />
      </Modal>{' '}
    </div>
  )
}
export default Liquidity

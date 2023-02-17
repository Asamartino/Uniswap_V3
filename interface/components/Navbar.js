import React, { useContext, useState } from 'react'
import Link from 'next/link'
import Image from 'next/image'
import { FiArrowUpRight } from 'react-icons/fi'
import { AiOutlineDown } from 'react-icons/ai'
import { HiOutlineDotsVertical } from 'react-icons/hi'
import uniswap from '../assets/uniswap.png'
import astar from '../assets/astar.png'
import { TransactionContext } from '../context/TransactionContext'
import dynamic from 'next/dynamic'
const Identicon = dynamic(() => import('@polkadot/react-identicon'), {
  ssr: false,
})

const style = {
  headwrapper: `p-4 w-screen flex justify-between items-center`,
  headerLogo: `flex w-1/4 items-center justify-start`,
  nav: `flex-1 flex justify-center items-center`,
  navItemsContainer: `flex bg-[#191B1F] rounded-3xl`,
  navItem: `px-4 py-2 m-1 flex items-center text-lg font-semibold text-[0.9rem] cursor-pointer rounded-3xl`,
  // activeNavItem: `bg-[#20242A]`,
  buttonsContainer: `flex w-1/4 justify-end items-center`,
  button: `flex items-center bg-[#191B1F] rounded-2xl mx-2 text-[0.9rem] font-semibold cursor-pointer`,
  buttonPadding: `p-2`,
  buttonTextContainer: `h-8 p-2 flex items-center`,
  buttonIconContainer: `flex items-center justify-center w-8 h-8`,
  buttonAccent: `bg-[#172A42] border border-[#163256] hover:border-[#234169] h-full rounded-2xl flex items-center justify-center text-[#4F90EA]`,
}

const Navbar = () => {
  const [selectedNav, setSelectedNav] = useState('swap')
  const { connectWallet, currentAccount } = useContext(TransactionContext)

  return (
    <div className='flex justify-between items-center px-4 py-5'>
      <div className={style.headerLogo}>
        <Image src={uniswap} alt='Uniswap' height={40} width={40} />
      </div>
      <div className={style.nav}>
        <div className={style.navItemsContainer}>
          <Link href='/'>
            <div
              onClick={() => setSelectedNav('swap')}
              className={`${style.navItem} ${selectedNav === 'swap' && style.activeNavItem}`}
            >
              Swap
            </div>
          </Link>
          <Link href='/pools'>
            <div
              onClick={() => setSelectedNav('pool')}
              className={`${style.navItem} ${selectedNav === 'pool' && style.activeNavItem}`}
            >
              Pool
            </div>
          </Link>
          <Link href='/account'>
            <div
              onClick={() => setSelectedNav('account')}
              className={`${style.navItem} ${selectedNav === 'account' && style.activeNavItem}`}
            >
              Account
            </div>
          </Link>
          <a href='https://portal.astar.network/#/shiden/assets' target='_blank' rel='noreferrer'>
            <div className={style.navItem}>
              bridge <FiArrowUpRight />
            </div>
          </a>
        </div>
      </div>

      <div className={style.buttonsContainer}>
        <div className={`${style.button} ${style.buttonPadding}`}>
          <div className={style.buttonIconContainer}>
            <Image src={astar} alt='astar' height={20} width={20} />
          </div>
          <p>Shibuya</p>
          <div className={style.buttonIconContainer}>
            <AiOutlineDown />
          </div>
        </div>
				
        {currentAccount ? (
          <div className={`${style.button} ${style.buttonPadding}`}>
            <div className={style.buttonIconContainer}>
              <Identicon value={currentAccount.address} size={20} theme={'polkadot'} />
            </div>
            <div className={style.buttonTextContainer}>{currentAccount.meta.name}</div>
          </div>
        ) : (
          <div onClick={() => connectWallet()} className={`${style.button} ${style.buttonPadding}`}>
            <div className={`${style.buttonAccent} ${style.buttonPadding}`}>Connect Wallet</div>
          </div>
        )}
        <div className={`${style.button} ${style.buttonPadding}`}>
          <div className={`${style.buttonIconContainer} mx-2`}>
            <HiOutlineDotsVertical />
          </div>
        </div>
      </div>
    </div>
  )
}

export default Navbar

import React, { useContext, useEffect, useState } from 'react'
import type { NextPage } from 'next'
// import AddLiquidity from '../components/AddLiquidity'
// import AccountList from '../components/PositonLists'
import Main from '../components/Main'
// import WalletConnect from '../components/ConnectWallet'
// import Image from 'next/image'
// import { FiArrowUpRight } from 'react-icons/fi'
// import { AiOutlineDown } from 'react-icons/ai'
// import { HiOutlineDotsVertical } from 'react-icons/hi'

// import astar from '../assets/astar.png'
// import uniswap from '../assets/uniswap.png'
import { TransactionContext } from '../context/TransactionContext'
import Footer from '../components/Footer'
import dynamic from 'next/dynamic'
import Navbar from '../components/Navbar'
const Identicon = dynamic(() => import('@polkadot/react-identicon'), {
  ssr: false,
})

const style = {
  wrapper: `h-screen max-h-screen h-min-screen w-screen from-gray-900 to-gray-600 bg-gradient-to-r text-white select-none flex flex-col justify-between	`,
}

const Home: NextPage = () => {
  // const [selectedNav, setSelectedNav] = useState('swap')
  // const { connectWallet, currentAccount, api } = useContext(TransactionContext)
  return (
    <div className={style.wrapper}>
      <Navbar />
      <Main />
      <Footer />
    </div>
  )
}

export default Home

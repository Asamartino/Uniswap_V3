import { ApiPromise, WsProvider } from '@polkadot/api'
import React, { useState, useEffect } from 'react'

export const TransactionContext = React.createContext()

let pjs

if (typeof window !== 'undefined') {
  pjs = window.injectedWeb3
}
/* EndPoint
  'wss://shibuya.public.blastapi.io'
	'wss://shibuya-rpc.dwellir.com'
	'wss://rpc.shibuya.astar.network' 
	'ws://127.0.0.1:9944'
 */
const WS_PROVIDER = 'wss://rpc.shibuya.astar.network'
const DAPP_NAME = 'CLAMM'
export const TransactionProvider = ({ children }) => {
  const [currentAccount, setCurrentAccount] = useState()
  const [api, setapi] = useState()
  const [signer, setSigner] = useState()
  useEffect(() => {
    checkIfWalletIsConnected()
  }, [])
  const connectWallet = async (pjs = 'polkadot-js') => {
    try {
      const { web3Enable, web3Accounts, web3FromAddress } = await import('@polkadot/extension-dapp')
      const extensions = await web3Enable(DAPP_NAME)
      if (extensions.length === 0) {
        // no extension installed, or the user did not accept the authorization
        // in this case we should inform the use and give a link to the extension
        return
      }
      const provider = new WsProvider(WS_PROVIDER)
      const api = await ApiPromise.create({ provider })
      setapi(api)
      const allaccounts = await web3Accounts()
      const account = allaccounts[0]
      setCurrentAccount(account)
      if (account.address) {
        const injector = await web3FromAddress(account.address)
        setSigner(injector)
      }
    } catch (error) {
      console.error(error)
    }
  }
  const checkIfWalletIsConnected = async (pjs = 'polkadot-js') => {
    try {
      if (!currentAccount) {
        const { web3Enable } = await import('@polkadot/extension-dapp')
        const extensions = await web3Enable(DAPP_NAME)
        if (extensions.length === 0) {
          return
        }
      }
      const { web3Enable, web3Accounts, web3FromAddress } = await import('@polkadot/extension-dapp')
      const extensions = await web3Enable(DAPP_NAME)
      if (extensions.length === 0) {
        return
      }
      const allaccounts = await web3Accounts()
      const provider = new WsProvider(WS_PROVIDER)

      const api = await ApiPromise.create({ provider })
      setapi(api)
      const properties = await api.rpc.system.properties()
      const account = allaccounts[0]
      setCurrentAccount(account)
      if (account.address) {
        const injector = await web3FromAddress(account.address)
        setSigner(injector)
      }
    } catch (error) {
      console.error(error)
    }
  }
  return (
    <TransactionContext.Provider
      value={{
        currentAccount,
        connectWallet,
        api,
        signer,
      }}
    >
      {children}
    </TransactionContext.Provider>
  )
}

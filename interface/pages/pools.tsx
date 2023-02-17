import type { NextPage } from 'next'
import Footer from '../components/Footer'
import Navbar from '../components/Navbar'
import PoolsContainer from '../components/LP/PoolsContainer'
import PoolList from '../components/LP/PoolList'
const style = {
  wrapper: `h-screen max-h-screen h-min-screen w-screen from-gray-900 to-gray-600 bg-gradient-to-r text-white select-none flex flex-col justify-between	`,
}
const Pools: NextPage = () => {
  return (
    <div className={style.wrapper}>
      <Navbar />
      <PoolsContainer title={'Active Pools'} children={<PoolList />} />
      <Footer />
    </div>
  )
}
export default Pools

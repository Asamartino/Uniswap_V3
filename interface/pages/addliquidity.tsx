import type { NextPage } from 'next'
import Footer from '../components/Footer'
import Navbar from '../components/Navbar'
import Strategy from '../components/LP/Strategy'
const style = {
  wrapper: `h-screen max-h-screen h-min-screen w-screen from-gray-900 to-gray-600 bg-gradient-to-r text-white select-none flex flex-col justify-between	`,
}
const AddLiquidity: NextPage = () => {
  return (
    <div className={style.wrapper}>
      <Navbar />
      <Strategy />
      <Footer />
    </div>
  )
}
export default AddLiquidity

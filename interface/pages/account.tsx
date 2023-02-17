import type { NextPage } from 'next'
import Navbar from '../components/Navbar'
import PositonList from '../components/positons/PositonLists'
import Footer from '../components/Footer'
const style = {
  wrapper: `h-screen max-h-screen h-min-screen w-screen from-gray-900 to-gray-600 bg-gradient-to-r text-white select-none flex flex-col justify-between	`,
}
const Account: NextPage = () => {
  return (
    <div className={style.wrapper}>
      <Navbar />
      <PositonList />
      <Footer />
    </div>
  )
}
export default Account

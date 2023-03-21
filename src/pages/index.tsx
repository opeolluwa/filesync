import SearchBar from '@/components/SearchBar'
import { ArrowDownIcon, Bars3BottomLeftIcon, ComputerDesktopIcon, MusicalNoteIcon, PhotoIcon, PlayIcon } from '@heroicons/react/24/outline'


interface QuickAccessTab {
  name: string,
  icon: any,
  color: string,
  action?: (path: string) => void
}


const quickAccessTabs: QuickAccessTab[] = [
  {
    name: 'Images',
    icon: <PhotoIcon />,
    color: '#6166fe'
  },
  {
    name: 'Music',
    icon: <MusicalNoteIcon />,
    color: '#6166fe'

  },
  {
    name: 'Videos',
    icon: <PlayIcon />,
    color: '#3074f5'
  },
  {
    name: 'Documents',
    icon: <Bars3BottomLeftIcon />,
    color: '#3074f5'

  },
  {
    name: 'Downloads',
    icon: <ArrowDownIcon />,
    color: '#22244a'
  },
  {
    name: 'Desktop',
    icon: <ComputerDesktopIcon />,
    color: '#22244a'

  }
]

const recentFiles = [
  {
    name: 'image1.jpg',
    size: '1.2MB',
    lastModified: 'Aug 26 2022'
  },
  {
    name: 'image2.jpg',
    size: '1.2MB',
    lastModified: 'Oct 6 2022'
  },
  {
    name: 'image3.jpg',
    size: '1.2MB',
    lastModified: 'Oct 6 2022'
  },

]

export default function Main() {
  return (
    <>
     {/* search bar goes here */}
      <section>
        <SearchBar onSearch={function (city: string): void {
          throw new Error('Function not implemented.')
        }} />
      </section>

      {  /*quick access section*/}
      <section className='my-12'>
        <h2 className='mt-20 mb-8  font-medium dark:text-gray-300'>
          Quick Access
        </h2>
        <ul className='flex flex-wrap gap-10 items-center justify-around mt-4'>
          {quickAccessTabs.map((tab, index) => (
            <li key={index} className='flex flex-col items-center justify-center w-20 h-20' >
              <a href={tab.name.toLowerCase()} className='rounded-[20px] shadow-md dark:shadow-none shadow-gray-400  px-3' style={{
                backgroundColor: tab.color
              }}>
                <div className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100'>
                  {tab.icon}
                </div>
              </a>
              <span className='text-gray-600 dark:text-gray-500 block mt-2 text-small'>
                {tab.name}
              </span>
            </li>
          ))}
        </ul>
      </section>
      {/**preview section */}
      <section className="my-16">
        <h2 className='flex justify-between mt-24 mb-4 '>
          <span className=' font-medium dark:text-gray-400'>
            Recent Files
          </span>
          <span className='text-gray-500 text-violet-600 dark:text-violet'>
            view all
          </span>
        </h2>


        <div className="relative overflow-x-auto bg-white rounded-[24px] shadow-lg px-4 py-8 dark:bg-mirage-500">
          <table className="w-full text-sm text-left">
            <thead className="text-gray-500">
              <tr>
                <th scope="col" className="px-6 py-3 rounded-l-lg">
                  Name
                </th>
                <th scope="col" className="px-6 py-3">
                  Size
                </th>
                <th scope="col" className="px-6 py-3 rounded-r-lg">
                  Last Modified
                </th>
              </tr>
            </thead>
            <tbody className='text-gray-500'>

              {recentFiles.map((file, index) => (
                <tr key={index}>
                  <td className="px-6 py-4">
                    {file.name}
                  </td>
                  <td className="px-6 py-4">
                    {file.size}
                  </td>
                  <td className="px-6 py-4">
                    {file.lastModified}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

      </section>
      {/*recent files section*/}
      <section className='my-16 hidden'>
        <h2 className='flex justify-between mt-20 mb-8 '>
          <span className=' font-medium dark:text-gray-400'>
            Recent Files
          </span>
          <span className='text-gray-500 dark:text-violet'>
            view all
          </span>
        </h2>

      </section>
    </>
  )
}

import PageTitle from '@/components/PageTitle'
import SearchBar from '@/components/SearchBar'
import { Bars3BottomLeftIcon, CloudArrowDownIcon, ComputerDesktopIcon, MusicalNoteIcon, PhotoIcon, PlayIcon } from '@heroicons/react/24/solid'

interface QuickAccessTab {
  name: string,
  icon: any,
  color: string,
}

const quickAccessTabs: QuickAccessTab[] = [
  {
    name: 'Images',
    icon: <PhotoIcon className='rounded-lg my-4 mx-2 flex w-[47.5px] dark:text-shilo-300 text-gray-100' />,
    color: '#6166fe',
    // action: getImageFiles
  },
  {
    name: 'Music',
    icon: <MusicalNoteIcon className='rounded-lg my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#6166fe'

  },
  {
    name: 'Videos',
    icon: <PlayIcon className='rounded-lg my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#3074f5'
  },
  {
    name: 'Documents',
    icon: <Bars3BottomLeftIcon className='rounded-lg my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#3074f5'

  },
  {
    name: 'Downloads',
    icon: <CloudArrowDownIcon className='rounded-lg my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#22244a'
  },
  {
    name: 'Desktop',
    icon: <ComputerDesktopIcon className='rounded-lg my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
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
      <section>
        <SearchBar onSearch={function (city: string): void {
          throw new Error('Function not implemented.')
        }} />
      </section>

      <section className='my-12'>
        <PageTitle title={'Quick Access'} />
        <ul className='flex flex-wrap gap-24 items-center justify-start mt-4'>
          {quickAccessTabs.map((tab, index) => (
            <li key={index} className='flex flex-col items-center justify-center w-20 h-20'>
              <a href={'quick-access/' + tab.name.toLowerCase()} className='rounded-[12px]  dark:shadow-none shadow-gray-400  px-3' style={{
                backgroundColor: tab.color
              }}>
                <div className='hover:brightness-50 sepia-0'>
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
  

    </>
  )
}

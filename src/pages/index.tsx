import SearchBar from '@/components/SearchBar'
import { ArrowDownIcon, Bars3BottomLeftIcon, ComputerDesktopIcon, MusicalNoteIcon, PhotoIcon, PlayIcon } from '@heroicons/react/24/outline'
import { message } from '@tauri-apps/api/dialog';
import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
import { audioDir } from '@tauri-apps/api/path';
// const audioDirPath = await audioDir();

interface QuickAccessTab {
  name: string,
  icon: any,
  color: string,
  // action?: () => any // action that will be executed when the route is clicked 
}

// get image files 
// async function getImageFiles() {
//   // todo read dir
//   const files = await readDir('/', { dir: BaseDirectory.Picture, recursive: true }).catch((err) => {
//     message('error opening file manager', {
//       title: 'Access error',
//       type: 'error'
//     })
//     console.log(err.message);

//   });
// }
// get music files
// const musicFiles = await getFiles('music')

// get video files
// const videoFiles = await getFiles('videos')

// get document files
// const documentFiles = await getFiles('documents')

// get download files
// const downloadFiles = await getFiles('downloads')

// get desktop files
// const desktopFiles = await getFiles('desktop')

// async function getFiles() {
//   // const files = await readdir('C:\\Users\\user\\Desktop')
//   // console.log(files)
// }



const quickAccessTabs: QuickAccessTab[] = [
  {
    name: 'Images',
    icon: <PhotoIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#6166fe',
    // action: getImageFiles
  },
  {
    name: 'Music',
    icon: <MusicalNoteIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#6166fe'

  },
  {
    name: 'Videos',
    icon: <PlayIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#3074f5'
  },
  {
    name: 'Documents',
    icon: <Bars3BottomLeftIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#3074f5'

  },
  {
    name: 'Downloads',
    icon: <ArrowDownIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
    color: '#22244a'
  },
  {
    name: 'Desktop',
    icon: <ComputerDesktopIcon className='rounded-full my-4 mx-2 flex w-[47.5px]  dark:text-shilo-300 text-gray-100' />,
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
        <ul className='flex flex-wrap gap-10 items-center justify-start mt-4'>
          {quickAccessTabs.map((tab, index) => (
            <li key={index} className='flex flex-col items-center justify-center w-20 h-20'>
              <a href={'quick-access/' + tab.name.toLowerCase()} className='rounded-[20px] shadow-md dark:shadow-none shadow-gray-400  px-3' style={{
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

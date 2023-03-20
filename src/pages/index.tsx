import AppLogo from '@/components/AppLogo'
import { Cog8ToothIcon, HomeIcon, FolderOpenIcon, WifiIcon, ArrowRightOnRectangleIcon } from '@heroicons/react/24/outline'

interface Route {
  path: string,
  component: React.FC
  icon: any
}
// the routes
const routes: Route[] = [{
  path: '/',
  component: Home,
  icon: <HomeIcon />
},
{
  path: '/files',
  component: Home,
  icon: <FolderOpenIcon />
},
{
  path: '/wifi',
  component: Home,
  icon: <WifiIcon />
},
{
  path: '/settings',
  component: Home,
  icon: <Cog8ToothIcon />
},
{
  path: '/settings',
  component: Home,
  icon: <ArrowRightOnRectangleIcon />
}
]
export default function Home() {
  return (
    <div className='grid grid-cols-12 h-full' id='container' style={
      {
        height: '100vh',
      }
    }>
      <nav className='col-span-1 bg-[#fafbfd] dark:bg-[#1a1b1b] pt-4'>
        <AppLogo />
        <ul className=' h-full flex flex-col items-center'>
          {routes.map((route, index) => (
            <li key={index} className='w-6 h-6 my-5 first:mt-10 last:mt-auto last:mb-20 text-gray-500 dark:text-gray-300'>
              <a href={route.path}>
                <span className='sr-only'>
                  {route.path}
                </span>
                {route.icon}
              </a>
            </li>
          ))}
        </ul>
      </nav>
      <main className='col-span-7 pt-4  dark:bg-[#1f2222] '>
       
      </main>
      <aside className='col-span-4 pt-4  bg-[#fafbfd] dark:bg-[#1a1b1b] dark:tet-gray-400'>
      </aside>
    </div>
  )
}

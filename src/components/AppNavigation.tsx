// import Home from '@/pages/home'
import { Cog8ToothIcon, HomeIcon, ArrowsRightLeftIcon, FolderOpenIcon, InformationCircleIcon, ShareIcon, ClockIcon, SignalIcon } from '@heroicons/react/24/outline';
import { Cog8ToothIcon as SolidCog8ToothIcon, HomeIcon as SolidHomeIcon, FolderOpenIcon as SolidFolderIconOpen, InformationCircleIcon as SolidInformationIcon, ShareIcon as SolidShareIcon, SignalIcon as SolidSignalIcon, ClockIcon as SolidClockIcon } from '@heroicons/react/24/solid'
import { DialogFilter, message, ask } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { goToPage as gotoPage } from '@/utils';
import { Fragment, useEffect, useState } from 'react'




/**
 * @function openFileManager - opens a file manager
 * @returns {Array<Files>} an array of selected files 
 */
async function openFileManager()/* : Array<Files> */ {
    // Open a selection dialog for directories
    const selected = await open({
        directory: true,
        multiple: true,
        filters: allowedExtension
        // defaultPath: await appDir(),
    }).catch((err) => {
        message('error opening file manager', {
            title: 'Access error',
            type: 'error'
        }).then((result) => {
            console.log(result)
        })
    });
    if (Array.isArray(selected)) {
        // user selected multiple directories
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        // user selected a single directory
    }
}

// allowed file extension 
const allowedExtension: DialogFilter[] = [{ name: 'image', extensions: ['ai', 'dxf', 'odg', 'fodg', 'svg', 'svgz', 'bmp', 'gif', 'ico', 'jpg', 'jpeg', 'png', 'psd', 'pdd', 'tga', 'tiff', 'xcf', 'xpm'] },
{ name: 'audio', extensions: ['au', 'aif', 'aifc', 'aiff', 'wav', 'flac', 'la', 'pac', 'm4a', 'ape', 'wv', 'wma', 'ast', 'mp2', 'mp3', 'spx', 'aac', 'mpc', 'ra', 'ogg', 'mid', 'm3u', 'pls'] },
{ name: 'pdf', extensions: ['pdf', 'ps'] },
{ name: 'video', extensions: ['webm', 'mkv', 'flv', 'vob', 'ogv', 'drc', 'avi', 'mov', 'qt', 'wmv', 'rm', 'rmvb', 'asf', 'mp4', 'm4p', 'm4v', 'mpg', 'mpeg', 'mpe', 'mpv', '3gp', '3g2', 'mxf', 'aff', 'm2ts', 'mts'] },
{ name: 'powerpoint', extensions: ['ppt', 'pot', 'pps', 'pptx', 'pptm', 'potx', 'potm', 'ppam', 'ppsx', 'ppsm', 'sldx', 'sldm', 'odp', 'fodp', 'otp'] },
{ name: 'word', extensions: ['doc', 'dot', 'docx', 'docm', 'dotx', 'dotm', 'docb', 'odt', 'fodt', 'ott'] },
{ name: 'excel', extensions: ['xls', 'xlt', 'xlm', 'xlsx', 'xlsm', 'xltx', 'xltm', 'xla', 'xlam', 'ods', 'fods', 'ots'] },
{ name: 'xml', extensions: ['xml', 'xslt', 'html', 'xhtml', 'htm'] },
{
    name: 'delimited', extensions: ['csv']
},
{ name: 'document', extensions: ['txt', 'rtf', 'c', 'h', 'cpp', 'hpp', 'cxx', 'hxx', 'java', 'js', 'rb', 'py', 'cs', 'm', 'sh', 'php', 'css', 'go'] }]

interface Route {
    icon: any, // the route icon
    name: string, // the route name
    alternateIcon: any, // the icon to show on hover or active state
    action?: () => any // action that will be executed when the route is clicked
    path?: string, // the path string
}
// the port on which th application urn for the sender PC 
interface SenderProps {
    port: number
}

const ProgressComponent = () => {
    return (
        <>
            <div className="flex justify-between mb-1">
                <span className="text-base font-medium text-blue-700 dark:text-white">Flowbite</span>
                <span className="text-sm font-medium text-blue-700 dark:text-white">45%</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                <div className="bg-blue-600 h-2.5 rounded-full" style={{ width: '45%' }}></div>
            </div>
        </>
    );
};



interface SystemInformation {
    /// the current user name eg - drizzle
    systemName: string,
    /// available store
    freeMemory: string,
    /// the port on which the core server runs
    port: number,
    /// the uptime e.g 2 hours     
    uptime: string,
}



export default function AppNavigation() {
    let [systemInformation, setSystemInformation] = useState({} as SystemInformation);

    useEffect(() => {
        invoke('get_system_information').then((sysInfo) => {
            setSystemInformation((sysInfo as any).data)
        })
    })

    const routes: Route[] = [{
        path: '/',
        icon: <HomeIcon className='w-6 h-6' />,
        name: 'home',
        alternateIcon: <SolidHomeIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "settings" })

    },
    {
        icon: <SignalIcon className='w-6 h-6' />,
        name: 'Connection',
        alternateIcon: <SolidSignalIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "settings" })

    },
    {
        path: '/share',
        icon: <ClockIcon className='w-6 h-6' />,
        name: 'History',
        alternateIcon: <SolidClockIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "settings" })

    },
    {
        path: '/files',
        icon: <FolderOpenIcon className='w-6 h-6' />,
        action: openFileManager,
        alternateIcon: <SolidFolderIconOpen className='w-6 h-6' />,
        name: 'File Manager'
    },

    {
        path: '/share',
        icon: <ShareIcon className='w-6 h-6' />,
        name: 'Shared files',
        alternateIcon: <SolidShareIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "settings" })

    },
    {
        path: '/settings',
        icon: <Cog8ToothIcon className='w-6 h-6' />,
        alternateIcon: <SolidCog8ToothIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "settings" }),
        name: 'settings'
    },
    {
        path: '/help',
        icon: <InformationCircleIcon className='w-6 h-6' />,
        alternateIcon: <SolidInformationIcon className='w-6 h-6' />,
        action: () => gotoPage({ routePath: "help" }),
        name: 'help'

    },
    ]

    return (
        <>
            <nav className='col-span-1 lg:col-span-2 bg-[rgba(249,250,254,255)]  px-auto   text-gray-600  pt-10' style={
                {
                    height: "calc(100vh-200px)",
                    overflowY: "hidden"
                }
            }>
                <ul className=' h-full flex flex-col px-4 pl-6'>
                    {
                        /**
                         * show the icon and the name side by side on full screen mode 
                         * otherwise, hide the name and show the icons only 
                         */
                    }
                    {routes.map((route, index) => (
                        <li key={index} className='flex h-6 my-8 lg:my-8 first:mt-10 last:mb-20 text-app-500 cursor-pointer'>
                            <span onClick={route.action /**action to perform on route clicked */} className='cursor-pointer'>
                                <span className='sr-only'>
                                    {route.path}
                                </span>
                                <div className='gap-2 justify-center align-center flex capitalize'>
                                    {route.icon /**the route icon */}
                                    <span className='hidden lg:block' id='route__name'>
                                        {route.name /** the route name */}
                                    </span>
                                </div>
                            </span>
                        </li>
                    ))}
                </ul>
                <ProgressComponent />
            </nav>
        </>
    )
}



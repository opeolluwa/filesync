// import Home from '@/pages/home'
import { Cog8ToothIcon, HomeIcon, FolderOpenIcon, InformationCircleIcon, } from '@heroicons/react/24/outline'
import AppLogo from './AppLogo'
import { DialogFilter, message, ask } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import HostSpotIcon from './icons/HostSpotIcon';
import { goToPage as gotoPage } from '@/utils';
import { Dialog, Transition } from '@headlessui/react'
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
    path: string, // the path string
    icon: any, // the route icon
    action?: () => any // action that will be executed when the route is clicked
}
// the port on which th application urn for the sender PC 
interface SenderProps {
    port: number
}
function SendConfig({ port }: SenderProps) {
    return (
        <div className="text-2xl font-mono my-2 text-center text-gray-600">
            <strong className='text-bold'>{port}</strong>
        </div>
    )
}

//
function ReceiveConfig() {
    return (
        <div className="h-full">
            <form action="">
                <div className="flex flex-col align-center justify-center mx-8">
                    <label htmlFor="connectionID " className="text-gray-600 sr-only">connection ID</label>
                    <input type="text" maxLength={6} name="connectionID" placeholder='enter connection ID' id="connectionID" className="border-2 placeholder:text-small w-3/5 mx-auto border-gray-300 rounded-md p-2 my-2" />
                </div>

                <button className='hidden'>
                    Connect
                </button>
            </form>

        </div>
    )
}


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
    let [isModalOpen, setModalState] = useState(false)
    let [systemInformation, setSystemInformation] = useState({} as SystemInformation);

    const closeModal = () => setModalState(false)
    const openModal = () => setModalState(true)

    let [showSendConfig, setSendConfig] = useState(false);
    let [showReceiveConfig, setReceiveConfig] = useState(true);

    const showSendComponent = () => { setSendConfig(true); setReceiveConfig(false); /* setModalState(false) */ }
    const showReceiveComponent = () => { setReceiveConfig(true); setSendConfig(false);/*  setModalState(false) */ }

    useEffect(() => {
        invoke('get_system_information').then((sysInfo) => {
            setSystemInformation((sysInfo as any).data)
        })
    })

    const routes: Route[] = [{
        path: '/',
        icon: <HomeIcon />,
        action: () => gotoPage({ routePath: "settings" })

    },
    {
        path: '/files',
        icon: <FolderOpenIcon />,
        action: openFileManager
    },
    {
        path: '/wifi',
        icon: < HostSpotIcon />,
        action: openModal
    },
    {
        path: '/settings',
        icon: <Cog8ToothIcon />,
        action: () => gotoPage({ routePath: "settings" })
    },
    {
        path: '/help',
        icon: <InformationCircleIcon />,
        action: () => gotoPage({ routePath: "help" })

    },
    ]

    return (
        <>


            <Transition appear show={isModalOpen} as={Fragment}>
                <Dialog as="div" className="relative z-10" onClose={closeModal}>
                    <Transition.Child
                        as={Fragment}
                        enter="ease-out duration-300"
                        enterFrom="opacity-0"
                        enterTo="opacity-100"
                        leave="ease-in duration-200"
                        leaveFrom="opacity-100"
                        leaveTo="opacity-0"
                    >
                        <div className="fixed inset-0 bg-black bg-opacity-50" />
                    </Transition.Child>

                    <div className="fixed inset-0 overflow-y-auto py-10">
                        <div className="flex min-h-full items-center justify-center p-4 text-center">
                            <Transition.Child
                                as={Fragment}
                                enter="ease-out duration-300"
                                enterFrom="opacity-0 scale-95"
                                enterTo="opacity-100 scale-100"
                                leave="ease-in duration-200"
                                leaveFrom="opacity-100 scale-100"
                                leaveTo="opacity-0 scale-95"
                            >
                                <Dialog.Panel className="w-full dark:bg-gray-200  max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all mb-8">
                                    <Dialog.Title
                                        as="h3"
                                        className="text-sm text-gray-500 text-center"
                                    >

                                        {showSendConfig ? "Input the connection ID below in the recipient computer" : showReceiveConfig ? "Provide connection  ID shown on the host computer" : "Select transfer mode"}
                                    </Dialog.Title>
                                    <div className="mt-6 ">

                                        {
                                            showSendConfig && <SendConfig port={
                                                systemInformation.port
                                            } />
                                        }
                                        {
                                            showReceiveConfig && <ReceiveConfig />
                                        }
                                        <div className="text-sm flex justify-center gap-5 text-gray-500 mt-6">
                                            <button
                                                type="button"
                                                className="inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500"
                                                onClick={showSendComponent}
                                            >
                                                Send files
                                            </button>

                                            <button
                                                type="button"
                                                className="inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500"
                                                onClick={showReceiveComponent}
                                            >
                                                receive files
                                            </button>

                                        </div>
                                    </div>
                                </Dialog.Panel>
                            </Transition.Child>
                        </div>
                    </div>
                </Dialog>
            </Transition >


            <nav className='col-span-2 bg-[rgba(249,250,254,255)] dark:text-shilo-500 px-auto  dark:border-r-mirage-xx-800 dark:border-r text-gray-600  dark:bg-mirage-600 pt-10' style={
                {
                    height: "calc(100vh-200px)",
                    overflowY: "hidden"
                }
            }>
                {<AppLogo />}
                <ul className=' h-full flex flex-col items-center'>
                    {routes.map((route, index) => (
                        <li key={index} className='w-6 h-6 my-6 first:mt-10 last:mt-auto last:mb-20 text-app-500 cursor-pointer'>
                            <span onClick={route.action} className='cursor-pointer'>
                                <span className='sr-only'>
                                    {route.path}
                                </span>
                                {route.icon}
                            </span>
                        </li>
                    ))}
                </ul>
            </nav>
        </>
    )
}



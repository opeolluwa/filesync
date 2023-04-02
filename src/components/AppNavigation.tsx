import Home from '@/pages/home'
import { Cog8ToothIcon, HomeIcon, FolderOpenIcon, InformationCircleIcon } from '@heroicons/react/24/outline'
import AppLogo from './AppLogo'
import Link from 'next/link'
import { DialogFilter, message } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/dialog';
import { appDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';
import HostSpotIcon from './icons/HostSpotIcon';


// give off connection details 
async function promptConnection() {
    invoke('get_ip_address').then((ipAddr) => {
        message('connect to ' + ipAddr, {
            title: 'Connection',
            type: 'info'
        }).then((result) => {
            console.log(result)
        })
    })

}


// @function openFileManager
async function openFileManager() {
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


// allowed file extendion 
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
// the routes
const routes: Route[] = [{
    path: '/',
    icon: <HomeIcon />
},
{
    path: '/files',
    icon: <FolderOpenIcon />,
    action: openFileManager
},
{
    path: '/wifi',
    icon: < HostSpotIcon />,
    action: promptConnection
},
{
    path: '/settings',
    icon: <Cog8ToothIcon />
},
{
    path: '/help',
    icon: <InformationCircleIcon />
},
    // {
    //     path: '/settings',
    //     component: Home,
    //     icon: <ArrowRightOnRectangleIcon />
    // },

]


export default function Nav() {
    return (
        <nav className='col-span-1 bg-[rgba(249,250,254,255)] dark:text-shilo-500   dark:border-r-mirage-xx-800 dark:border-r text-gray-600  dark:bg-mirage-600 pt-10' style={
            {
                height: "calc(100vh-200px)",
                overflowY: "hidden"
            }
        }>
            <AppLogo />
            <ul className=' h-full flex flex-col items-center'>
                {routes.map((route, index) => (
                    <li key={index} className='w-6 h-6 my-5 first:mt-10 last:mt-auto last:mb-20 text-app-500'>
                        <Link href={'#'} onClick={route.action}>
                            <span className='sr-only'>
                                {route.path}
                            </span>
                            <span>
                            </span>
                            {route.icon}
                        </Link>
                    </li>
                ))}
            </ul>
        </nav>
    )
}

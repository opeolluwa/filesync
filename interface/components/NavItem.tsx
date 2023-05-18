import Link from 'next/link'
import React from 'react'

interface Props {
    path: React.ReactNode, // the path string
    icon: React.ReactNode, // the route icon
    action?: () => any
}
export default function NavItem({ action, icon, path }: Props) {
    return (
        <>
            <li  className='w-6 h-6 my-5 first:mt-10 last:mt-auto last:mb-20 text-app-500'>
                {/* <Link href={route.path}> */}
                <Link href={'#'} onClick={action}>

                    <span className='sr-only'>
                        {path}
                    </span>
                    {icon}
                </Link>
            </li>
        </>
    )
}

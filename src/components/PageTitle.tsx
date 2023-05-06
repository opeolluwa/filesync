import React from 'react'

interface Props {
    title: string
}
export default function PageTitle({ title }: Props) {
    return (
        <h2 className='mt-20 mb-8  text-2xl font-bold text-gray-700 dark:text-gray-300'>
            {title}
        </h2>
    )
}

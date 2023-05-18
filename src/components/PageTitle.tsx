import React from 'react'

interface Props {
    title: string
}
export default function PageTitle({ title }: Props) {
    return (
        <h2 className='mt-20 mb-8  font-bold text-gray-700 dark:text-gray-300'>
            {title}
        </h2>
    )
}

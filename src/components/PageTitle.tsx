import React from 'react'

interface Props {
    title: string
}
export default function PageTitle({ title }: Props) {
    return (
        <h2 className='font-bold  dark:text-gray-400 flex items-center justify-between mb-10'>
            {title}

        </h2>
      /*   <h2 className='mt-20 mb-8  font-bold text-gray-700 dark:text-gray-300'>
            {title}
        </h2> */
    )
}

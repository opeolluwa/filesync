interface Props {
    title: string
}
export default function PageTitle({ title }: Props) {
    return (
        <h2 className='font-bold  dark:text-gray-400 flex items-center justify-between mb-10'>
            {title}
        </h2>
    )
}

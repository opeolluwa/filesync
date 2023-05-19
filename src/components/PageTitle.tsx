interface Props {
    title: string
}
export default function PageTitle({ title }: Props) {
    return (
        <h2 className='font-bold  dark:text-gray-400 flex items-center justify-between mb-12'>
            {title}
        </h2>
    )
}

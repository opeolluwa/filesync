import FileCard, { FileInterface } from "./FileCard";


const sampleFIles: FileInterface[] = [
    {
        fileType: "png",
        fileName: "LeonaMillie.mp4.",
        fileSize: 235809,
        downloadInProgress: true
    },
    {
        fileType: "mp4",
        fileName: "EdwardBenjamin.mp4",
        fileSize: 2794089,
        downloadInProgress: true
    },
    {
        fileType: "xls",
        fileName: "IreneJennie.xls",
        fileSize: 57889,
        downloadInProgress: true
    },
    {
        fileType: "docs",
        fileName: "memo.docx",
        fileSize: 52146,
        downloadInProgress: false
    },
    {
        fileType: "exe",
        fileName: "FloraMabel.exe",
        fileSize: 24790181,
        downloadInProgress: false
    }
]
export default function () {
    return (
        <aside className='col-span-4 pt-10 px-8  bg-[rgba(226,233,252,255)] dark:bg-[#1a1b1b] dark:tet-gray-400' >
            <h2 className='font-medium dark:text-gray-400 flex items-center justify-between mb-10'>
                Downloads
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-8 h-8 font-extrabold">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM12.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM18.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0z" />
                </svg>
            </h2>
            {
                /**
                 * use state management to display files here
                 * a procedure to determine the file type too and the right file icon should be aded
                 */
            }
            {
                sampleFIles.map((file, index) => (
                    <FileCard key={index} fileType={file.fileType} fileName={file.fileName} fileSize={file.fileSize} downloadInProgress={file.downloadInProgress} />
                ))
            }

        </aside>
    )
}
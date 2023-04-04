import FileCard, { FileInterface } from "./FileCard";


const sampleFIles: FileInterface[] = [
  /*   {
        fileType: "png",
        fileName: "LeonaMillie.mp4.",
        fileSize: 235809,
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
    } */
]
export default function Aside() {
    return (
        <aside className='col-span-4 pt-10 px-8  bg-[rgba(226,233,252,255)]   dark:border-l-mirage-x-700 dark:border-l dark:bg-mirage-600' >
            <h2 className='font-medium dark:text-gray-400 flex items-center justify-between mb-10'>
                Received Files

            </h2>
            {
                /**
                 * use state management to display files here
                 * a procedure to determine the file type too and the right file icon should be aded
                 */
            }
            {
                sampleFIles.length == 0 ? <p className='text-gray-500 dark:text-gray-400'>No files received yet</p> : sampleFIles.map((file, index) => (
                    <FileCard key={index} fileType={file.fileType} fileName={file.fileName} fileSize={file.fileSize} downloadInProgress={file.downloadInProgress} />
                ))
            }

            <h2 className='font-medium mt-12 dark:text-gray-400 flex items-center justify-between mb-10'>
                Sent Files

            </h2>
            {
                /**
                 * use state management to display files here
                 * a procedure to determine the file type too and the right file icon should be aded
                 */
            }
            {
                sampleFIles.length == 0 ? <p className='text-gray-500 dark:text-gray-400'>No files received yet</p> : sampleFIles.map((file, index) => (
                    <FileCard key={index} fileType={file.fileType} fileName={file.fileName} fileSize={file.fileSize} downloadInProgress={file.downloadInProgress} />
                ))
            }

        </aside>
    )
}
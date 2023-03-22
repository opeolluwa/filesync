

// the structure of data tha will be returned from the application core
export interface AppData<T> {
    data: T,
    message: string,
    status: boolean
}

// to interface with audio files coming from the application core
// the type extends the AppData type 
export interface AudioFile {
    fileName: string,
    fileFormat: string,
    fileSize: number,
    filePath: string
}
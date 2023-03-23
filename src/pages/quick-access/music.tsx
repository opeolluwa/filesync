import MusicFile from '@/components/MusicFile'
import QuickAccessLayout from '@/components/QuickAccessLayout'
import SearchBar from '@/components/SearchBar';
import { AppData, AudioFile } from '@/types';
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';


export default function Music() {
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke('fetch_audio_files').then((res) => {
      setData(res as any);
      setLoading(false);
    });
  }, []);

  // typecast the response into AppData type
  const musicData = data as unknown as AppData<Array<AudioFile>>;
  if (isLoading) {
    return (<h2>fetch you audio files</h2>)
  }
  return (
    <QuickAccessLayout pageTitle={'Music'}>
      <div>
        <SearchBar onSearch={function (city: string): void {
          throw new Error('Function not implemented.');
        }} />
        <div className='flex flex-wrap  flex-grow gap-4 justify-start mt-12'>
          {musicData?.data.map((file, index) => (
            <MusicFile
              key={index}
              fileName={file.fileName}
              fileSize={file.fileSize} fileFormat={file.fileFormat} filePath={file.filePath} />
          ))}
        </div>
      </div>
    </QuickAccessLayout>
  )
}

import MusicFile from '@/components/MusicFile'
import QuickAccessLayout from '@/components/QuickAccessLayout'
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

  useEffect(() => {
    invoke('greet', { name: 'World geng' })
      .then(console.log)
      .catch(console.error)
  }, []);

  // typecast the response into AppData type
  const __music__data__ = data as unknown as AppData<Array<AudioFile>>;
  if (isLoading) {
    return (<h2>fetch you audio files</h2>)
  }
  return (
    <QuickAccessLayout pageTitle={'Music'}>
      <div>
        <div className='flex flex-wrap gap-4'>

          {__music__data__?.data.map((file, index) => (
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

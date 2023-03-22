import MusicFile from '@/components/MusicFile'
import QuickAccessLayout from '@/components/QuickAccessLayout'
import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';
import { AppData } from '..';
// import { AppData } from '..';


// async function getMusicFiles() {
//   return typeof window !== 'undefined' && await invoke('fetch_audio_files')
// }

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
  const __music__data__ = data as unknown as AppData;
  if (isLoading) {
    return (<h2>fetch you audio files</h2>)
  }

  let fetchedData = Array(__music__data__?.data)
  let rr =
    fetchedData.map((file: any) => {
      return (
        <MusicFile
          key={file.id}
          name={file.name}
          size={file.size} format={file.name.split('.')[1]} duration={0} />
      )
    });

  return (
    <QuickAccessLayout pageTitle={'Music'}>
      {Boolean(__music__data__?.status) == true ? rr : "nay"}
    </QuickAccessLayout>
  )
}

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
  const __music__data__: AppData = data;

  // render the files
  if (isLoading) {
    return (<h2>fetch you audi files</h2>)
  }
  return (
    <QuickAccessLayout pageTitle={'Music'}>
      {JSON.stringify(__music__data__)}
      {/* <MusicFile name={'dax-alchhol.pm4'} format={"avi"} duration={0} size={0} /> */}
    </QuickAccessLayout>
  )
}

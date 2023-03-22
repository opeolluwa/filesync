import MusicFile from '@/components/MusicFile'
import QuickAccessLayout from '@/components/QuickAccessLayout'

export default function Images() {
  return (
    <QuickAccessLayout pageTitle={'Music'}>
      <div>
        <MusicFile name={'dax-alchhol.pm4'} format={"avi"} duration={0} size={0} />
      </div>
    </QuickAccessLayout>
  )
}

"use client"
import Image from 'next/image';
import View from '../View';

interface Props {
  className?: string;
}
export default function AppIcon({className}:Props) {
  return (
    <>
      <View className={"flex items-center justify-left " + className}>
        <Image
          src="/assets/app-icon.png"
          alt={"filesync Logo"}
          className="w-[45px]"
        ></Image>{" "}
       
      </View>
    </>
  );
}

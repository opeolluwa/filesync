import React from 'react'
import Heading from '../Heading';
import View from '../View';

interface Props {
  className?: string;
}
export default function AppIcon({className}:Props) {
  return (
    <>
      <View className={"flex items-center justify-left " + className}>
        <img
          src="/assets/app-icon.png"
          alt={"filesync Logo"}
          className="w-[45px]"
        ></img>{" "}
       
      </View>
    </>
  );
}

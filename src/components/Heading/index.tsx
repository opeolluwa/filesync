import React, { Children } from "react";
interface Props {
  children?: React.ReactNode;
  context?: string;
  className?: string;
}
export default function Heading({ children, className, context }: Props) {
  return (
    <h2 className={"font-bold  text-3xl leading-loose" + className}>
      {context || children}
    </h2>
  );
}

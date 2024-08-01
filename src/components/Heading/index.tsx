import React, { Children } from "react";
interface Props {
  children?: React.ReactNode;
  context?: string;
  className?: string;
}
export default function Heading({ children, className, context }: Props) {
  return (
    <h2 className={"font-semibold text-2xl " + className}>
      {context || children}
    </h2>
  );
}

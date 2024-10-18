import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  context?: string;
}
export default function SmallText({ children, className, context }: Props) {
  return (
    <p className={"leading-5 text-sm text-inherit dark:text-dark-500 " + className}>
      {context || children}
    </p>
  );
}

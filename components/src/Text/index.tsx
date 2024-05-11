import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
}
export default function Text({ children, className }: Props) {
  return (
    <p className={"leading-5 text-gray-500 dark:text-dark-500 " + className}>
      {children}
    </p>
  );
}

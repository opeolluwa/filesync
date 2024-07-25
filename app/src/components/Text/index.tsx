import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  context?: string;
}
export default function Text({ children, className , context}: Props) {
  return (
    <p className={"leading-5 text-gray-500 dark:text-dark-500 " + className}>
      {context || children}
    </p>
  );
}

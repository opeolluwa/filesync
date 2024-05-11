import React from "react";
interface Props {
  children?: React.ReactNode;
  className?: string;
  onclick?: () => void;
}

export default function Button({ children, className, onclick }: Props) {
  return (
    <button className={"rounded px-4 py-3 " + className} onClick={onclick}>
      {children}
    </button>
  );
}

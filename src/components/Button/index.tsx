interface Props {
  children?: React.ReactNode;
  className?: string;
}

export default function Button({ children, className }: Props) {
  return <button className={"rounded px-4 py-3 " + className}>{children}</button>;
}

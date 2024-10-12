interface Props {
  children?: React.ReactNode;
  className?: string;
  href?: string;
}

export default function Button({ children, className, href }: Props) {
  if (href) {
    return (
      <a href={href} className={"rounded px-4 py-3 " + className}>
        {children}
      </a>
    );
  }
  return (
    <button className={"rounded px-4 py-3 " + className}>{children}</button>
  );
}

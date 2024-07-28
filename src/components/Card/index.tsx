interface CardProps {
  style?: React.CSSProperties;
  children?: React.ReactNode;
  className?: string;
}

export default function Card({ style, children, className }: CardProps) {
  return (
    <>
      <div className={"p-4 px-2 rounded-lg " + className} style={style}>
        {children}
      </div>
    </>
  );
}

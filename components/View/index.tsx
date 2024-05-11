interface CardProps {
  style?: React.CSSProperties;
  children?: React.ReactNode;
  className?: string;
}

export default function View({ style, children, className }: CardProps) {
  return (
    <>
      <div className={" "+className} style={style}>
        {children}
      </div>
    </>
  );
}

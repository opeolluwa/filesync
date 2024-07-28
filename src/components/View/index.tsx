interface CardProps {
  style?: React.CSSProperties;
  children?: React.ReactNode;
  className?: string;
  clickEvent?: () => void;
}

export default function View({
  style,
  children,
  className,
  clickEvent,
}: CardProps) {
  return (
    <>
      <div className={" " + className} style={style} onClick={clickEvent}>
        {children}
      </div>
    </>
  );
}

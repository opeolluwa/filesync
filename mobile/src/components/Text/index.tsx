interface Props {
  context: string;
  withStyle?: string;
}
export default function Text({ context, withStyle }: Props) {
  return (
    <p className={"leading-5 text-gray-500 dark:text-dark-500 " + withStyle}>
      {context}
    </p>
  );
}

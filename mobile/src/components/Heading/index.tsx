interface Props {
  context: string;
  withStyle?: string;
}
export default function Heading({ context, withStyle }: Props) {
  return (
    <h2
      className={"font-semibold text-gray-400 dark:text-dark-400 " + withStyle}
    >
      {context}
    </h2>
  );
}

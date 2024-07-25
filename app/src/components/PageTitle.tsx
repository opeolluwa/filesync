interface Props {
  title: string;
  styles?: string;
}
export default function PageTitle({ title, styles }: Props) {
  return (
    <h2
      className={
        "font-bold  dark:text-gray-400 flex items-center justify-between" +
        " " +
        styles
      }
    >
      {title}
    </h2>
  );
}

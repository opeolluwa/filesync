import { Head, Html, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className="dark:bg-dark-950">
        <Main />
        <NextScript />
        {/*   <Script src='@/utils/splashscreen.js' ></Script> */}
      </body>
    </Html>
  );
}

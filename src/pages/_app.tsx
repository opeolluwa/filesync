import Layout from "@/components/AppLayout";
import FileStore from "@/store/context";
import SystemInfoStore from "@/store/sys-info";
import "@/styles/globals.css";
import type { AppProps } from "next/app";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <SystemInfoStore>
      <FileStore>
        <Layout>
          <Component {...pageProps} />
        </Layout>
      </FileStore>
    </SystemInfoStore>
  );
}

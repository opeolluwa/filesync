import Layout from "@/components/app/AppLayout";
import FileStore from "@/store/context";
import SystemInfoStore from "@/store/sys-info";
import "@/styles/globals.css";
import type { AppProps } from "next/app";
// import WifiStatus from "@/store/wifi-status";
import "remixicon/fonts/remixicon.css";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <SystemInfoStore>
      <FileStore>
        {/* <WifiStatus> */}
          <Layout>
            <Component {...pageProps} />
          </Layout>
        {/* </WifiStatus> */}
      </FileStore>
    </SystemInfoStore>
  );
}

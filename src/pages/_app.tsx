import Layout from "@/components/app/AppLayout";
import FileStore from "@/store/context";
import SystemInfoStore from "@/store/sys-info";
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import WifiStatus from "@/store/wifi-status";
import "remixicon/fonts/remixicon.css";
import Platform from "@/store/platform";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <SystemInfoStore>
      <FileStore>
        <WifiStatus>
          <Platform>
            <Layout>
              <Component {...pageProps} />
            </Layout>
          </Platform>
        </WifiStatus>
      </FileStore>
    </SystemInfoStore>
  );
}

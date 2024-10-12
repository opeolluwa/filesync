import Layout from "@/components/app/AppLayout";
import FileStore from "@/store/context";
import SystemInfoStore from "@/store/system_information";
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import WifiStatus from "@/store/wifi-status";
import "remixicon/fonts/remixicon.css";
import DeviceInformationStore from "@/store/device";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <SystemInfoStore>
      <FileStore>
        <WifiStatus>
          <DeviceInformationStore>
            <Layout>
              <Component {...pageProps} />
            </Layout>
          </DeviceInformationStore>
        </WifiStatus>
      </FileStore>
    </SystemInfoStore>
  );
}

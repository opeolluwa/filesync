import Layout from "@/components/app/AppLayout";
import FileStore from "@/store/context";
import SystemInfoStore from "@/store/system_information";
import "@/styles/globals.css";
import type { AppProps } from "next/app";
import WifiStatus from "@/store/network";
import "remixicon/fonts/remixicon.css";
import DeviceInformationStore from "@/store/device";
import { ConfigProvider } from "antd";
import theme from "../theme/themeConfig";



export default function App({ Component, pageProps }: AppProps) {
  return (
    <SystemInfoStore>
      <FileStore>
        <WifiStatus>
          <DeviceInformationStore>
            <Layout>
              <ConfigProvider theme={theme}>
                <Component {...pageProps} />
              </ConfigProvider>
            </Layout>
          </DeviceInformationStore>
        </WifiStatus>
      </FileStore>
    </SystemInfoStore>
  );
}

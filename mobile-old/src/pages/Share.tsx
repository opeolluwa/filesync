import { Heading, Text, View } from "@filesync/components";
import { CloudArrowUpIcon } from "@heroicons/react/24/outline";
import { IonContent, IonPage } from "@ionic/react";
import type { UploadProps } from "antd";
import { message, Upload } from "antd";
import { useContext, useEffect, useState } from "react";
import { BASE_URL } from "../store/app";
import { IonFab, IonFabButton, IonIcon } from "@ionic/react";
import { add, paperPlane } from "ionicons/icons";
import { CapacitorHttp } from "@capacitor/core";
import { SystemInformation } from "@filesync/types/SystemInformation";

export enum FileTransferStatus {
  DOWNLOADING = "downloading",
  PAUSED = "paused",
  PENDING = "pending",
  UPLOADING = "uploading",
  COMPLETED = "done",
  ERROR = "error",
}

const Share: React.FC = () => {
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  // request options =
  const options = {
    url: BASE_URL + "/api/sys-info",
    headers: { "X-Fake-Header": "Fake-Value" }, //TODO: impl client without header
  };

  const getDataFromAPI = async () => {
    const response = await CapacitorHttp.get(options);
    return response.data;
  };

  useEffect(() => {
    async function loadData() {
      const loadedData = await getDataFromAPI();
      setSystemInformation(loadedData);
    }

    loadData();
  }, []);

  const { Dragger } = Upload;
  //   const { onUpdate } = useContext(FileContext);
  const serverAddress = BASE_URL + "/upload";

  const props: UploadProps = {
    name: "file",
    multiple: true,
    action: serverAddress,
    async onChange(info) {
      const { status } = info.file;
      if (status !== FileTransferStatus.UPLOADING) {
        // onUpdate(info.fileList);
      }
      if (status === FileTransferStatus.COMPLETED) {
        message.success(`${info.file.name} file uploaded successfully.`);
        // save the file to transfer history

        const fileName = info.file.name;
        const fileSize = String(info.file.size);
        const transactionType = "sent";


      } else if (status === FileTransferStatus.ERROR) {
        message.error(
          `${info.file.name} file upload failed. due to ${JSON.stringify(
            info.file.error
          )}`
        );
      }
    },
    onDrop(e) {
      console.log("Dropped files", e.dataTransfer.files);
    },
  };

  return (
    <>
      <IonPage id="main-content" className="bg-accent">
        <IonContent className="ion-padding">
          <View className="h-full w-full flex flex-col justify-center items-center">
            <Heading className="text-center text-2xl">Share Files</Heading>
            <Text className="text-gray-400 leading-5">
              Share files with other devices
            </Text>
            <Dragger className="h-[300px]   block mt-2" {...props}>
              <CloudArrowUpIcon className="text-app-300  text-center small w-20 mx-auto" />
              <p className="text-gray-400 leading-2">
                Drop files here to share or click to browse
              </p>
            </Dragger>
          </View>
          <View>hey there, the detail is {window.location.href}</View>

          <iframe
            className="w-full h-[500px]"
            src={BASE_URL + "/upload"}
          ></iframe>

          <iframe
            className="w-full h-[500px]"
            src='https://www.google.com'
          ></iframe>

          <Text className="text-white">
            the system name is {systemInformation.systemName} the server url is{" "}
            {systemInformation.serverBaseUrl}{" "}
            {JSON.stringify(systemInformation)}
          </Text>

          <IonFab vertical="bottom" horizontal="end" className="mr-8 mb-4">
            <IonFabButton>
              <IonIcon icon={paperPlane}></IonIcon>
            </IonFabButton>
          </IonFab>
        </IonContent>
      </IonPage>
    </>
  );
};

export default Share;

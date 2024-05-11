import { IonContent, IonPage } from "@ionic/react";
import Thumbnail from "../components/thumbnail";
import { File } from "@filesync/types/File";
import { Card, Heading, View } from "@filesync/components";


// ranon files with name, size, date and type which is either send or receive
const files: File[] = [
  {
    fileFormat: "pdf",
    fileName: "heheh.pdf",
    filePath: "/home.pdf",
    fileSize: "12MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "mp4",
    fileName: "example.docx",
    filePath: "/documents/example.docx",
    fileSize: "8MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "csv",
    fileName: "example.docx",
    filePath: "/documents/example.docx",
    fileSize: "8MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "docx",
    fileName: "example.docx",
    filePath: "/documents/example.docx",
    fileSize: "8MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "jpg",
    fileName: "image.jpg",
    filePath: "/pictures/image.jpg",
    fileSize: "3MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "mp3",
    fileName: "song.mp3",
    filePath: "/music/song.mp3",
    fileSize: "5MB",
    isFolder: false,
    isHidden: false,
  },
  {
    fileFormat: "txt",
    fileName: "notes.txt",
    filePath: "/documents/notes.txt",
    fileSize: "1MB",
    isFolder: false,
    isHidden: false,
  },
];
const TrasferHistory: React.FC = () => {
  return (
    <>
      <IonPage id="main-content" className="bg-accent">
        <IonContent className="ion-padding">
          
          <Card className="flex gap-4 mt-6">
            <Card className="w-1/2 rounded-lg bg-app h-[50px] text-white flex gap-2 items-center justify-start">
              <i className="ri-send-plane-line text-2xl text-white bg-app p-2 rounded-full"></i>
              <Heading context="Send" className="text-white" />
            </Card>

            <Card className="w-1/2 rounded-lg bg-app h-[50px] text-white flex gap-2 items-center justify-start">
              <i className="ri-download-cloud-line text-2xl text-white bg-app p-2 rounded-full"></i>
              <Heading context="Receive" className="text-white" />
            </Card>
          </Card>

         

          <Card className="mt-6">
            <View className="flex justify-between items-center">
              <Heading context="Recent Files" />
              <a href="/transfer-history">see all</a>
            </View>
            <View>
              {files.map((file, index) => (
                <Card
                  key={index}
                  className="flex justify-between items-center mt-1"
                >
                  <View className="flex gap-4">
                    <Thumbnail
                      fileName={file.fileName}
                      fileFormat={file.fileFormat}
                      filePath={file.filePath}
                      fileSize={file.fileSize}
                      isHidden={false}
                      isFolder={false}
                    ></Thumbnail>
                  </View>
                  <i className="ri-more-2-fill text-gray-400"></i>
                </Card>
              ))}
            </View>
          </Card>
        </IonContent>
      </IonPage>
    </>
  );
};

export default TrasferHistory;

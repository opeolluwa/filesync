import { IonContent, IonPage } from "@ionic/react";
import { File } from "@/filesync/types/File";
import Thumbnail from "../components/thumbnail";
import ExploreContainer from "../components/ExploreContainer";

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
const Home: React.FC = () => {
  return (
    <IonPage className="page">
      <IonContent fullscreen class="bg-accent">
        <ExploreContainer name={"Home Page"} />
      </IonContent>
    </IonPage>
  );
};

export default Home;

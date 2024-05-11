import { IonContent, IonPage } from "@ionic/react";
import { Barcode, BarcodeScanner } from "@capacitor-mlkit/barcode-scanning";
import { useEffect, useState } from "react";
import { Button, View, Text, Heading }  from "../../../components";


const History: React.FC = () => {
  const [cameraPermission, setCameraPermission] = useState<boolean>(false);

  useEffect(() => {
    const exec = () => {
      BarcodeScanner.requestPermissions().then((res) => {
        const { camera } = res;
        setCameraPermission(camera === "granted" || camera === "limited");
      });
    };
    exec();
  }, []);
  return (
    <IonPage className="page">
      <IonContent fullscreen class="bg-accent">
        <View className=" text-center  h-full flex flex-col justify-center items-center">
          <View>
            <Heading
              context="Connect to peer device"
              className="text-gray-800 mb-2 text-2xl capitalize font-sans"
            />
            <Text className="text-gray-400 leading-5 mb-8">
              Scan the QR code to connect to the peer device Lorem ipsum dolor
              sit
            </Text>
          </View>
          <View>
            <Button className="bg-app text-white w-full">
              Open Camera <i className="ri-camera-fill"></i>
            </Button>
          </View>
        </View>
        {/* <ExploreContainer name={"bb"}/> */}
      </IonContent>
    </IonPage>
  );
};

export default History;

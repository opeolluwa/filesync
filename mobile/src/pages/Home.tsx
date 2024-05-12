import { BarcodeScanner } from "@capacitor-mlkit/barcode-scanning";
import { IonContent, IonPage } from "@ionic/react";
import { useContext, useEffect, useState } from "react";
import { useHistory } from "react-router-dom";
import { Button, Heading, Text, View } from "../../../components";
import { BASE_URL, SystemInformation, SystemInformationContext } from "../store/app";
import { CapacitorHttp, HttpResponse } from "@capacitor/core";

const Home: React.FC = () => {
  const [cameraPermission, setCameraPermission] = useState<boolean>(false);
  const [cameraIsOpen, setCameraOpen] = useState<boolean>(false);
  const [ipAddress, setIpAddress] = useState<string>("");
  const history = useHistory();
  const { serverBaseUrl, systemName, port, availableDisk } = useContext(
    SystemInformationContext
  );

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
    return response.data
   }

  useEffect(() => {
    async function loadData() {
      const loadedData = await getDataFromAPI();
      console.log("hhhhhhhhhhhhhhhhwhjkasfbajkfb,alkbfd iljkdfa")
      setCameraPermission(true)
      console.log(JSON.stringify(loadData), {
        cameraPermission,
        serverBaseUrl,
        systemName,
        port,
        availableDisk,
      });
      setSystemInformation(loadedData);
    }

    loadData();
  }, []);




  const scanQrCode = async () => {
    // The camera is visible behind the WebView, so that you can customize the UI in the WebView.
    // However, this means that you have to hide all elements that should not be visible.
    // You can find an example in our demo repository.
    // In this case we set a class `barcode-scanner-active`, which then contains certain CSS rules for our app.
    setCameraOpen(true);
    // Add the `barcodeScanned` listener
    const listener = await BarcodeScanner.addListener(
      "barcodeScanned",
      async (result) => {
        if (result.barcode) {
          setIpAddress(result.barcode.rawValue);
          // Remove all listeners
          await BarcodeScanner.removeAllListeners();
          // Stop the barcode scanner
          await BarcodeScanner.stopScan();
          // Close the barcode scanner
          setCameraOpen(false);
          // go to the share page
          history.push("/share?ip=" + ipAddress);
        }
      }
    );

    // Start the barcode scanner
    await BarcodeScanner.startScan();
  };

  const requestCameraPermission = () => {
    BarcodeScanner.requestPermissions().then(async (res) => {
      const { camera } = await BarcodeScanner.requestPermissions();
      return camera;
    });
  };

  return (
    // hine this page if the camera is open

    <IonPage className={cameraIsOpen === false ? "page" : "hidden"}>
      <IonContent fullscreen class="bg-accent">
        <View className=" text-center  h-full flex flex-col justify-center items-center">
          <View>
            <Heading
              context="Connect to peer device"
              className="text-gray-800 mb-2 text-2xl capitalize font-sans"
            />
            <Text className="text-gray-400 leading-5 mb-8">
              Scan the QR code to connect to the peer device
            </Text>
           
          </View>
          <View>
            <Button
              className="bg-app-600 text-white w-full"
              onclick={scanQrCode}
            >
              Open Camera <i className="ri-camera-fill"></i>
            </Button>
          </View>
        </View>
      </IonContent>
    </IonPage>
  );
};

export default Home;

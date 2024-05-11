import {
  BarcodeScanner,
  BarcodeFormat,
  LensFacing,
} from "@capacitor-mlkit/barcode-scanning";

const startScan = async () => {
  // The camera is visible behind the WebView, so that you can customize the UI in the WebView.
  // However, this means that you have to hide all elements that should not be visible.
  // You can find an example in our demo repository.
  // In this case we set a class `barcode-scanner-active`, which then contains certain CSS rules for our app.
  document.querySelector("body")?.classList.add("barcode-scanner-active");

  // Add the `barcodeScanned` listener
  const listener = await BarcodeScanner.addListener(
    "barcodeScanned",
    async (result) => {
      console.log(result.barcode);
    }
  );

  // Start the barcode scanner
  await BarcodeScanner.startScan();
};

const stopScan = async () => {
  // Make all elements in the WebView visible again
  document.querySelector("body")?.classList.remove("barcode-scanner-active");

  // Remove all listeners
  await BarcodeScanner.removeAllListeners();

  // Stop the barcode scanner
  await BarcodeScanner.stopScan();
};

const scanSingleBarcode = async () => {
  return new Promise(async (resolve) => {
    document.querySelector("body")?.classList.add("barcode-scanner-active");

    const listener = await BarcodeScanner.addListener(
      "barcodeScanned",
      async (result) => {
        await listener.remove();
        document
          .querySelector("body")
          ?.classList.remove("barcode-scanner-active");
        await BarcodeScanner.stopScan();
        resolve(result.barcode);
      }
    );

    await BarcodeScanner.startScan();
  });
};

const scan = async () => {
  const { barcodes } = await BarcodeScanner.scan({
    formats: [BarcodeFormat.QrCode],
  });
  return barcodes;
};

const isSupported = async () => {
  const { supported } = await BarcodeScanner.isSupported();
  return supported;
};

const enableTorch = async () => {
  await BarcodeScanner.enableTorch();
};

const disableTorch = async () => {
  await BarcodeScanner.disableTorch();
};

const toggleTorch = async () => {
  await BarcodeScanner.toggleTorch();
};

const isTorchEnabled = async () => {
  const { enabled } = await BarcodeScanner.isTorchEnabled();
  return enabled;
};

const isTorchAvailable = async () => {
  const { available } = await BarcodeScanner.isTorchAvailable();
  return available;
};

const setZoomRatio = async () => {
  await BarcodeScanner.setZoomRatio({ zoomRatio: 0.5 });
};

const getZoomRatio = async () => {
  const { zoomRatio } = await BarcodeScanner.getZoomRatio();
  return zoomRatio;
};

const getMinZoomRatio = async () => {
  const { zoomRatio } = await BarcodeScanner.getMinZoomRatio();
  return zoomRatio;
};

const getMaxZoomRatio = async () => {
  const { zoomRatio } = await BarcodeScanner.getMaxZoomRatio();
  return zoomRatio;
};

const openSettings = async () => {
  await BarcodeScanner.openSettings();
};

const isGoogleBarcodeScannerModuleAvailable = async () => {
  const { available } =
    await BarcodeScanner.isGoogleBarcodeScannerModuleAvailable();
  return available;
};

const installGoogleBarcodeScannerModule = async () => {
  await BarcodeScanner.installGoogleBarcodeScannerModule();
};

const checkPermissions = async () => {
  const { camera } = await BarcodeScanner.checkPermissions();
  return camera;
};

const requestPermissions = async () => {
  const { camera } = await BarcodeScanner.requestPermissions();
  return camera;
};

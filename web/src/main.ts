import axios from "axios";
import Notifier from "awesome-notifications";

document.addEventListener("DOMContentLoaded", () => {
  const dropzone = document.getElementById("dropzone") as HTMLInputElement;
  const progressBar = document.querySelector("progress");
  const log = document.querySelector("output") as HTMLElement;

  const API_BASE_URL = window.location.host;
  const fileUploadEndpoint = `http://${API_BASE_URL}/upload`;
  const websocketEndpoint = `ws://localhost:8080`;

  //   const websocketEndpoint = `ws://${API_BASE_URL}/api/notify`;
  const websocketClient = new WebSocket(websocketEndpoint);
  websocketClient.addEventListener("open", () => {
    websocketClient.send("Connetion Successfully established");
  });
  websocketClient.addEventListener("message", (message) => {
    console.log("recieved new message from server", message);
  });
  websocketClient.addEventListener("close", () => {});


  // files
  http: dropzone.addEventListener("change", async () => {
    const { data } = await axios.post(
      fileUploadEndpoint,
      {
        files: dropzone?.files,
      },
      {
        headers: {
          "Content-Type": "multipart/form-data",
          Authorization: "Gwm7JViOHa0jqjeREuTWcCtCDNc",
        },
        onUploadProgress(progressEvent) {
          const { total, loaded } = progressEvent;
          console.log({ total, loaded });
        },
      }
    );
    console.log(JSON.stringify(data));
  });

  // notification
  // Set global options
  let globalOptions = {};
  // Initialize instance of AWN
  let notifier = new Notifier(globalOptions);

  // Set custom options for next call if needed, it will override globals
  let nextCallOptions = {};
  // Call one of available functions
  notifier.success("Your custom message", nextCallOptions);
});

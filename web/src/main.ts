import axios from "axios";
import "awesome-notifications";
import AWN from "awesome-notifications";
// Import as an ESM module
// import JSAlert from "js-alert";
// import "./tailwind.js";

document.addEventListener("DOMContentLoaded", () => {
  const dropzone = document.getElementById("dropzone") as HTMLInputElement;
  const progressBar = document.querySelector("#progress") as HTMLElement;
  const log = document.querySelector("output") as HTMLElement;

  // const API_BASE_URL = window.location.host;
  const API_BASE_URL = "192.168.0.170:18005"; ;

    const fileUploadEndpoint = `http://${API_BASE_URL}/upload`;
  // const fileUploadEndpoint = "http://192.168.0.170:18005/upload";
  // const websocketEndpoint = `ws://localhost:8080`;

    const websocketEndpoint = `ws://${API_BASE_URL}/notify`;
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
          const { loaded, total } = progressEvent;
          var percentCompleted = Math.round((loaded * 100) / Number(total));
          progressBar.style.width =`${percentCompleted}%`
          log.innerHTML = percentCompleted.toString();
        },
      }
    );
    console.log(JSON.stringify(data));
  });

  // notification
  let notifier = new AWN({});
  // Show an alert with a title and custom dismiss button
});

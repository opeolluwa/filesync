addEventListener("DOMContentLoaded", () => {
  const API_ENDPOINT = "http://192.168.0.170:18005/notify";

  const socket = new WebSocket(API_ENDPOINT);

  socket.addEventListener("open", function (event) {
    socket.send("Hello Server!");
  });

  socket.addEventListener("message", function (event) {
    /// thr messages that would be sent are file paths, once received, try to download the file
    console.log("Message from server ", event.data);
  });

  // setTimeout(() => {
  //   const obj = { hello: "world" };
  //   const blob = new Blob([JSON.stringify(obj, null, 2)], {
  //     type: "application/json",
  //   });
  //   console.log("Sending blob over websocket");
  //   socket.send(blob);
  // }, 1000);

  // setTimeout(() => {
  //   socket.send("About done here...");
  //   console.log("Sending close over websocket");
  //   socket.close(3000, "Crash and Burn!");
  // }, 3000);
});

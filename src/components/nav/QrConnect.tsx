import QRCode from "react-qr-code";

/**
 * @function QRConnect - display QR code in which URL  for connection is embedded
 * @param url - the core application URl
 * @param serverId, the serverId the application
 * @return UI
 */
export default function QRConnect({ url }: { url: string }) {
  return (
    <>
      <div
        style={{ background: "white", padding: "4px", height: "260px" }}
        className="flex flex-col items-center rounded"
      >
        <QRCode
          fgColor={"#1e293b"}
          value={url.trim()}
          style={{ boxSizing: "border-box", maxWidth: "150px" }}
        />
      </div>
    </>
  );
}

import QRConnect from "./QrConnect";

export default function ReceiveConfig({
  serverId,
  ipAddress,
}: {
  serverId: number;
  ipAddress: string;
}) {
  return (
    <div>
      <div>
        <QRConnect url={`http://${ipAddress.trim()}:${serverId}/`} />
      </div>
      <div className=" font-mono my-2 text-center text-gray-600">
        <strong>ID:</strong>
        {serverId}
      </div>
    </div>
  );
}

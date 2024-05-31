import { Colors } from "@/constants/Colors";
import { Feather } from "@expo/vector-icons";
import { Pressable, StyleSheet, Text, View, Image } from "react-native";
export default function HomeScreen() {
  return (
    <View
      style={{
        padding: 20,
        flex: 1,
        justifyContent: "center",
        // alignContent: "center",
        alignItems: "center",
      }}
    >
      {/* <Image
        source={require("../../assets/images/icons8-qrcode-64.png")}
        style={{ marginVertical: 30 }}
      /> */}
      <Text style={{ fontSize: 28, fontWeight: "500", marginBottom: 10 }}>
        Scan QR
      </Text>

      <Text
        style={{
          lineHeight: 20,
          fontSize: 16,
          color: Colors.gray,
          marginHorizontal: 10,
          textAlign: "center",
          paddingHorizontal: 25,
          marginBottom: 20,
        }}
      >
        Ready to get started? Open your camera app and scan the QR code below to
        connect.
      </Text>
      <Pressable style={{ paddingVertical: 12, marginBottom: 30 }}>
        <Text
          style={{
            color: Colors.accent,
            backgroundColor: Colors.app.DEFAULT,
            paddingVertical: 12,
            paddingHorizontal: 18,
            borderRadius: 7,
            fontWeight: 500,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            gap: 4,
          }}
        >
          Open Camera <Feather name="camera" size={20} />
        </Text>
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  titleContainer: {
    flexDirection: "row",
    alignItems: "center",
    gap: 8,
  },
  stepContainer: {
    gap: 8,
    marginBottom: 8,
  },
  reactLogo: {
    height: 178,
    width: 290,
    bottom: 0,
    left: 0,
    position: "absolute",
  },
});

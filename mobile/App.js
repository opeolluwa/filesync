import { StatusBar } from "expo-status-bar";
import { Button, StyleSheet, Text, View } from "react-native";
import "./global.css";
import theme from "./theme.json";
import { Header } from "./components/Header";

export default function App() {
  return (
    <View
      style={{
        height: "100%",
      }}
    >
      <Header />
      <View
        style={{
          height: "20%",
          width: "100%",
          backgroundColor: theme.app[50],
          padding: 20,
        }}
      >
        <View
          style={{
            flex: 1,
            alignItems: "flex-end",
            flexDirection: "row",
            justifyContent: "space-between",
          }}
        >
          <Text>Image</Text>
          <Text>Video</Text>
          <Text>Audio</Text>
          <Text>Document</Text>
        </View>
      </View>

      <Button
        title=""
        style={{ backgroundColor: theme.app[600], width: 20, height: 20 }}
      ></Button>
      <StatusBar style="auto" />
    </View>
  );
}

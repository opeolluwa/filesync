import { StatusBar } from "expo-status-bar";
import { StyleSheet, Text, View } from "react-native";
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
            alignItems: "center",
            flexDirection: "row",
            justifyContent: "space-between",
          }}
        ></View>
      </View>
      <StatusBar style="auto" />
    </View>
  );
}

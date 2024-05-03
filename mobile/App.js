import { StatusBar } from "expo-status-bar";
import { Button, Pressable, StyleSheet, Text, View } from "react-native";
import "./global.css";
import theme from "./theme.json";
import MaterialIcons from "@expo/vector-icons/MaterialIcons";
import Ionicons from "@expo/vector-icons/Ionicons";
import { useFonts } from "expo-font";

export default function App() {
  const [fontsLoaded] = useFonts({
    "Open-Sans": require("./assets/fonts/OpenSans_Condensed-Regular.ttf"),
  });

  return (
    <View
      style={{
        height: "100%",
        paddingHorizontal: 25,
      }}
    >
      <View
        style={{
          height: "10%",
          paddingTop: 50,
          width: "100%",
        }}
      >
        <View style={styles.flexContainerRow}>
          <MaterialIcons name="menu" size={24} color="black" />
          <Ionicons name="search-outline" size={24} color="black" />
        </View>
      </View>

      <Text
        style={{
          marginTop: 70,
          fontSize: 20,
          paddingLeft: 0,
          fontWeight: 500,
          lineHeight: 20,
        }}
      >
        Hello 👋
      </Text>
      <Text
        style={{
          marginTop: 2,
          fontSize: 28,
          paddingLeft: 0,
          fontWeight: 700,
        }}
      >
        Welcome Back
      </Text>

      {/* <View style={{...styles.flexContainerRow, marginVertical:100}}>
        <View style={{ ...styles.button, width: "40%", height: 100 }}></View>
        <View style={{ ...styles.button, width: "40%", height: 100 }}></View>
      </View> */}
      <StatusBar style="auto" />
    </View>
  );
}

const styles = StyleSheet.create({
  mediaTab: {
    borderRadius: 10,
    backgroundColor: "#fecaca",
    paddingHorizontal: 10,
    paddingVertical: 10,
    width: "45%",
    height: 50,
    display: "flex",
    alignItems: "center",
    flexDirection: "row",
  },
  button: {
    width: "100%",
    backgroundColor: "#6861D5",
    height: 150,
    borderRadius: 15,
    marginTop: 20,
    paddingVertical: 25,
    paddingHorizontal: 15,
    paddingBottom: 5,
    color: theme.white,
  },
  flexContainerRow: {
    display: "flex",
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
  },
});

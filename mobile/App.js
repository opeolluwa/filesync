import { StatusBar } from "expo-status-bar";
import {
  Button,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  View,
  FlatList,
  Image,
} from "react-native";
import "./global.css";
import theme from "./theme.json";
import MaterialIcons from "@expo/vector-icons/MaterialIcons";
import Ionicons from "@expo/vector-icons/Ionicons";
import { useFonts } from "expo-font";
import * as Progress from "react-native-progress";
import MenuIcon from "./assets/icons/menu.svg";

export default function App() {
  const [fontsLoaded, fontError] = useFonts({
    "open-sans": require("./assets/fonts/OpenSans_Condensed-Regular.ttf"),
  });

  return (
    <View
      style={{
        height: "100%",
        paddingHorizontal: 25,
        fontfamily: "open-sans",
        backgroundColor:theme.app[50]
      }}
    >
      <View
        style={{
          height: "10%",
          paddingTop: 50,
          width: "100%",
          display: "none",
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
          fontSize: 24,
          paddingLeft: 0,
          fontWeight: 700,
        }}
      >
        Welcome Back
      </Text>

      <View
        horizontal={true}
        showsHorizontalScrollIndicator={false}
        style={{
          ...styles.flexContainerRow,
          flexWrap: "nowrap",
          overflow: "scroll",
          marginTop: 25,
          marginVertical: 2,
          justifyContent: "space-between",
          gap: 15,
        }}
      >
        <View
          style={{
            ...styles.card,
            backgroundColor: theme.app.DEFAULT,
            width: "43%",
            height: 120,
          }}
        ></View>

        <View
          style={{
            ...styles.card,
            backgroundColor: theme.app.DEFAULT,
            width: "43%",
            height: 120,
          }}
        ></View>

        <View
          style={{
            ...styles.card,
            backgroundColor: theme.app.DEFAULT,
            width: "43%",
            height: 120,
          }}
        ></View>
        <View
          style={{
            ...styles.card,
            backgroundColor: theme.app.DEFAULT,
            width: "43%",
            height: 120,
          }}
        ></View>
      </View>
      <View
        style={{
          marginVertical: 50,
          borderColor: theme.gray[300],
          borderRadius: 18,
          borderWidth: 1,
          ...styles.container,
          height: 100,
          ...styles.flexContainerRow,
        }}
      >
        <View>
          <Text
            style={{
              fontWeight: 500,
              fontSize: 20,
            }}
          >
            680 MB / 1024 MB{" "}
          </Text>
          <Text style={{ color: theme.gray[400], fontSize: 18, marginTop: 6 }}>
            Available Storage{" "}
          </Text>
        </View>

        <Progress.Circle
          size={60}
          indeterminate={false}
          color={theme.app.DEFAULT}
          thickness={5}
          progress={0.67}
          unfilledColor={theme.app[100]}
          borderColor="transparent"
          direction="clockwise"
        />
      </View>

      <View style={{ ...styles.flexContainerRow }}>
        <Text
          style={{
            fontWeight: 500,
            fontSize: 20,
          }}
        >
          Recent Files
        </Text>
        <Text style={{ color: theme.gray[400], fontSize: 18, marginTop: 6 }}>
          See all
        </Text>
      </View>
      <View style={{ ...styles.container, paddingTop: 1, marginTop: 0 }}>
        <FlatList
          data={[
            { key: "Joel" },
            { key: "John" },
            { key: "Jillian" },
            { key: "Julie" },
          ]}
          renderItem={({ item }) => (
            <View
              style={{
                ...styles.flexContainerRow,
                columnGap: 5,
                justifyContent: "flex-start",
              }}
            >
              <View
                style={{
                  ...styles.card,
                  borderRadius: 5,
                  width: 40,
                  height: 40,
                  backgroundColor: theme.app.DEFAULT,
                }}
              />
              <Text style={styles.item}>{item.key}</Text>
            </View>
          )}
        />
      </View>

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
  card: {
    width: "100%",
    borderRadius: 15,
    paddingVertical: 25,
    paddingHorizontal: 15,
    paddingBottom: 5,
    color: theme.white,
  },
  container: {
    paddingVertical: 25,
    paddingHorizontal: 20,
  },
});

import { StyleSheet, Text, View } from "react-native";

export default function FilesScreen() {
  return (
    <View style={{ padding: 20 }}>
      <Text>
        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Commodi in
        assumenda maiores quis consequatur, fugiat, asperiores et unde labore
        ea, error alias quod repellat voluptas consectetur repudiandae magnam
        earum obcaecati.
      </Text>
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

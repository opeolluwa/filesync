import { Text, View } from "react-native";
import theme from "../theme.json";

export const Header = ()=>{
       <View
         style={{
           height: "30%",
           width: "100%",
           backgroundColor: theme.app[500],
           color: theme.white,
         }}
       >
         <View
           style={{
             flex: 1,
             alignItems: "center",
             flexDirection: "row",
             justifyContent: "space-between",
           }}
         >
           <Text></Text>
           <Text>Opeolluwa Pc</Text>
           <Text>Opeolluwa Pc</Text>
           <Text>Opeolluwa Pc</Text>
         </View>
       </View>
};





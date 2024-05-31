import { Tabs } from "expo-router";
import React from "react";

import { Colors } from "@/constants/Colors";
import { useColorScheme } from "@/hooks/useColorScheme";
import { Feather } from "@expo/vector-icons";
import { Ionicons } from "@expo/vector-icons";

export default function TabLayout() {
  const colorScheme = useColorScheme();

  return (
    <Tabs
      screenOptions={{
        // tabBarActiveTintColor: Colors[colorScheme ?? 'light'].tint,
        headerShown: false,
      }}
    >
      <Tabs.Screen
        name="home"
        options={{
          title: "Home",
          tabBarIcon: ({ focused, color }) => (
            <Feather
              name="home"
              size={24}
              color={!focused ? Colors.gray : Colors.app.DEFAULT}
            />
          ),
        }}
      />
      <Tabs.Screen
        name="index"
        options={{
          title: "Scan QR",
          tabBarIcon: ({ color, focused }) => (
            <Ionicons
              name="scan"
              size={24}
              color={!focused ? Colors.gray : Colors.app.DEFAULT}
            />
          ),
        }}
      />

      <Tabs.Screen
        name="files"
        options={{
          title: "Files",
          tabBarIcon: ({ focused , color}) => (
            <Ionicons
              name="folder-open-outline"
              size={24}
              color={!focused ? Colors.gray : Colors.app.DEFAULT}
            />
          ),
        }}
      />
    </Tabs>
  );
}

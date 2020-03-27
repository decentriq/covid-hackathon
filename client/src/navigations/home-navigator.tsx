import React from "react";

import { createBottomTabNavigator } from "@react-navigation/bottom-tabs";
import { StatusScreen, AboutScreen } from "../scenes/home";

export type HomeNavigatorParamList = {
  Status: undefined,
  About: undefined,
}

const HomeStack = createBottomTabNavigator<HomeNavigatorParamList>();

export function HomeNavigator() {
  return (
  <HomeStack.Navigator initialRouteName="Home">
    <HomeStack.Screen name="Status" component={StatusScreen} />
    <HomeStack.Screen name="About" component={AboutScreen} />
  </HomeStack.Navigator>
  )
}

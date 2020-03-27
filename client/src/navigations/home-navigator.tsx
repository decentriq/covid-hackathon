import React from "react";

import { createStackNavigator } from "@react-navigation/stack";
import { HomeScreen } from "../scenes/home";

export type HomeNavigatorParamList = {
  Home: undefined,
}

const HomeStack = createStackNavigator<HomeNavigatorParamList>();

export function HomeNavigator() {
  return (
  <HomeStack.Navigator initialRouteName="Home">
    <HomeStack.Screen name="Home" component={HomeScreen} />
  </HomeStack.Navigator>
  )
}

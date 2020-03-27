import React from "react";

import { NavigationContainer } from "@react-navigation/native";
import { createStackNavigator } from '@react-navigation/stack';
import { OnboardingNavigator } from "./onboarding-navigator";
import { HomeNavigator } from "./home-navigator";

export type RootNavigatorParamList = {
  Onboarding: undefined,
  Home: undefined,
}

const RootStack = createStackNavigator<RootNavigatorParamList>();

function RootNavigator() {
  // TODO implement logic to check if we need to do onboarding
  const doOnboarding = true;
  return (
    <NavigationContainer>
      <RootStack.Navigator screenOptions={{
        headerShown: false
      }}
      initialRouteName={doOnboarding ? "Onboarding" : "Home"}>
      <RootStack.Screen name="Onboarding" component={OnboardingNavigator} />
      <RootStack.Screen name="Home" component={HomeNavigator} />
    </RootStack.Navigator>
  </NavigationContainer>
);
}

export default RootNavigator;

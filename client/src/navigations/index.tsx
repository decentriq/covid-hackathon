import React from "react";

import { NavigationContainer } from "@react-navigation/native";
import { createStackNavigator } from '@react-navigation/stack';
import { OnboardingNavigator } from "./onboarding-navigator";
import { HomeNavigator } from "./home-navigator";

function RootNavigator() {
  const doOnboarding = true;
  if (doOnboarding) {
    const Stack = createStackNavigator();
    return (
      <NavigationContainer>
        <Stack.Navigator>
          <Stack.Screen name="Onboarding" component={OnboardingNavigator} />
          <Stack.Screen name="Home" component={HomeNavigator} />
        </Stack.Navigator>
      </NavigationContainer>
    );
  } else {
    return (
      <NavigationContainer>
          <HomeNavigator />
      </NavigationContainer>
    );

  }
}

export default RootNavigator;

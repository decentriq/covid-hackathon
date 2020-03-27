import React from "react";

import { createStackNavigator } from "@react-navigation/stack";

import { IntroScreen, CompletedScreen } from "../scenes/onboarding";

export type OnboardingNavigatorParamList = {
  Intro: undefined,
  Completed: undefined,
}

const OnboardingStack = createStackNavigator<OnboardingNavigatorParamList>();

export function OnboardingNavigator() {
  return (
  <OnboardingStack.Navigator initialRouteName="Intro">
    <OnboardingStack.Screen name="Intro" component={IntroScreen} />
    <OnboardingStack.Screen name="Completed" component={CompletedScreen} />
  </OnboardingStack.Navigator>
  )
}

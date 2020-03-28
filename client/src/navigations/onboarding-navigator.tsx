import React from "react";

import { createStackNavigator, StackNavigationProp } from "@react-navigation/stack";

import { IntroScreen, CompletedScreen, ExplainationScreen } from "../scenes/onboarding";

export type OnboardingNavigatorParamList = {
  Intro: undefined,
  Completed: undefined,
  Home: undefined,
}

export type OnboardingIntroNavigatorParamList = {
  Intro: undefined,
  Completed: undefined,
  Explaination: undefined,
}

type OnboardingIntroStackNavigationProps =StackNavigationProp<
  OnboardingNavigatorParamList,
  "Intro"
>;

type OnboardingIntroProps = {
  navigation: OnboardingIntroStackNavigationProps;
};

const OnboardingStack = createStackNavigator<OnboardingNavigatorParamList>();
const OnboardingIntroStack = createStackNavigator<OnboardingIntroNavigatorParamList>();

function OnboardingIntroNavigator({navigation}: OnboardingIntroProps) {
  navigation.setOptions({headerShown: false})

  return (
    <OnboardingIntroStack.Navigator mode="modal" headerMode="none" initialRouteName="Intro">
      <OnboardingIntroStack.Screen name="Intro" component={IntroScreen} />
      <OnboardingIntroStack.Screen name="Explaination" component={ExplainationScreen} />
    </OnboardingIntroStack.Navigator>
  )
}

export function OnboardingNavigator() {
  return (
    <OnboardingStack.Navigator initialRouteName="Intro">
      <OnboardingStack.Screen name="Intro" component={OnboardingIntroNavigator} />
      <OnboardingStack.Screen name="Completed" component={CompletedScreen} />
  </OnboardingStack.Navigator>
  )
}

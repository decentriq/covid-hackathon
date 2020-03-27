import React from 'react';
import {
  SafeAreaView,
  Text,
  TouchableHighlight
} from 'react-native';
import { StackNavigationProp } from '@react-navigation/stack';
import { OnboardingNavigatorParamList } from "../../navigations/onboarding-navigator"

type IntroScreenNavigationProp = StackNavigationProp<
  OnboardingNavigatorParamList,
  'Intro'
>;

type IntroProps = {
  navigation: IntroScreenNavigationProp;
};

type CompletedScreenNavigationProp = StackNavigationProp<
  OnboardingNavigatorParamList,
  'Completed'
>;

type CompletedProps = {
  navigation: CompletedScreenNavigationProp;
};

export const IntroScreen = ({navigation}: IntroProps) => (
  <SafeAreaView>
    <Text>Screen: Intro</Text>

    <TouchableHighlight onPress={() => navigation.navigate('Completed')}>
      <Text>Go to completed</Text>
    </TouchableHighlight>
  </SafeAreaView>
);

export const CompletedScreen = ({navigation}: CompletedProps) => (
  <SafeAreaView>
    <Text>Screen: Login</Text>

    <TouchableHighlight onPress={() => navigation.navigate('Home')}>
      <Text>Go to home</Text>
    </TouchableHighlight>
  </SafeAreaView>
);

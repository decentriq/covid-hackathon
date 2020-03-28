import React from 'react';
import {
  SafeAreaView,
  Button,
  Text,
} from 'react-native';
import { StackNavigationProp } from '@react-navigation/stack';
import { OnboardingIntroNavigatorParamList } from "../../navigations/onboarding-navigator";

type ExplainationScreenNavigationProp = StackNavigationProp<
  OnboardingIntroNavigatorParamList,
  'Explaination'
>;

type ExplainationProps = {
  navigation: ExplainationScreenNavigationProp;
};

export const ExplainationScreen = ({navigation}: ExplainationProps) => (
  <SafeAreaView>
    <Text>Screen: Explaination</Text>
    <Button onPress={() => navigation.goBack()} title="Dismiss" />
  </SafeAreaView>
);

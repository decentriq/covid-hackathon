import React from 'react';
import {
  SafeAreaView,
  Text,
} from 'react-native';
import { BottomTabNavigationProp } from '@react-navigation/bottom-tabs';
import { HomeNavigatorParamList } from "../../navigations/home-navigator"

type StatusScreenNavigationProp = BottomTabNavigationProp<
  HomeNavigatorParamList,
  'Status'
>;

type StatusPropos = {
  navigation: StatusScreenNavigationProp;
};

export const StatusScreen = ({navigation}: StatusPropos) => (
  <SafeAreaView>
    <Text>Screen: Status</Text>
  </SafeAreaView>
);

type AboutScreenNavigationProp = BottomTabNavigationProp<
  HomeNavigatorParamList,
  'About'
>;

type AboutPropos = {
  navigation: AboutScreenNavigationProp;
};

export const AboutScreen = ({navigation}: AboutPropos) => (
  <SafeAreaView>
    <Text>Screen: About</Text>
  </SafeAreaView>
);

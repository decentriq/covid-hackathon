import React from 'react';
import {
  SafeAreaView,
  Text,
  TouchableHighlight
} from 'react-native';
import { StackNavigationProp } from '@react-navigation/stack';
import { HomeNavigatorParamList } from "../../navigations/home-navigator"

type HomeScreenNavigationProp = StackNavigationProp<
  HomeNavigatorParamList,
  'Home'
>;

type HomePropos = {
  navigation: HomeScreenNavigationProp;
};

export const HomeScreen = ({navigation}: any) => (
  <SafeAreaView>
    <Text>Screen: Home</Text>

    <TouchableHighlight onPress={() => navigation.navigate('Home')}>
      <Text>Go to home</Text>
    </TouchableHighlight>
  </SafeAreaView>
);

import "react-native-gesture-handler";
import React from "react";
import {
  SafeAreaView,
  StyleSheet,
  View,
  StatusBar,
} from "react-native";
import RootNavigator from "./navigations"

const App = () => {
  return (
    <RootNavigator />
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  sectionContainer: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center"
  },
});

export default App;

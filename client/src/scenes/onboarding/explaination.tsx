import React from "react";
import {
  SafeAreaView,
  TouchableHighlight,
  View,
  StyleSheet,
  Text,
} from "react-native";
import { StackNavigationProp } from "@react-navigation/stack";
import { OnboardingIntroNavigatorParamList } from "../../navigations/onboarding-navigator";
import { Colors } from "../../styles";
import Icon from "react-native-vector-icons/FontAwesome";
import Swiper from "react-native-swiper"
import { Slide } from "../../components/slide";

const styles = StyleSheet.create({
  safeContainer: {
    flex: 1,
    flexDirection: "column",
    backgroundColor: "white",
  },
  wrapper: {
  },
  slide1: {
    backgroundColor: '#97CAE5'
  },
  slide2: {
    backgroundColor: '#97CAE5'
  },
  slide3: {
    backgroundColor: '#92BBD9'
  },
  header: {
    flexDirection: "row",
    justifyContent: "flex-end",
    backgroundColor: 'white',
    shadowColor: Colors.DARK_BLUE,
    borderTopLeftRadius: 10,
    borderTopRightRadius: 10,
    shadowOffset: {width: 0, height: -3 },
    shadowRadius: 2,
    shadowOpacity: 0.2,
    elevation: 0,
  },
  dismissButton: {
    shadowColor: "#000",
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    elevation: 5,
    padding: 10
  }
});

type ExplainationScreenNavigationProp = StackNavigationProp<
  OnboardingIntroNavigatorParamList,
  'Explaination'
>;

type ExplainationProps = {
  navigation: ExplainationScreenNavigationProp;
};

const SLIDES_CONTENT = {
  one: {
    image: require("../../assets/images/person.png"),
    textTop: "Some text top",
    textBot: "Some text bot",
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  },
  two: {
    image: require("../../assets/images/avato.png"),
    textTop: "Some text top",
    textBot: "Some text bot",
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  },
  three: {
    image: require("../../assets/images/analisys.png"),
    textTop: "Some text top",
    textBot: "Some text bot",
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  }
}

export const ExplainationScreen = ({navigation}: ExplainationProps) => (
  <SafeAreaView style={styles.safeContainer}>
    <View style={styles.header}>
      <TouchableHighlight underlayColor="transparent" style={styles.dismissButton} onPress={() => navigation.goBack()}>
        <Icon name="close" size={30} color={Colors.MAIN_BLUE}/>
      </TouchableHighlight>
    </View>
    <Swiper style={styles.wrapper} loop={false} >
      <Slide {...SLIDES_CONTENT.one} />
      <Slide {...SLIDES_CONTENT.two} />
      <Slide {...SLIDES_CONTENT.three} />
    </Swiper>
  </SafeAreaView>
);

import React from 'react';
import {
  SafeAreaView,
  TouchableHighlight,
  View,
  StyleSheet,
  Text,
} from 'react-native';
import {StackNavigationProp} from '@react-navigation/stack';
import {OnboardingIntroNavigatorParamList} from '../../navigations/onboarding-navigator';
import {Colors} from '../../styles';
import Icon from 'react-native-vector-icons/FontAwesome';
import Swiper from 'react-native-swiper';
import {Slide} from '../../components/slide';

const styles = StyleSheet.create({
  safeContainer: {
    flex: 1,
    flexDirection: 'column',
    backgroundColor: 'white',
  },
  wrapper: {},
  slide1: {
    backgroundColor: '#97CAE5',
  },
  slide2: {
    backgroundColor: '#97CAE5',
  },
  slide3: {
    backgroundColor: '#92BBD9',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'flex-end',
    backgroundColor: 'white',
    shadowColor: Colors.DARK_BLUE,
    borderTopLeftRadius: 10,
    borderTopRightRadius: 10,
    shadowOffset: {width: 0, height: -3},
    shadowRadius: 2,
    shadowOpacity: 0.2,
    elevation: 0,
  },
  dismissButton: {
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2,
    },
    shadowOpacity: 0.25,
    shadowRadius: 3.84,
    elevation: 5,
    padding: 10,
  },
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
    imageSrc: require('../../assets/images/person.png'),
    textTop: 'Your data is important',
    textBot:
      "To battle the spread of COVID-19 it is imperative that we stay informed about the spread of the infection, and whether we have been exposed to an infected person. To this end we need to have some form of access to one another's location data.",
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  },
  two: {
    lottieSrc: require('../../assets/lottie/lf20_aPomb3.json'),
    textTop: 'Your privacy is important',
    textBot:
      'However, exposing our location history to the government or unknown third parties has great potential for abuse. We must make sure that our commitment to end this disease does not compromise our right to privacy.',
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  },
  three: {
    lottieSrc: require('../../assets/lottie/lf20_E0YFed.json'),
    textTop: 'Tech to the rescue',
    textBot:
      'cocotrace uses a new technology called Trusted Execution Environments (Intel SGX) to hide our private data from literally everyone, while also allowing the necessary computations to take place. ',
    backgroundColorTop: Colors.EXTRA_LIGHT_BLUE,
    backgroundColorBot: Colors.LIGHT_ORANGE,
  },
};

export const ExplainationScreen = ({navigation}: ExplainationProps) => (
  <SafeAreaView style={styles.safeContainer}>
    <View style={styles.header}>
      <TouchableHighlight
        underlayColor="transparent"
        style={styles.dismissButton}
        onPress={() => navigation.goBack()}>
        <Icon name="close" size={30} color={Colors.MAIN_BLUE} />
      </TouchableHighlight>
    </View>
    <Swiper style={styles.wrapper} loop={false}>
      <Slide {...SLIDES_CONTENT.one} />
      <Slide {...SLIDES_CONTENT.two} />
      <Slide {...SLIDES_CONTENT.three} />
    </Swiper>
  </SafeAreaView>
);

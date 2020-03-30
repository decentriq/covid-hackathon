import React from 'react';
import {
  SafeAreaView,
  View,
  Text,
  TouchableHighlight,
  Image,
  StyleSheet,
} from 'react-native';
import {StackNavigationProp} from '@react-navigation/stack';
import {OnboardingIntroNavigatorParamList} from '../../navigations/onboarding-navigator';
import {Colors} from '../../styles';
import {API} from '../../services/api';
import { box } from "tweetnacl";

type IntroScreenNavigationProp = StackNavigationProp<
  OnboardingIntroNavigatorParamList,
  'Intro'
>;

type IntroProps = {
  navigation: IntroScreenNavigationProp;
};

export const IntroScreen = ({navigation}: IntroProps) => {
  // Testing
  let keypair = box.keyPair();
  let api = new API(keypair.publicKey);
  api.getReport().then((val) => {
    let enclave_key = val.slice(320, 320+32);
    const sharedA = box.before(enclave_key, keypair.secretKey);
    const test = {
      user_id: "lol",
      illnesses: [] as any,
      timestamped_coordinates: [] as any,
    };
    api.poll(sharedA, test).then((val: any)=> {
      console.log(val);
    }).catch((err: any)=> {
      console.log(err);
    });
  });

  navigation.setOptions({headerShown: false});
  return (
    <SafeAreaView style={styles.safeContainer}>
      <View style={styles.container}>
        <View style={styles.header}>
          <Text style={styles.h1}>Cocotrace</Text>
          <View style={styles.subtitle}>
            <Text style={styles.h2}>Location tracing done</Text>
            <Text style={styles.h2_underline}>right</Text>
          </View>
          <Text style={styles.header_paragraph}>
            Text that keeps goinText that keeps goinText that keeps goinText
            that keeps goinText that keeps goingggggText that keeps going
          </Text>
        </View>
        <View style={styles.info_section}>
          <TouchableHighlight
            style={styles.info_section_button}
            onPress={() => navigation.navigate('Explaination')}>
            <View style={styles.info_section_container}>
              <View style={styles.info_image_container}>
                <Image
                  style={styles.info_image}
                  source={require('../../assets/images/safe.png')}
                />
              </View>
              <View style={styles.info_section_more}>
                <Text style={styles.info_section_text}>How it works</Text>
              </View>
            </View>
          </TouchableHighlight>
        </View>
        <View style={styles.footer}>
          <Text>Some more text</Text>
          <TouchableHighlight
            underlayColor={Colors.DARK_ORANGE}
            style={styles.button}
            onPress={() => navigation.navigate('Completed')}>
            <Text style={styles.buttonText}>Sign Up</Text>
          </TouchableHighlight>
        </View>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  safeContainer: {
    flex: 1,
    flexDirection: 'column',
    backgroundColor: 'white',
  },
  container: {
    alignItems: 'center',
    backgroundColor: 'white',
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'space-between',
    padding: 50,
  },
  header: {
    flex: 2,
    justifyContent: 'flex-start',
    alignSelf: 'stretch',
  },
  h1: {
    fontSize: 40,
    fontWeight: 'bold',
    color: Colors.EXTRA_DARK_BLUE,
  },
  subtitle: {
    marginTop: 5,
    flexDirection: 'row',
  },
  h2: {
    fontSize: 20,
    fontWeight: 'bold',
    color: Colors.EXTRA_DARK_BLUE,
  },
  h2_underline: {
    paddingLeft: 5,
    fontSize: 20,
    fontWeight: 'bold',
    textDecorationLine: 'underline',
    color: Colors.MAIN_BLUE,
  },
  header_paragraph: {
    marginTop: 10,
    color: Colors.EXTRA_DARK_BLUE,
  },
  info_section: {
    flex: 3,
    justifyContent: 'center',
    shadowColor: 'black',
    shadowOpacity: 0.3,
    shadowOffset: {
      width: 2,
      height: 2,
    },
    shadowRadius: 10,
    borderRadius: 10,
    margin: 15,
  },
  info_section_container: {},
  info_section_button: {
    borderRadius: 10,
  },
  info_image: {
    height: '100%',
    aspectRatio: 1,
    borderRadius: 10,
  },
  info_image_container: {
    backgroundColor: 'white',
    height: '100%',
    aspectRatio: 1,
    borderRadius: 10,
  },
  info_section_more: {
    backgroundColor: Colors.EXTRA_LIGHT_BLUE,
    position: 'absolute',
    borderBottomLeftRadius: 10,
    borderBottomRightRadius: 10,
    borderTopColor: Colors.MAIN_BLUE,
    borderTopWidth: 1,
    bottom: 0,
    left: 0,
    right: 0,
  },
  info_section_text: {
    padding: 10,
    fontWeight: 'bold',
    color: Colors.MAIN_BLUE,
    fontSize: 30,
  },
  footer: {
    flex: 1,
    justifyContent: 'flex-end',
    alignItems: 'center',
  },
  button: {
    marginTop: 10,
    backgroundColor: Colors.MAIN_ORANGE,
    shadowColor: Colors.MAIN_ORANGE,
    shadowOpacity: 0.5,
    shadowOffset: {
      width: 2,
      height: 2,
    },
    shadowRadius: 10,
    borderRadius: 10,
    paddingLeft: 25,
    paddingRight: 25,
  },
  buttonText: {
    padding: 5,
    fontSize: 30,
    fontWeight: 'bold',
    color: Colors.LIGHT_ORANGE,
  },
});

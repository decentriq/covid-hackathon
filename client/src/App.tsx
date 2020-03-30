import 'react-native-gesture-handler';
import React from 'react';
import {SafeAreaView, StyleSheet, View, StatusBar} from 'react-native';
import RootNavigator from './navigations';
import {createStore, applyMiddleware} from 'redux';
import {Provider} from 'react-redux';
import logger from 'redux-logger';

import {rootReducer} from './store';
import nacl from "tweetnacl";
import { randomBytes } from 'react-native-randombytes'

nacl.setPRNG((x, n) => {
  let bytes = randomBytes(n);
  x.set(bytes);
})


export const store = createStore(rootReducer, applyMiddleware(logger));

const App = () => {
  return (
    <Provider store={store}>
      <RootNavigator />
    </Provider>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  sectionContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
});

export default App;

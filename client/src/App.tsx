import 'react-native-gesture-handler';
import React from 'react';
import {SafeAreaView, StyleSheet, View, StatusBar} from 'react-native';
import RootNavigator from './navigations';
import {createStore, applyMiddleware, compose} from 'redux';
import {Provider} from 'react-redux';
import logger from 'redux-logger';
import {persistStore, persistReducer} from 'redux-persist';
import AsyncStorage from '@react-native-community/async-storage';

import {rootReducer} from './store';
import nacl from 'tweetnacl';
import {randomBytes} from 'react-native-randombytes';
import {PersistGate} from 'redux-persist/integration/react';

nacl.setPRNG((x, n) => {
  let bytes = randomBytes(n);
  x.set(bytes);
});

const persistConfig = {
  key: 'root',
  storage: AsyncStorage,
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

export const store = createStore(persistedReducer, applyMiddleware(logger));
export const persistor = persistStore(store);

const App = () => {
  return (
    <Provider store={store}>
      <PersistGate loading={null} persistor={persistor}>
        <RootNavigator />
      </PersistGate>
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

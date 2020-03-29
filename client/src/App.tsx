import 'react-native-gesture-handler';
import React from 'react';
import {SafeAreaView, StyleSheet, View, StatusBar} from 'react-native';
import RootNavigator from './navigations';
import {createStore, applyMiddleware} from 'redux';
import {Provider, connect} from 'react-redux';

import {rootReducer} from './reducers';

const store = createStore(rootReducer);

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

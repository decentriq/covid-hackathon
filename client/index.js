import {AppRegistry} from 'react-native';
import App from './src/App';
import {name as appName} from './app.json';
import BackgroundFetch from 'react-native-background-fetch';
import BackgroundGeolocation from 'react-native-background-geolocation';
import {addLocation, deleteLocations} from './src/store/traces/actions';
import {RootState} from './src/store';
import {store} from './src/App'

BackgroundGeolocation.onLocation(
    (location) => {
        store.dispatch(addLocation(location))
    }, (error) => {
        console.warn('[location] ERROR -', error);
    }
    );
    
const BackgroundGeolocationHeadlessTask = async (event, params) => {
    console.log('[BackgroundGeolocation HeadlessTask] -', event.name, params);
};

AppRegistry.registerComponent(appName, () => App);
BackgroundGeolocation.registerHeadlessTask(BackgroundGeolocationHeadlessTask);
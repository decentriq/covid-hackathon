import React from 'react';
import {
  SafeAreaView,
  Text,
} from 'react-native';
import { BottomTabNavigationProp } from '@react-navigation/bottom-tabs';
import { HomeNavigatorParamList } from "../../navigations/home-navigator"
import BackgroundGeolocation, {
  Location,
  LocationError,
  HeartbeatEvent,
  MotionActivityEvent,
  MotionChangeEvent,
  ProviderChangeEvent,
} from "react-native-background-geolocation";

type StatusScreenNavigationProp = BottomTabNavigationProp<
  HomeNavigatorParamList,
  'Status'
>;

type StatusProps = {
  navigation: StatusScreenNavigationProp,
};

type StatusState = {
}

export class StatusScreen extends React.Component<StatusProps, StatusState> {
  state: StatusState = {
  }

  constructor(props: StatusProps) {
    super(props)
    BackgroundGeolocation.onLocation(this.onLocation, this.onError);
    BackgroundGeolocation.onMotionChange(this.onMotionChange);
    BackgroundGeolocation.onActivityChange(this.onActivityChange);
    BackgroundGeolocation.onProviderChange(this.onProviderChange);
    BackgroundGeolocation.onHeartbeat(this.onHeartbeat);
  }

  onLocation(location: Location) {
    console.log('[location] -', location);
  }
  onError(error: LocationError) {
    console.warn('[location] ERROR -', error);
  }
  onActivityChange(event: MotionActivityEvent) {
    console.log('[activitychange] -', event);  // eg: 'on_foot', 'still', 'in_vehicle'
  }
  onProviderChange(provider: ProviderChangeEvent) {
    console.log('[providerchange] -', provider.enabled, provider.status);
  }
  onMotionChange(event: MotionChangeEvent) {
    console.log('[motionchange] -', event.isMoving, event.location);
  }
  onHeartbeat(event: HeartbeatEvent) {
    console.log('[heartbeat] -', event.location);
  }

  componentDidMount() {
    console.log("Home mounted");
    BackgroundGeolocation.ready({
      distanceFilter: 10,
      stopOnTerminate: false,
      logLevel: BackgroundGeolocation.LOG_LEVEL_VERBOSE,
      debug: false
    }, (state) => {
      console.log("- BackgroundGeolocation is configured and ready: ", state.enabled);

      if (!state.enabled) {
        BackgroundGeolocation.start(function() {
          console.log("- Start success");
        });
      }
    });
  }

  componentWillUnmount() {
    BackgroundGeolocation.removeListeners();
  }

  render() {

    return (
      <SafeAreaView>
        <Text>Screen: Status</Text>
      </SafeAreaView>
    )
  }
}

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

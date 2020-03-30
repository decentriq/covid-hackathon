import React from 'react';
import {SafeAreaView, Text, Button, ScrollView} from 'react-native';
import {BottomTabNavigationProp} from '@react-navigation/bottom-tabs';
import {HomeNavigatorParamList} from '../../navigations/home-navigator';
import BackgroundGeolocation, {
  Location,
  LocationError,
  HeartbeatEvent,
  MotionActivityEvent,
  MotionChangeEvent,
  ProviderChangeEvent,
} from 'react-native-background-geolocation';
import {connect, ConnectedProps} from 'react-redux';
import {RootState} from '../../store';
import {addLocation, deleteLocations} from '../../store/traces/actions';
import StatusComponent from '../../components/status';
import LocaationComponent from '../../components/location';

type StatusScreenNavigationProp = BottomTabNavigationProp<
  HomeNavigatorParamList,
  'Status'
>;

type MyStatusProps = {
  navigation: StatusScreenNavigationProp;
};
type StatusProps = StatusPropsFromRedux & MyStatusProps;
type StatusState = {};

class StatusScreenPrivate extends React.Component<StatusProps, StatusState> {
  state: StatusState = {};

  constructor(props: StatusProps) {
    super(props);
    // BackgroundGeolocation.onLocation(l => this.onLocation(l), this.onError);
  }

  onLocation(location: Location) {
    console.log('[location] -', location);
    // console.log(this.props)
    this.props.addLocation(location);
  }
  onError(error: LocationError) {
    console.warn('[location] ERROR -', error);
  }

  componentDidMount() {
    console.log('Home mounted');
    BackgroundGeolocation.ready(
      {
        distanceFilter: 30,
        logLevel: BackgroundGeolocation.LOG_LEVEL_VERBOSE,
        stopOnTerminate: false,
        startOnBoot: true,
        debug: false,
        desiredAccuracy: BackgroundGeolocation.DESIRED_ACCURACY_HIGH,
        locationUpdateInterval: 60000,
        fastestLocationUpdateInterval: 60000,
        disableMotionActivityUpdates: true,
        stopTimeout: 1,
      },

      state => {
        console.log(
          '- BackgroundGeolocation is configured and ready: ',
          state.enabled,
        );

        if (!state.enabled) {
          BackgroundGeolocation.start(function() {
            console.log('- Start success');
          });
        }
      },
    );
  }

  componentWillUnmount() {
    // BackgroundGeolocation.removeListeners();
  }

  render() {
    const {counter, incrementClick} = this.props;
    return (
      <SafeAreaView>
        <ScrollView>
          <Text>Screen: Status</Text>
          <StatusComponent />
          {/* <Button onPress={() => incrementClick()} title="Increment Me!" /> */}
          <LocaationComponent />
        </ScrollView>
      </SafeAreaView>
    );
  }
}

const mapState = (state: RootState) => ({
  counter: state.counter.value,
});

const mapDispatch = {
  incrementClick: () => ({type: 'INCREMENT'}),
  addLocation: (l: any) => addLocation(l),
};

type StatusPropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapState, mapDispatch);
export const StatusScreen = connector(StatusScreenPrivate);

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

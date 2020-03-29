import React from 'react';
import {Text, Button, View} from 'react-native';
import {connect, ConnectedProps} from 'react-redux';
import {RootState} from '../store';
import BackgroundGeolocation, {
  Location,
  LocationError,
  HeartbeatEvent,
  MotionActivityEvent,
  MotionChangeEvent,
  ProviderChangeEvent,
} from 'react-native-background-geolocation';


type MyProps = {
  fontSize: number;
};
type Props = PropsFromRedux & MyProps;

type CounterState = {
  interval: number | null;
};

export class Counter extends React.Component<Props, CounterState> {
  state: CounterState = {
    interval: null,
  };
  constructor(props: Props) {
    super(props);
    BackgroundGeolocation.onLocation(this.props.addLocation, this.onError);
    BackgroundGeolocation.onMotionChange(this.onMotionChange);
    BackgroundGeolocation.onActivityChange(this.onActivityChange);
    BackgroundGeolocation.onProviderChange(this.onProviderChange);
    BackgroundGeolocation.onHeartbeat(this.onHeartbeat);
    this.onLocation = this.onLocation.bind(this)
  }

  onLocation(location: Location) {
    console.log('[location] -', location);
    console.log(this.props)
    // this.props.addLocation(location)
  }
  onError(error: LocationError) {
    console.warn('[location] ERROR -', error);
  }
  onActivityChange(event: MotionActivityEvent) {
    console.log('[activitychange] -', event); // eg: 'on_foot', 'still', 'in_vehicle'
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
    console.log('Home mounted');
    BackgroundGeolocation.ready(
      {
        distanceFilter: 10,
        stopOnTerminate: false,
        logLevel: BackgroundGeolocation.LOG_LEVEL_VERBOSE,
        debug: false,
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
    BackgroundGeolocation.removeListeners();
  }

  render() {
    const {counter, incrementClick, fontSize, addLocation} = this.props;

    return (
      <View>
        <Text style={{fontSize}}>{counter}</Text>
        <Button onPress={() => incrementClick()} title="Increment Me!" />
        <Button onPress={() => addLocation("sadsa")} title="Increment Message!" />
      </View>
    );
  }
}

const mapState = (state: RootState) => ({
  counter: state.counter.value,
});

const mapDispatch = {
  incrementClick: () => ({type: 'INCREMENT'}),
  addLocation: (location: any) => ({
        type: 'ADD_LOCATION',
        payload: location
      }),
};

type PropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapState, mapDispatch);
export default connector(Counter);

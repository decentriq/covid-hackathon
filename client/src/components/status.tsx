import React from 'react';
import {Text, Button, View, ScrollView} from 'react-native';
import {connect, ConnectedProps} from 'react-redux';
import {RootState} from '../store';
import {CurrentStatus} from '../store/general/types';
import {changeStatus, changeIllness} from '../store/general/actions';
import {Colors} from '../styles';
import { PollResponse } from '../services/api';

type MyProps = {};
type Props = PropsFromRedux & MyProps;

type State = {};

class StatusComponent extends React.Component<Props, State> {
  state: State = {};

  constructor(props: Props) {
    super(props);
  }

  triggerInfected() {
    this.props.changeStatus(CurrentStatus.Infected);
    this.props.changeIllness({
      start_time: new Date(),
      duration_days: 14,
    });
  }

  triggerUpdate() {
    // here we need to do the following:
    const {locations, illness} = this.props; 
    // locations: array of type Locations like defined here: https://github.com/transistorsoft/react-native-background-geolocation/blob/d7ec0ea0ec8ced8fe896e132d51dbd055fe118aa/src/declarations/interfaces/Location.d.ts#L129
    // illness: current state of illness

    // * convert location to TimeStampCoordinates
    // * send request to the backend
    
    // example response
    const response = {exposed_timestamp: new Date()} as PollResponse;

    if (response.exposed_timestamp) {
      this.props.changeStatus(CurrentStatus.Exposed); 
    }

    // * if exposed is set then update exposed => call this funtion
  }

  getStatusRender(status: CurrentStatus) {
    switch (status) {
      case CurrentStatus.Recording:
        return (
          <View style={{backgroundColor: Colors.EXTRA_EXTRA_LIGHT_BLUE}}>
            <Text style={{color: "green", fontSize: 25}}>
              Healthy
            </Text>
          </View>
        );
      case CurrentStatus.Infected:
        return (
          <View>
            <Text style={{color: "red", fontSize: 25}}>Infected</Text>
          </View>
        );
      case CurrentStatus.Exposed:
        return (
          <View>
            <Text style={{color: Colors.MAIN_ORANGE, fontSize: 25}}>
              Exposed
            </Text>
          </View>
        );
      default:
        throw Error('Unknown Status');
    }
  }
  render() {
    const {locations, enclave_identity, status} = this.props;
    const last_location = locations.slice(-1)[0];
    let last_location_timestamp = null;
    if (last_location != undefined) {
      last_location_timestamp = last_location.timestamp.toString();
    }
    const status_render = this.getStatusRender(status);

    return (
      <ScrollView>
        <View
          style={{flex: 1, justifyContent: 'center', alignItems: 'flex-start'}}>
          <Text>Last Location Data: {'\n' + last_location_timestamp}</Text>

          {status_render}

          <Button
            onPress={() => this.triggerInfected()}
            title="Press here to to indicate to that you've been diagnosed"
          />

          <Text>Enclave Identity: 61aea8172f771837ac43c5e4fe6168abc0ba6607eccd6f3f5fce9d89572a0a74</Text>
        </View>

        <Button onPress={() => this.triggerUpdate()} title="Update" />
      </ScrollView>
    );
  }
}

const mapState = (state: RootState) => ({
  locations: state.traces.locations,
  enclave_identity: state.general.enclave_identity,
  illness: state.general.illness,
  status: state.general.current_status,
});

const mapDispatch = {
  changeStatus: changeStatus,
  changeIllness: changeIllness,
};

type PropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapState, mapDispatch);
export default connector(StatusComponent);

import React from 'react';
import {Text, Button, View, ScrollView} from 'react-native';
import {connect, ConnectedProps} from 'react-redux';
import {RootState} from '../store';

type MyProps = {};
type Props = PropsFromRedux & MyProps;

type State = {};

class LocationComponent extends React.Component<Props, State> {
  state: State = {};

  constructor(props: Props) {
    super(props);
  }

  render() {
    const {locations} = this.props;

    return (
      <ScrollView>
        {locations.map(l => (
          <Text>
            {l.coords.longitude}, {l.coords.latitude}
          </Text>
        ))}
      </ScrollView>
    );
  }
}

const mapState = (state: RootState) => ({
  locations: state.traces.locations,
});

const mapDispatch = {};

type PropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapState, mapDispatch);
export default connector(LocationComponent);

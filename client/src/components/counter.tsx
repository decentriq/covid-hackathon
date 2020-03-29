import React from 'react';
import {Text, Button, View} from 'react-native';
import {connect, ConnectedProps} from 'react-redux';
import {RootState} from '../reducers';

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
  }

  componentDidMount() {
    this.state.interval = setInterval(() => this.props.incrementClick(), 1000);
  }

  componentWillUnmount() {
    clearInterval(this.state.interval!);
  }

  render() {
    const {counter, incrementClick, fontSize} = this.props;

    return (
      <View>
        <Text style={{fontSize}}>{counter}</Text>
        <Button onPress={() => incrementClick()} title="Increment Me!" />
      </View>
    );
  }
}

const mapState = (state: RootState) => ({
  counter: state.counter.value,
});

const mapDispatch = {
  incrementClick: () => ({type: 'INCREMENT'}),
};

type PropsFromRedux = ConnectedProps<typeof connector>;
const connector = connect(mapState, mapDispatch);
export default connector(Counter);

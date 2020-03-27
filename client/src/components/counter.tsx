import React from 'react';
import {
  Text,
} from 'react-native';

type CounterProps = {
  color: string,
  size: number
}

type CounterState = {
  count: number,
  interval: number | null
}

export class Counter extends React.Component<CounterProps, CounterState> {
  state: CounterState = {
    count: 0,
    interval: null
  }

  constructor(props: CounterProps) {
    super(props)
  }

  componentDidMount() {
    this.state.interval = setInterval(() => {
      this.setState({count: this.state.count + 1})
    }, 1000)
  }

  componentWillUnmount() {
    clearInterval(this.state.interval!);
  }

  render() {
    const {count} = this.state
    const {color, size} = this.props

    return (
      <Text style={{color, fontSize: size}}>
        {count}
      </Text>
    )
  }
}

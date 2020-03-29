import React from 'react';
import {
  Text,
} from 'react-native';
// import { MobileAppBridge } from 'NativeModules';
import {NativeModules} from 'react-native';


type Props = {
}

type State = {
  name: string,
}

export class RustNative extends React.Component<Props, State> {
  state: State = {
    name: "",
  }

  componentDidMount() {
    this.displayHelloWorld().then((val) => 
      this.setState({
        name: val
      }));
  }

  render() {
    return (
      <Text>
        {this.state.name}
        {/* Hallo test */}
      </Text>
    )
  }

  async displayHelloWorld(): Promise<string> {
    try {
      console.log("triggered rust callback")
      let text = await NativeModules.MobileAppBridge.sayHelloWorld("Android")
      return text;
    } catch (e) {
      return Promise.reject("failed to get data for native");
    }
  }
}




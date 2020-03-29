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

export default class RustNative extends React.Component<Props, State> {
  state: State = {
    name: "",
  }

  componentDidMount() {
    displayHelloWorld(this)
  }


  render() {

    return (
      <Text>
        {this.state.name}
        {/* Hallo test */}
      </Text>
    )
  }
}


async function displayHelloWorld (self: any) {
  try {
    console.log("triggered rust callback")
    let text = await NativeModules.MobileAppBridge.sayHelloWorld("Android")
    console.log(test)
    self.setState({
      hello: text
    })
  } catch (e) {
      console.log(e)
  }
}



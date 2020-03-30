import React from 'react';
import {View, Text, StyleSheet, Image, ImageSourcePropType} from 'react-native';
import LottieView from 'lottie-react-native';
import {Colors} from '../styles';

type SlideProps = {
  backgroundColorTop: string;
  backgroundColorBot: string;
  imageSrc?: ImageSourcePropType;
  lottieSrc?: AnimationObjectPropType;
  textTop: string;
  textBot: string;
};

interface AnimationObjectPropType {
  v: string;
  fr: number;
  ip: number;
  op: number;
  w: number;
  h: number;
  nm: string;
  ddd: number;
  assets: any[];
  layers: any[];
}

type SlideState = {};

export class Slide extends React.Component<SlideProps, SlideState> {
  state: SlideState = {};

  constructor(props: SlideProps) {
    super(props);
  }

  componentDidMount() {}

  componentWillUnmount() {}

  render() {
    const {imageSrc, textTop, textBot, lottieSrc} = this.props;
    const backgroundColorTop = {
      backgroundColor: this.props.backgroundColorTop,
      shadowColor: this.props.backgroundColorTop,
    };
    const backgroundColorBot = {
      backgroundColor: this.props.backgroundColorBot,
      shadowColor: this.props.backgroundColorBot,
    };

    return (
      <View style={styles.slide_general}>
        <View style={{...backgroundColorTop, ...styles.header}}>
          <Text style={styles.textTop}>{textTop}</Text>
        </View>
        <View style={styles.section_image}>
          <View style={styles.section_image_container}>
            {imageSrc && (
              <Image
                resizeMode="contain"
                style={styles.slide_image}
                source={imageSrc as ImageSourcePropType}
              />
            )}

            {lottieSrc && (
              <LottieView
                style={styles.slide_image}
                source={(lottieSrc as any) as AnimationObjectPropType}
                resizeMode="cover"
                autoPlay
                loop={true}
              />
            )}
          </View>
        </View>
        <View style={{...backgroundColorBot, ...styles.footer}}>
          <Text style={styles.textBot}>{textBot}</Text>
        </View>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  slide_general: {
    flex: 1,
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  header: {
    flex: 1,
    alignSelf: 'stretch',
    shadowOpacity: 1.0,
    shadowOffset: {
      width: 2,
      height: 2,
    },
    shadowRadius: 10,
    borderRadius: 10,
    margin: 15,
  },
  footer: {
    flex: 2,
    alignSelf: 'stretch',
    shadowOpacity: 1.0,
    shadowOffset: {
      width: 2,
      height: 2,
    },
    shadowRadius: 10,
    borderRadius: 10,
    margin: 15,
    marginBottom: 50,
  },
  section_image: {
    margin: 15,
    alignSelf: 'stretch',
    flex: 5,
  },
  section_image_container: {
    flex: 1,
    alignSelf: 'stretch',
    alignItems: 'center',
  },
  slide_image: {
    flex: 1,
    height: '100%',
    width: '100%',
  },
  textTop: {
    padding: 15,
    color: Colors.DARK_BLUE,
    fontSize: 30,
    fontWeight: 'bold',
  },
  textBot: {
    padding: 15,
    color: Colors.MAIN_ORANGE,
    fontSize: 15,
    fontWeight: 'bold',
  },
});

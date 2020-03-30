# cocotrace | confidential contact tracing

This is the repo for the *cocotrace* project in the CodeVsCovid19 Hackathon. 

https://devpost.com/software/cocotrace-confidential-contact-tracing

https://www.codevscovid19.org

Most members of this team are from [decentriq](https://www.decentriq.ch).


## Inspiration
At decentriq we work with privacy preserving technologies. We learned that contact tracing based on smartphone data (GPS, proximity data, etc) has huge potential for fighting the current pandemic. It has been successfully deployed in countries such as South Korea, but is facing resistance in many European countries due to legitimate privacy concerns of such data collections.

We will build a system that allows gathering smartphone data for contact tracing, while provably keeping all sensitive data confidential from peers and any central authority. We hope that such a system will help the fight against the Corona virus without the need for any privacy compromises.

### Comparison to Hamagen

Israel's health ministery recently launched [Hamagen](https://[https://play.google.com/store/apps/details?id=com.hamagen](https://play.google.com/store/apps/details?id=com.hamagen)
), a contact tracing system. The Hamagen app claims that it only processes the user's location data on device. Which is a good first step, however, it fetches the traces of relevant infected people from public government servers. Hence, the location data of infected patients is not protected at all but instead gets pushed to all other users in order to compute locally if they had been close to the infected person. 

Hamagen protects the data privacy of the healthy people and is a clear improvement over a central server collecting all data on an ongoing basis. It however requires the infected people to share all their  movement data with all people in the system. This sharing is likely to prevent some people from revealing their infection.

In terms of privacy, the *cocotrace* system is superior to Hamagen as *cocotrace* protects the privacy of all participants. The only information getting out about you is the fact that you have met one of patients. If we generalise the returned timestamp to only reveal the day when this happened, we argue that *cocotrace* also protects the privacy of infected people really well. 

In *cocotrace* there is nothing preventing people from letting the system know that they have been infected. This improved privacy should bring more users to the system and improve the crucial contact tracing success rate. 

## Tech
We will build a GPS collecting app using react-native and a central server built in Rust leveraging the Intel SGX confidential computing technology to gather the data in a privacy-preserving way.

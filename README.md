# cocotrace | confidential contact tracing

This is the repo for the cocotrace project in the CodeVsCovid19 Hackathon. 

https://devpost.com/software/cocotrace-confidential-contact-tracing

https://www.codevscovid19.org

Most members of this team are from decentriq.

https://www.decentriq.ch

## Inspiration
At decentriq we work with privacy preserving technologies. We learned that contact tracing based on smartphone data (GPS, proximity data, etc) has huge potential for fighting the current pandemic. It has been successfully deployed in countries such as South Korea, but is facing resistance in many European countries due to legitimate privacy concerns of such data collections.

We will build a system that allows gathering smartphone data for contact tracing, while provably keeping all sensitive data confidential from peers and any central authority. We hope that such a system will help the fight against the Corona virus without the need for any privacy compromises.

## Tech
We will build a GPS collecting app using react-native and a central server built in Rust leveraging the Intel SGX confidential computing technology to gather the data in a privacy-preserving way.
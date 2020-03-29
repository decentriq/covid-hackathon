#! /bin/bash

set -xeuo pipefail

# ios 
rustup target add i386-apple-ios
rustup target add x86_64-apple-ios
rustup target add armv7-apple-ios
rustup target add armv7s-apple-ios
rustup target add aarch64-apple-ios

# yarn
yarn
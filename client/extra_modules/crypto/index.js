import { randomBytes } from 'react-native-randombytes';

exports.getRandomValues = function getRandomValues(arr) {
    let orig = arr;
    if (arr.byteLength !== arr.length) {
        // Get access to the underlying raw bytes
        arr = new Uint8Array(arr.buffer);
    }
    const bytes = randomBytes(arr.length);
    for (var i = 0; i < bytes.length; i++) {
        arr[i] = bytes[i];
    }

    return orig;
};
exports.randomBytes = exports.rng = exports.pseudoRandomBytes = exports.prng = randomBytes;

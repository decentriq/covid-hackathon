import * as axios from 'axios';
import {TextEncoder, TextDecoder} from 'text-encoding-shim';

const base64abc = (() => {
  let abc = [],
    A = 'A'.charCodeAt(0),
    a = 'a'.charCodeAt(0),
    n = '0'.charCodeAt(0);
  for (let i = 0; i < 26; ++i) {
    abc.push(String.fromCharCode(A + i));
  }
  for (let i = 0; i < 26; ++i) {
    abc.push(String.fromCharCode(a + i));
  }
  for (let i = 0; i < 10; ++i) {
    abc.push(String.fromCharCode(n + i));
  }
  abc.push('+');
  abc.push('/');
  return abc;
})();

function bytesToBase64(bytes: Uint8Array) {
  let result = '',
    i,
    l = bytes.length;
  for (i = 2; i < l; i += 3) {
    result += base64abc[bytes[i - 2] >> 2];
    result += base64abc[((bytes[i - 2] & 0x03) << 4) | (bytes[i - 1] >> 4)];
    result += base64abc[((bytes[i - 1] & 0x0f) << 2) | (bytes[i] >> 6)];
    result += base64abc[bytes[i] & 0x3f];
  }
  if (i === l + 1) {
    // 1 octet missing
    result += base64abc[bytes[i - 2] >> 2];
    result += base64abc[(bytes[i - 2] & 0x03) << 4];
    result += '==';
  }
  if (i === l) {
    // 2 octets missing
    result += base64abc[bytes[i - 2] >> 2];
    result += base64abc[((bytes[i - 2] & 0x03) << 4) | (bytes[i - 1] >> 4)];
    result += base64abc[(bytes[i - 1] & 0x0f) << 2];
    result += '=';
  }
  return result;
}

export interface Illness {
  start_time: string;
  duration_days?: number;
}

export interface TimestampedCoordinate {
  timestamp: string;
  x: number;
  y: number;
}

export interface PollRequest {
  user_id: string;
  illnesses: Array<Illness>;
  timestamped_coordinates: Array<TimestampedCoordinate>;
}

export interface PollResponse {
  exposed_timestamp?: string;
}

import {box, randomBytes} from 'tweetnacl';

const newNonce = () => randomBytes(box.nonceLength);
export const generateKeyPair = () => box.keyPair();

function encrypt(
  secretOrSharedKey: Uint8Array,
  json: any,
  key?: Uint8Array,
): Uint8Array {
  const nonce = newNonce();
  const messageUint8 = new TextEncoder('utf-8').encode(JSON.stringify(json));
  const encrypted = key
    ? box(messageUint8, nonce, key, secretOrSharedKey)
    : box.after(messageUint8, nonce, secretOrSharedKey);

  const fullMessage = new Uint8Array(nonce.length + encrypted.length);
  fullMessage.set(nonce);
  fullMessage.set(encrypted, nonce.length);

  return fullMessage;
}

function decrypt(
  secretOrSharedKey: Uint8Array,
  messageWithNonce: Uint8Array,
  key?: Uint8Array,
): any {
  const nonce = messageWithNonce.slice(0, box.nonceLength);
  const message = messageWithNonce.slice(
    box.nonceLength,
    messageWithNonce.length,
  );

  const decrypted = key
    ? box.open(message, nonce, key, secretOrSharedKey)
    : box.open.after(message, nonce, secretOrSharedKey);

  if (!decrypted) {
    throw new Error('Could not decrypt message');
  }

  const decodedDecryptedMessage = new TextDecoder('utf-8').decode(decrypted);
  return JSON.parse(decodedDecryptedMessage);
}

export class API {
  session: axios.AxiosInstance;

  constructor(clientKey: Uint8Array) {
    this.session = axios.default.create({
      headers: {ClientPubKey: bytesToBase64(clientKey)},
      baseURL: 'http://localhost:3000/',
      responseType: 'arraybuffer',
    });
  }

  async getReport(): Promise<Uint8Array> {
    let buffer: ArrayBuffer = (await this.get('/report')).data;
    return new Uint8Array(buffer);
  }

  async poll(
    sharedSecret: Uint8Array,
    request: PollRequest,
  ): Promise<PollResponse> {
    let encrypted_data = encrypt(sharedSecret, request);
    let buffer: ArrayBuffer = (
      await this.post(
        '/poll',
        encrypted_data,
        {'Content-Type': 'application/octet-stream'},
        'arraybuffer',
      )
    ).data;
    let decrypted_data: PollResponse = decrypt(
      sharedSecret,
      new Uint8Array(buffer),
    );
    return decrypted_data;
  }

  private async post(
    url: string,
    reqBody: object,
    headers: object = {},
    responseType = 'json',
  ): Promise<axios.AxiosResponse> {
    return this.session.post(url, reqBody, {
      headers: headers,
      responseType: responseType as any,
    });
  }

  private async get(
    url: string,
    options: object = {},
  ): Promise<axios.AxiosResponse> {
    return this.session.get(url, options);
  }

  private async head(
    url: string,
    options: object = {},
  ): Promise<axios.AxiosResponse> {
    return this.session.head(url, options);
  }

  private async delete(
    url: string,
    options: object = {},
  ): Promise<axios.AxiosResponse> {
    return this.session.delete(url, options);
  }
}

import * as axios from 'axios';
import { TextEncoder, TextDecoder } from 'text-encoding-shim';

interface Illness {
    start_time: Date,
    duration_days?: number 
}

interface TimestampedCoordinate {
    timestamp: Date,
    x: number,
    y: number,
}

interface PollRequest {
    user_id: string,
    illnesses: [Illness],
    timestamped_coordinates: [TimestampedCoordinate],
}

interface PollResponse {
    exposed_timestamp?: Date,
}

import { box, randomBytes } from 'tweetnacl';

const newNonce = () => randomBytes(box.nonceLength);
export const generateKeyPair = () => box.keyPair();

function encrypt(
  secretOrSharedKey: Uint8Array,
  json: any,
  key?: Uint8Array
): Uint8Array {
  const nonce = newNonce();
  const messageUint8 = new TextEncoder("utf-8").encode(json);
  const encrypted = key
    ? box(messageUint8, nonce, key, secretOrSharedKey)
    : box.after(messageUint8, nonce, secretOrSharedKey);

  const fullMessage = new Uint8Array(nonce.length + encrypted.length);
  fullMessage.set(nonce);
  fullMessage.set(encrypted, nonce.length);

  return fullMessage;
};

function decrypt (
  secretOrSharedKey: Uint8Array,
  messageWithNonce: Uint8Array,
  key?: Uint8Array
): any {
  const nonce = messageWithNonce.slice(0, box.nonceLength);
  const message = messageWithNonce.slice(
    box.nonceLength,
    messageWithNonce.length
  );

  const decrypted = key
    ? box.open(message, nonce, key, secretOrSharedKey)
    : box.open.after(message, nonce, secretOrSharedKey);

  if (!decrypted) {
    throw new Error('Could not decrypt message');
  }

  const decodedDecryptedMessage = new TextDecoder("utf-8").decode(decrypted);
  return JSON.parse(decodedDecryptedMessage);
};

export class API {
  session: axios.AxiosInstance;

  constructor(authorizationToken: string) {
    this.session = axios.default.create({
      headers: {Authorization: `Bearer ${authorizationToken}`},
    });
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

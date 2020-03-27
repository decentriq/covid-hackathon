import { expect } from 'chai';
import 'mocha';

import * as chily from "../pkg-node/chily";

describe('Using hard-coded secrets', () => {
    
    let alice_secret = chily.StaticSecret.fromBytes(new Uint8Array([
        56, 4, 190, 47, 42, 57, 72, 25, 194, 75, 105, 248, 146, 43, 60, 53, 55, 165, 30, 61,
        122, 57, 95, 197, 210, 123, 95, 97, 215, 28, 193, 92,
    ]));
    
    let bob_secret = chily.StaticSecret.fromBytes(new Uint8Array([
        64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
        243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
    ]));
    
    let bob = chily.Keypair.fromSecret(bob_secret);
    let alice = chily.Keypair.fromSecret(alice_secret);

    let nonce = chily.Nonce.fromBytes(new Uint8Array(24).fill(0));
    
    it('Enc-Dec is identity', () => {
        
        var plaintext = new Uint8Array([21,31]);
        let cipher = new chily.Cipher(alice.secret, bob.publicKey);
        
        let encrypted = cipher.encrypt(plaintext, nonce);
        let decrypted = cipher.decrypt(encrypted, nonce);
        expect(plaintext).to.eql(decrypted);
    });

    it('Check unseeded ciphertext', () => {

        var plaintext = new Uint8Array([21,31]);
        let cipher = new chily.Cipher(alice.secret, bob.publicKey);
        
        let encrypted = cipher.encrypt(plaintext, nonce);
        // just take the first two bytes here, tail is the auth tag
        expect(encrypted.slice(0,2)).to.eql(new Uint8Array([63,106]));
    });
});

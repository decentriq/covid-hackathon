"use strict";
exports.__esModule = true;
var chai_1 = require("chai");
require("mocha");
var chily = require("../pkg-node/chily");
describe('Using hard-coded secrets', function () {
    var alice_secret = chily.StaticSecret.fromBytes(new Uint8Array([
        56, 4, 190, 47, 42, 57, 72, 25, 194, 75, 105, 248, 146, 43, 60, 53, 55, 165, 30, 61,
        122, 57, 95, 197, 210, 123, 95, 97, 215, 28, 193, 92,
    ]));
    var bob_secret = chily.StaticSecret.fromBytes(new Uint8Array([
        64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229, 128, 129,
        243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
    ]));
    var bob = chily.Keypair.fromSecret(bob_secret);
    var alice = chily.Keypair.fromSecret(alice_secret);
    var nonce = chily.Nonce.fromRandom();
    it('Enc-Dec is identity', function () {
        var plaintext = new Uint8Array([21, 31]);
        var cipher = chily.Cipher(alice.secret, bob.publicKey);
        cipher.encrypt(plaintext, nonce);
        cipher.decrypt(plaintext, nonce);
        chai_1.expect(plaintext).to.eql(new Uint8Array([21, 31]));
    });
    it('Check unseeded ciphertext', function () {
        var plaintext = new Uint8Array([21, 31]);
        var cipher = chily.Cipher(alice.secret, bob.publicKey);
        cipher.encrypt(plaintext, nonce);
        chai_1.expect(plaintext).to.eql(new Uint8Array([68, 150]));
    });
});

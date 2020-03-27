import chily
import pytest

alice_secret = chily.StaticSecret.from_bytes([
    56, 4, 190, 47, 42, 57, 72, 25, 194, 75, 105, 248, 146, 43, 60,53, 55,
    165, 30, 61, 122, 57, 95, 197, 210, 123, 95, 97, 215, 28, 193, 92
])
bob_secret = chily.StaticSecret.from_bytes([
    64, 218, 126, 251, 171, 87, 50, 212, 196, 55, 166, 65, 195, 199, 64, 229,
    128, 129, 243, 144, 211, 52, 77, 159, 48, 167, 45, 8, 79, 228, 116, 101,
])

bob = chily.Keypair.from_secret(bob_secret)
alice = chily.Keypair.from_secret(alice_secret)

def test_enc_dec_identity():
    plain_text = [21, 31]
    nonce = chily.Nonce.from_random()
    cipher = chily.Cipher(alice.secret, bob.public_key)
    enc = cipher.encrypt(plain_text, nonce)
    dec = cipher.decrypt(enc, nonce)

    assert dec == plain_text

def test_unseeded_ciphertext():
    plain_text = [21, 31]
    nonce = chily.Nonce.from_bytes([0]*24)
    cipher = chily.Cipher(alice.secret, bob.public_key)
    enc_res = cipher.encrypt(plain_text, nonce)
    
    # just take the first two bytes here, tail is the auth tag
    assert enc_res[0:2] == [63, 106]

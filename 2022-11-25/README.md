# 2022-11-25

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-11-25+Session+2+Notes)

## Homework related to Chapter 3

* [x] **How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?**

For each key, we have 2<sup>64</sup> rows, where a row has a block plaintext (input) and ciphertext (output). Given there are 2<sup>80</sup> keys, this means we need 2<sup>64</sup> * 2<sup>80</sup> = 2<sup>64+80</sup> = 2<sup>144</sup> rows.

In terms of disk space: each row has 2*64 bits = 128 bits = 16 bytes. Approx 3.57x10<sup>29</sup> Petabytes..!

* [x] **Suppose you have a processor that can perform a single DES encryption or decryption operation in 2<sup>-26</sup> seconds. Suppose you also have a large number of plaintext-ciphertext pairs for DES under a single unknown key. How many hours would it take, on average, to find that DES key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of 2<sup>14</sup>processors?**

Suppose we have N pairs of plaintext-ciphertext. If we want to do an exhaustive search, we'd have to take a pair (doesn't matter which one really!) and try to decrypt or encrypt with a key. Repeat this for every possible DES key.

DES keys are 56 bit long. This means 2<sup>56</sup> possible keys. In terms of time, if I can perform one test in 2<sup>-26</sup>s, then I can do an exhaustive search in 2<sup>-26+56</sup> = 2<sup>30</sup> seconds ~ 34yrs.

If I parallelize across 2<sup>14</sup> processors, I can get down to 2<sup>16</sup> seconds ~ 18hrs.

* [ ] **Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES F function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?**

We have something to recover round 1, so we can go from all our plaintexts to the middle-ciphertexts. Similarly, because we have the 48-bit round key for round 2, we can go from all our ciphertexts back to middle-ciphertexts.

Unsure how this translates to a distinguishable attack against "DES2" :thinking:


* [x] **Familiarize yourself with a cryptographic CLI tools. A popular open source package is [OpenSSL](https://docs.rs/openssl/latest/openssl/aes/index.html)**

* [x] **Using an existing cryptographic library, decrypt the following ciphertext (in hex): `53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07` with the following 256-bit key (also in hex): `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using AES**

Using the little Rust program in [aes_utils](./aes_utils/):
```
80706050403020100807060504030201
```

* [x] **Using an existing cryptography library, encrypt the following plaintext (in hex): `29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D` with the following 256-bit key (also in hex) `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.**

Using the little Rust program in [aes_utils](./aes_utils/):
```
80000000000000000000000000000001
```
(special value! That's our key...)

Encrypting with gnupg requires key generation first:
```sh
$ gpg --full-generate-key
```

Now encrypt:
```sh
$ echo -n "80000000000000000000000000000001" | gpg --encrypt --armor -r 72501F2AECD370D5879269E6C2CF3C537A70830B
-----BEGIN PGP MESSAGE-----

hQGMAxdU7lJorCysAQv/Yshza2BX5WTw7smb+BMdAXMtzbGopB0w7acv2LOIvf73
1oUCk7rxBDhWJLfegPbzhOCO+fv4uykV/0sHJsQFGXuoZcQhPcUv34BymhA8Ejes
A8rylQIyU1sfya1/CTHwMNbe1zHILM9BpMbUdnEAjW9CarTjy58GfOnyjCVxX3aF
sKhjufI4ixIroUOFqPfhilPktQmQum43oJNClt2/bX62uqjmJYSvAt6gFq257w3i
xRCPzs7iVGE/K+1qElYCp5HtU/JrBDJwFHKFNIZLQ9vv8kcnWRrpyBL1ahykMIL+
SyD96hrc7YC8miUvSh7fSMzeYX+Nx+HHUcTQV10JGK/GX28Zy+Pll3ifF0iOmTb7
oGUPlND6nbOCrWl4FsS7hrqTc35Wh8DcQ6EPsWSmU+kHhk+4nBNtsgOsDazAbVjD
mpBPvPq00k2RU1LDgSbDFh/iXqtzp+0nE3PxzcUaYuT2n4C6dltvDAnsQa/nE9Q5
ymvovo3fmj5wHla/IDPl1EoBCQIQOHqWrGCyr7oFMY6K+oMxuuBhFL1uEKlDosHT
0K3pyWPm8khmzS6Qz+N3HnzwdZAcipjTKRW0B6FbRwFKWXmgA3/x+/JaQQ==
=ce0y
-----END PGP MESSAGE-----
```

And decrypt:
```
$ cat encrypted.txt | gpg --decrypt
gpg: encrypted with rsa3072 key, ID 1754EE5268AC2CAC, created 2022-12-02
      "Test <test@test.org>"
80000000000000000000000000000001
```

* [ ] **Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key K and a plaintext P and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.**

See [`des_utils`](./des_utils/). Running this program encrypts s with k, then encrypts complement(s) with complement(k). The result is printed byte-by-byte, it's easy to see that:
```
E(complement(s), complement(k)) == complement(E(s, k))
```

Below "orig. byteX" are the bytes of E(s, k), and "comp. byteX" are the bytes of E(complement(s), complement(k)). The bits are all flipped from one line to the other.
```
orig. byte#0: 0b00111011
comp. byte#0: 0b11000100

orig. byte#1: 0b11001101
comp. byte#1: 0b00110010

orig. byte#2: 0b11010100
comp. byte#2: 0b00101011

orig. byte#3: 0b00011110
comp. byte#3: 0b11100001

orig. byte#4: 0b01100001
comp. byte#4: 0b10011110

orig. byte#5: 0b01100101
comp. byte#5: 0b10011010

orig. byte#6: 0b10100101
comp. byte#6: 0b01011010

orig. byte#7: 0b11101000
comp. byte#7: 0b00010111
```

## Homework related to Chapter 4

* [ ] **Let `P` be a plaintext and let `l(P)` be the length of `P` in bytes. Let `b` be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme: "Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number `n` which satisfies `0 < = n <= b-1` and `n + lP` is a multiple of `b`. Pad the plaintext by appending `n` bytes, each with value `n`.**

* [ ] **Suppose you, as an attacker, observe the following 32-byte ciphertext (in hex): `46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D 2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75`. and the following 32-byte ciphertext `C'` (also in hex): `51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30 7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B`. Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext `P` corresponding to `C` is `43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F`. What information, if any, can you infer about the plaintext `P` corresponding to `C'`?**

* [ ] **The ciphertext (in hex): `87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91 7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92 6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17` was generated with the 256-bit AES key (also in hex)`80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise**

* [ ] **Let `P1, P2`, be a message that is two blocks long, and let `P1'` be a message that is one block long. Let `C0`, `C1`, `C2` be the encryption of `P1`, `P2` using CBC mode with a random IV and a random key, and let `C0'`, `C1'` be the encryption of `P1'` using CBC mode with a random IV and the same key. Suppose an attacker knows `P1`, `P2` and suppose the attacker intercepted and thus know `C0`, `C1`, `C2` and `C0'`, `C1'`. Further suppose that, by random chance, `C1` = `C2`. Show that the attacker can compute `P1'`**

* [ ] **Implement a pair of functions: A [PKCS](https://en.wikipedia.org/wiki/PKCS_7) message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.**

## Extra reading

Concurrently to New Directions in Cryptography, IBM published what would become the Data Encryption Standard in 1975, which would become standardized three years later in 1978. IBM originally targeted 64 bits of security, but the NSA offered to disclose vulnerabilities in the algorithm to IBM, in exchange for a lower bit security, 48 bits, so that the NSA would have an easier time breaking encryption by brute force. IBM and the NSA agreed on a midway, 56 bit security, and the NSA patched vulnerabilities in the DES algorithm. The precise contributions of the NSA were kept private, in part to maintain the secrecy of differential cryptanalysis techniques. However, the existence of the collaboration was publicly known and heavily scrutinized by cryptographers, including Whitfield Diffie and Martin Hellman, who argued for the necessity of a 128-bit key.

[Copy A](https://web.archive.org/web/20140226205104/http://origin-www.computer.org/csdl/mags/co/1977/06/01646525.pdf) // [Copy B](https://ee.stanford.edu/~hellman/publications/27.pdf)



# 2022-11-18

This was the intro session. The notes & assigned homework are [here](https://hackmd.io/@thor314/H12nS4SLj)

## Homework related to chapter 1

* [ ] Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

## Homework related to chapter 2
* [ ] Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
* [ ] Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.
* [ ] Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
* [ ] Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.

## Other homework (from Thor's brain)

* [ ] Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?
* Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help: https://cryptography.rs/, https://lib.rs/ (librs is equivalent to [crates.io](https://crates.io/), with a different interface)
* [ ] Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion).
* [ ] You’re implementing a [Tweakable Encryption scheme](https://en.wikipedia.org/wiki/Disk_encryption_theory). You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.
* [ ] You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?
* [ ] Implement the [Vignère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.
* [ ] What is a side channel attack? Is your cipher implementation constant time?
* [ ] Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
* [ ] Extra: Consider ways to contribute what you learned this week to the [Uncloak knowledge graph](https://uncloak.org/).

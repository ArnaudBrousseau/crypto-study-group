# 2022-11-18

This was the intro session. The notes & assigned homework are [here](https://hackmd.io/@thor314/H12nS4SLj)

## Homework related to chapter 1
* [x] **Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.**

In a software company, protecting against supply chain attacks with extra steps in the deployment process can slow down the release cycle (human approvals, automated checks). This slows down the ability for a company to ship fixes / respond to threats fast as a result.

In the same vein: splitting a cryptocurrency wallet seed phrase between 24 participants vastly decreases the risk of theft (all 24 participants would have to be coerced or attacked for that to happen), but it significantly increases the risk of losing funds (only one participant losing its share means the wallet is bricked!); it also increases the risk of denial of service (a participant can go rogue and refuse to give its seed word when asked.

## Homework related to chapter 2
* [x] **Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total?**

for 2 people: 1 key  
for 3 people: (2 people) + 2 keys for the new participant = 3 keys  
for 4 people: (3 people) + 3 keys for the new participant = 6 keys  
for 5 people: (4 people) + 4 keys for the new participant = 10 keys = 4+3+2+1 keys  
...etc...

Formula for `n` people: `1 + 2 + 3 + ... + n-1` = `n-1 * n / 2`

Applied to `n=30`: `29*30/2` = 435 keys

* [x] **Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.**

For this to be true, Bob needs to know that Alice's public key is indeed associated to Alice (out-of-band authentication). We also need to know Alice protected her private key and trust that an attacker did not get access to her private key. And finally, this could simply be a replay of a previous message produced/signed by Alice (does the message contain a nonce? If so, we're in luck).

* [x] **Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?**

Not necessarily. If the ciphertext can be decoded without the secret decryption key (because the encryption scheme is weak and the ciphertext isn't sufficiently random for example), then the encryption scheme isn't secure.

* [x] **Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.**

In this scheme, there are `2^n` different keys. Birthday attacks really are collisions, and we're asking how big `n` should be if we want an attacker to have to try (that is, pick a key from the set of `2^n` keys, then compare to a target victim key) at least `2^128` times before getting a collision (hence providing "128 bit security")

The answer is `n=256` because with `2^256` keys in the set, the birthday bound is `2^(256/2)` = `2^128`.

Now, why is the birthday bound `2^(n/2)` for a set of `2^n` elements? Because we approximate the first collision in a set of `N` elements to happen after choosing `sqrt(N)` elements from that set. Hence, we expect a collision in a set of `2^n` elements to happen after choosing `sqrt(2^n)` = `2^(n/2)`.

## Other homework (from Thor's brain)

* [x] **Suppose you read about RSA encryption and wanted to find it’s standard specification. Where would you look?**

Honest answer: I'd Google it, and consider any trustworthy link: Wikipedia, IETF, NIST, arXiv paper, etc. I suspect RSA evolved a lot over the years so there's most likely multiple versions of the standard. I'd consider the latest RSA standard the best if there are multiple available.

I did this just now, the most useful source is definitely wikipedia, which has a history, link to the different standards, and a solid explanation. Go wikipedia!

* [x] **Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help: https://cryptography.rs/, https://lib.rs/ (`librs` is equivalent to [crates.io](https://crates.io/), with a different interface)**

**RSA**

In pure Rust: https://docs.rs/rsa/latest/rsa/. This feels like solid Rust code with multiple modules + features.

Via OpenSSL: https://docs.rs/openssl/latest/openssl/rsa/. This library wraps the C openssl library. Lots of `unsafe` blocks to call into FFIs (foreign function interfaces).

**TLS/SSL**

RustTLS: https://docs.rs/rustls/latest/rustls/. Written in pure Rust on top of Brian Smith' [ring](https://github.com/briansmith/ring).

OpenSSL: https://docs.rs/openssl/latest/openssl/. Same story than `openssl/rsa`: lots of `unsafe` blocks and FFIs.

**AEAD**

Via RustCrypto: https://docs.rs/aes-gcm/latest/aes_gcm/. Again, this seems like a really well engineered crate. It has common traits shared with other crates in the RustCrypto ecosystem (e.g. `GenericArray`). Tempting to keep using the crates within the `RustCrypto` ecosystem once you start using one of them.

https://docs.rs/ring/latest/ring/aead/index.html. Ring is a minimal library. The docs are very sparse. The code seems solid but dense. An interesting bench example using macros: https://github.com/briansmith/ring/blob/main/benches/aead.rs. I'm gathering that this library is meant to be a "core" crate, useful for other crypto crate authors, rather than a crate used by end-users directly. Might explain the lack of docs? Still not a great excuse IMO.

* [x] **Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion).**

Benchmarking AES-GCM keygen: see [aes_gcm_keygen/](./aes_gcm_keygen/) for the code. Results:
```
     Running benches/keygen.rs (/Users/arnaud/git/crypto-study-group/target/release/deps/keygen-a070764bec5a4974)
Gnuplot not found, using plotters backend
Ring's AES-GCM keygen   time:   [342.98 ns 343.79 ns 344.64 ns]                                  
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

RustCrypto's AES-GCM keygen                                                                            
                        time:   [245.62 ns 246.15 ns 246.75 ns]
Found 17 outliers among 100 measurements (17.00%)
  12 (12.00%) high mild
  5 (5.00%) high severe
```

* [ ] **You’re implementing a [Tweakable Encryption scheme](https://en.wikipedia.org/wiki/Disk_encryption_theory). You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.**

Not sure I understand what a Tweakable Encryption Scheme is yet. Must read on this first!

* [x] **You want to understand a paper on a new polynomial commitment scheme, but you’ve been trying for more than an hour, and the math is over your head. What do you do?**

Use the Uncloak discord and ask to see if there's a good soul willing to unblock me! Before that though: use Google? It's really rare that Google doesn't yield something helpful: wikipedia articles, [math.stackexchange](https://math.stackexchange.com/) posts, papers, forum posts, etc.

* [x] **Implement the [Vigenère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.**

Implemented in [vigenere/](./vigenere/) in 95 lines total, or 53 lines if we exclude tests!

* [x] **What is a side channel attack? Is your cipher implementation constant time?**

A side-channel attack is defined as an attack which makes use of information leakage related to the context of the computation. For example: time it was performed, time it took to perform it, length of the result, etc.

The implementation of the Vigenère cipher isn't constant time given that:
1) we have a step to convert the key to a vec of ints. The longer the key, the slower this is
2) the length of the plaintext dictates the length of the ciphertext. This means the longer the plaintext, the longer the computation. And we're leaking the length of the plaintext with the ciphertext.

So, all-in-all, pretty poor encryption scheme we have here!

* [ ] **Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638)**

TODO

* [ ] **Extra: Consider ways to contribute what you learned this week to the [Uncloak knowledge graph](https://uncloak.org/)**

TODO

## Other random notes

In chapter 2: "one of the classical mistakes in cryptography is to think that encrypting a message also stops Eve from changing it. It doesn't."

I'm not sure that's true. If we're talking about a strong symmetric encryption scheme where keys are exchanged already, an attacker modifying the messages would simply cause decryption failures. They wouldn't be able to change the message in a way that results in successful decryptions :thinking:

In "New Directions in Cryptography":
* "Unconditional security results from the existence of multiple solutions to a cryptogram. For example, the simple substitution cryptogram XMD resulting from English text can represent the plaintext messages: now, and, the, etc." ==> what? Given a single key/algorithm, there should only be a single solution to a cryptogram right?
* "The only unconditionally secure system in common use is the one time pad" => How so? An attacker could break the encryption if it's iterating over all the possible keys. Would love to see the "proof"
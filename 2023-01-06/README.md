# 2023-01-06
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-01-06+Session+6+Notes).

## Homework related to Chapter 9

* [x] **Be familiar with the difference between a PRNG and a [CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator), and how the latter can be generated from any secure block cipher**

**PRNGs:** designed to give a stream of statistically-sound bytes. That is, if you perform statistical analysis on the output of a good PRNG, it should look like randomness. However, they're deterministic and based on a seed.

**CSPRNGs:** a PRNG where the internal state is continuously updated with true randomness. For example, Fortuna. This means that even in the case where the internal state of the PRNG is leaked at some point in time, given enough time, the internal state will go back to looking "random" to an attacker. This isn't the case for a PRNG. If the seed is leaked, the output is 100% predictable from that point onwards.

* [x] **Aim to understand the Fortuna construction, now standard in Apple devices. A Rust implementation can be found here: [crypto::fortuna](https://nicolasdp.github.io/git/crypto/fortuna/) - Rust , also see [wikipedia](https://en.wikipedia.org/wiki/Fortuna_(PRNG)).**

Fortuna is actually pretty intuitive: 32 entropy "buckets" or "pools" are made available. Each entropy source on the computer (e.g. mouse, keyboard, hard drive driver) is expected to fill pool equally. Whenever a pool is full of entropy, it is emptied and its entropy is mixed into the CSPRNG state. The "mixing in" is done by setting K to `SHA256(K||<new entropy>)`.

At the core, a block cipher is used to generate blocks of random data. The generator has a key K (refreshed and reseeded periodically) and a counter C. Just like any PRNG, the generator uses a block cipher encryption function E to produce blocks of pseudorandom data. When a request for random data is done the key K is rotated to the result of 2 new blocks. If we ask for 3 blocks of random data, we get `E(K, C)`, `E(K, C+1)`, `E(K, C+2)`, then the key K is rotated to `E(K, C+3)||E(K,C+4)`.

* [ ] **Pick one implementer of `CryptoRng`, and explain how the Rng generates values. See [CryptoRng in rand](https://rust-random.github.io/rand/rand/trait.CryptoRng.html) for a list.**

## Homework related to Chapter 10

* [x] **Be familiar with the definition of a [Sophie Germain Prime](https://en.wikipedia.org/wiki/Safe_and_Sophie_Germain_primes#Sophie_Germain_prime), and [Schnorr group](https://en.wikipedia.org/wiki/Schnorr_group). We may take 2 weeks to cover chapter 10 on primes, depending on group sentiment. As an extra resource, also see [this](https://medium.com/snips-ai/prime-number-generation-2a02f28508ff) blog post.**

Sophie Germain primes are essentially the complement to safe Primes. Safe primes are primes if and only if they can be written in the form `2*q+1`, where `q` is a prime. `q` is called a Sophie Germain prime.

Schnorr groups are actually **sub-groups** of prime order. Given a large prime `p`, if a prime `q` can be found such that `p=qr+1` (note: for `r=2`, q is a Sophie Germain prime!), then the subgroup of order `q` is called a Schnorr group. To find a generator for the group, picking a random number `h` and check that `h^r != 1` and that `(h^r)^q = 1 (mod p)`. Then `h^r` is a generator of the subgroup of order `q`, aka "Schnorr group"!

* [x] **The Extended Euclidean Algorithm "means" that obtaining inverses is not "hard". Explain what hardness means in this context, and why the EEA obtains this result.**

A "hard" operation means that it can't be solved in polynomial time. EEA computes inverses in polynomial time (TODO: demonstrate this? What's the exact runtime?), proving "by construction" that obtaining inverses is not hard.

## Homework related to Rust

* [ ] **Using the [typenum crate](https://docs.rs/typenum/latest/typenum/), modify the following struct to wrap a generically sized array. Implement a new, len, and get_index method for the array.**
```rust
struct GenArrayWrapper<T>{
    inner: Vec<T>
}
```

## Extra reading

* [ ] A recent development in secure cryptographic implementation is the development of auditing and secure design tools, including Jasmine, Vale, Z3, and others. See [this paper](https://hal.inria.fr/hal-03046757/file/BarbosaetalOakland21.pdf) for an overview.
* [x] A fun read: [The Grug Brained Developer](https://grugbrain.dev/)

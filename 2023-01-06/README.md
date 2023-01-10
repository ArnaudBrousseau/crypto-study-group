# 2023-01-06
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-01-06+Session+6+Notes).

## Homework related to Chapter 9

* [ ] **Be familiar with the difference between a PRNG and a [CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator), and how the latter can be generated from any secure block cipher**

* [ ] **Aim to understand the Fortuna construction, now standard in Apple devices. A Rust implementation can be found here: [crypto::fortuna](https://nicolasdp.github.io/git/crypto/fortuna/) - Rust , also see [wikipedia](https://en.wikipedia.org/wiki/Fortuna_(PRNG)).**

* [ ] **Pick one implementer of `CryptoRng`, and explain how the Rng generates values. See [CryptoRng in rand](https://rust-random.github.io/rand/rand/trait.CryptoRng.html) for a list.**

## Homework related to Chapter 10

* [ ] **Be familiar with the definition of a [Sophie Germain Prime](https://en.wikipedia.org/wiki/Safe_and_Sophie_Germain_primes#Sophie_Germain_prime), and [Schnorr group](https://en.wikipedia.org/wiki/Schnorr_group). We may take 2 weeks to cover chapter 10 on primes, depending on group sentiment. As an extra resource, also see [this](https://medium.com/snips-ai/prime-number-generation-2a02f28508ff) blog post.**

* [ ] **The Extended Euclidean Algorithm "means" that obtaining inverses is not "hard". Explain what hardness means in this context, and why the EEA obtains this result.**

## Homework related to Rust

* [ ] **Using the [typenum crate](https://docs.rs/typenum/latest/typenum/), modify the following struct to wrap a generically sized array. Implement a new, len, and get_index method for the array.**
```rust
struct GenArrayWrapper<T>{
    inner: Vec<T>
}
```

## Extra reading

* A recent development in secure cryptographic implementation is the development of auditing and secure design tools, including Jasmine, Vale, Z3, and others. See [this paper](https://hal.inria.fr/hal-03046757/file/BarbosaetalOakland21.pdf) for an overview.
* A fun read: [The Grug Brained Developer](https://grugbrain.dev/)

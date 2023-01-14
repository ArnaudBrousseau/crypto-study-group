# 2023-01-13
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-01-13+Session+7+Notes).

This week, although the assigned reading is chapter 11 & 12, homework is largely related to chapters 9/10!

## Homework related to Chapter 9

* [ ] **What is entropy? Why does a combination (eg. by XOR, or by hash-concatenation as on p145, Reseed) of two or more independent input-streams {S<sub>1</sub>..S<sub>n</sub>} of entropy has at least H(X)>max{S<sub>i</sub>} entropy; that is, why a combination of entropy streams is always at least as entropic as the most entropic stream.**. Note the word independent: an adaptive adversary controlling stream S<sub>i</sub> may choose a function of the other streams to control the randomness.

* [ ] **Give a one-sentence explanation of the difference between a PRNG and a CSPRNG (non-correlation versus unpredictability)**

* [ ] **Why can a CSPRNG can be constructed from a block cipher?**

## Homework related to Chapter 10

* [ ] **Compute 397<sup>-1</sup> mod 2357 by hand.**

* [ ] **Recursively implement the Extended Euclidean Algorithm. Use the above exercise as a test case.**

* [ ] **Implement the Miller-Rabin primality check.** ([reference implementation](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#213))

## Homework related to Chapter 11

* Using [common.rs.html](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#106), obtain a safe prime of n>1000 bits.

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

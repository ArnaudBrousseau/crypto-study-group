# 2023-01-13
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-01-13+Session+7+Notes).

This week, although the assigned reading is chapter 11 & 12, homework is largely related to chapters 9/10!

## Homework related to Chapter 9

* [x] **What is entropy? Why does a combination (eg. by XOR, or by hash-concatenation as on p145, Reseed) of two or more independent input-streams {S<sub>1</sub>..S<sub>n</sub>} of entropy has at least H(X)>max{S<sub>i</sub>} entropy; that is, why a combination of entropy streams is always at least as entropic as the most entropic stream.** Note the word independent: an adaptive adversary controlling stream S<sub>i</sub> may choose a function of the other streams to control the randomness.

It seems intuitive that to predict the combination of {S<sub>1</sub>..S<sub>n</sub>}, one has to predict all the sources. If an attacker knows all sources but S<sub>k</sub> for a given `k`, then the entropy of the combination is the entropy of S<sub>k</sub> from the point of view of the attacker. This can be generalized as saying: predicting a combination of sources is at least as hard as its predicting strongest source. In mathematical terms: H(x)>max({S<sub>i</sub>}).

* [x] **Give a one-sentence explanation of the difference between a PRNG and a CSPRNG (non-correlation versus unpredictability)**

A PRNG solves the problem of producing output which _looks_ random; a CSPRNG solves the problem of producing output which _is_ random from a cryptographic point-of-view. In particular: PRNGs are defeated if their internal state leaks out. CSPRNGs aren't because the internal state is continually mixed in with real entropy sources.

* [x] **Why can a CSPRNG can be constructed from a block cipher?**

A block cipher is unpredictable. Given a key and a counter, the output is determined but unpredictable to an attacker. As long as the seed (initial state, containing the key fed to the cipher) is truly random, so it the output!

## Homework related to Chapter 10

* [ ] **Compute 397<sup>-1</sup> mod 2357 by hand.**

* [ ] **Recursively implement the Extended Euclidean Algorithm. Use the above exercise as a test case.**

* [ ] **Implement the Miller-Rabin primality check.** ([reference implementation](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#213))

## Homework related to Chapter 11

* [ ] **Using [common.rs.html](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#106), obtain a safe prime of n>1000 bits.**
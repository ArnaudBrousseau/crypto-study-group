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

* [x] **Compute 397<sup>-1</sup> mod 2357 by hand.**

Both 397 and 2357 are prime numbers (see tests in [`is_prime`](./is_prime/)), which means we can find (u, v) such that 397u + 2357v = GCD(397, 2357) = 1 (through the Extended Euclidean Algorithm)

This implies:
```
397u = 1-2357v
397u = 1 (mod 2357)
u = 1/397 (mod 2357)
```

Now let's apply the Extended Euclidean Algorithm manually. We start with the trivial:
```
1*397 + 0*2357 = 397
uc      vc       c
0*397 + 1*2357 = 2357
ud      vd       d
```

After this we compute `q = d/c`:
```
2357 = 5*397 + 372
==> q = 5
==> c = 372, d = 397
==> uc = 0-5*1 = -5, vc = 1-5*0 = 1
==> ud = 1, vd = 0

-5*397 + 1*2357 = 372
uc       vc       c
1*397 + 0*2357 = 397
ud      vd       d
```

Same thing repeats! `q = d/c`:
```
397 = 1*372 + 25
==> q = 1
==> c = 25, d = 372
==> uc = 1-1*(-5) = 6, vc = 0-1*1 = -1
==> ud = -5, vd = 1

6*397 + -1*2357 = 25
uc      vc        c
-5*397 + 1*2357 = 372
ud       vd       d
```

Ditto, `q = d/c`!
```
372 = 14*25 + 22
==> q = 14
==> c = 22, d = 25
==> uc = -5-14*6=-95, vc = 1-14*(-1)=15
==> ud = 6, vc = -1

-89*397 + 15*2357 = 22
uc        vc        c
6*397 + -1*2357 = 25
ud       vd       d
```

Here we go again! Guess what? `q = d/c`:
```
25 = 1*22 + 3
==> q = 1
==> c = 3, d = 22
==> uc = 6-(1*-89) = 95, vc = -1-1*15 = -16
==> ud = -95, vd = 16

95*397 + -16*2357 = 3
uc        vc        c
-89*397 + 15*2357 = 22
ud        vd        d
```

And again:
```
22 = 7*3 + 1
==> q = 7
==> c = 1, d = 3
==> uc = -89-7*95 = -754, vc = 15-7*(-16) = 127
==> ud = 95, vd = -16

-754*397 + 127*2357 = 1
uc         vc         c
95*397 + -16*2357 = 3
ud       vd         d
```

We can go one more time, just to conclude and get to `c=0`:
```
3 = 3*1 + 0 
==> q = 3
==> c = 0, d = 1
==> uc = 95-3*(-754) = 2357 , vc = -16-3*127 = -397
==> ud = -754, vd = 127

2357*397 + -397*2357= 0
uc         vc         c
-754*397 + 127*2357 = 1
ud         vd        d
```

In conclusion, we have `-754*397 + 127*2357 = 1`, which means `-754*397 = 1 - 127*2357`, and `-754 = 1/397 (mod 2357)`.

If we actually want the proper inverse (in the range 0 <= inv <= 2357), the answer is -754 + 2357 = **1603**.

To verify the 1603 is the inverse of 397 modulo 2357, we check that 1603*397 = 1 (mod 2357) -- this has been verified! :white_check_mark:

* [x] **Recursively implement the Extended Euclidean Algorithm. Use the above exercise as a test case.**

Done in [`eea`](./eea/).

* [ ] **Implement the Miller-Rabin primality check.** ([reference implementation](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#213))

## Homework related to Chapter 11

* [ ] **Using [common.rs.html](https://docs.rs/pumpkin/2.0.1/src/pumpkin/common.rs.html#106), obtain a safe prime of n>1000 bits.**
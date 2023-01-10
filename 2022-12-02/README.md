# 2022-12-02

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-02+Session+3+Notes).

## Homework related to Chapter 5

* [x] **Exercise 5.3 Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first `n` bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each `n`. How long would you expect your program to take for SHA-512-256? For SHA-512?**

See in [`sha512n`](./sha512n/) for implementation.

For n = 16 we expect `sqrt(n)` = 2<sup>8</sup> = 256 tries on avg (collision). For n = 256, we expect 2<sup>128</sup> tries.

So...1.33.10<sup>36</sup> iterations. Oooof!

Tried with a few values: (base = "w" / "rno")
- n = 16 is quick: 0.02s / 0.59s
- n = 18 takes 0.23s / 14.38s
- n = 20 takes 2.90s / 49.81s
- n = 21 takes...didn't want to wait!

* [x] **Exercise 5.4 Let SHA-512-n be as in the previous exercise. Write a program that finds a message M that hashes to the following value under SHA-512-16 (in hex): 3D 4B. How many tries would you expect the algorithm to need? Running the algorithm 5 times, How many tries did it take on average?**

See in [`sha512n`](./sha512n/) for code. Running it yields the following collisions:
```sh
Collision found! "world26711" hashes to 3D4B... with SHA-512
Collision found! "foo24641" hashes to 3D4B... with SHA-512
Collision found! "bar69648" hashes to 3D4B... with SHA-512
Collision found! "baz2674" hashes to 3D4B... with SHA-512
```

Verified with:
```sh
$ echo -n 'world26711' | shasum -a 512
3d4b1d8334a65198633651ec7ebc8ca7ca42a3b67177c1146f8a894b2846a72c23769d5d97e3f3cde080a9958f0c7def21d1ff374f3a10ea0e02f7baca1718d9  -
```

The number at the end of each prefix is the number of tries. So, it varies widely. On average, after 4 attempts, it takes ~31k iterations. Not statistically significant, but it's far from what you'd expect: finding a collision in a N=2<sup>16</sup> elements space should take `sqrt(N)` = 2<sup>8</sup> = 256 tries. It's much closer to `N/2` = 2<sup>15 = 32,768 iterations.

* [x] **With command line tools or Criterion, benchmark the [blake3 hash](https://docs.rs/blake3/latest/blake3/) (default is 256 bit output), and compare it to benches of [SHA3-256](https://docs.rs/sha3/latest/sha3/) and [SHA-256](https://docs.rs/sha2/latest/sha2/) (when written without a number, SHA is assumed to be SHA2).**

Benchmark and code at [hash_fns](./hash_fns/). Results:
```
Blake3 hash             time:   [109.65 ns 109.90 ns 110.19 ns]                        
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) high mild
  15 (15.00%) high severe

SHA3-256 hash           time:   [270.06 ns 270.80 ns 271.77 ns]                          
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

SHA-256 hash            time:   [223.64 ns 224.03 ns 224.52 ns]                         
Found 20 outliers among 100 measurements (20.00%)
  14 (14.00%) high mild
  6 (6.00%) high severe
```

## Homework related to Chapter 6

* [x] **Suppose a and b are both one block long, and suppose the sender MACs a, b, and `a||b` with CBC-MAC. An attacker who intercepts the MAC tags for these messages can now forge the MAC for the message `m = b||(M(b) xor M(a) xor b)`, which the sender never sent. The forged tag for this message is equal to `M(a||b)`, the tag for `a||b`. Justify mathematically why this is true.**

In this question we must prove that `M(m) = M(a||b)`.

We'll use the fact that under CBC-MAC, for any 1-block messages `a` and `b`, `M(a||b) = M(a) xor b` (1)

We'll also use the fact that for any 1-block message, `a xor a xor b = b` (2)

`M(m) = M(b||(M(b) xor M(a) xor b))` by definition of `m`<br>
`     = M(b) xor (M(b) xor M(a) xor b)` by (1)<br>
`     = M(b) xor M(b) xor M(a) xor b` by `xor`'s associativity/commutativity<br>
`     = M(a) xor b` by (2)<br>
`     = M(a||b)` by (1)<br>
QED

* [x] **Suppose message `a` is one block long. Suppose that an attacker has received the MAC `t` for `a` using CBC-MAC under some random key unknown to the attacker. Explain how to forge the MAC for a two-block message of your choice. What is the two-block message that you chose? What is the tag that you chose? Why is your chosen tag a valid tag for your two-block message?**

In this case we have `t = M(a)`. We cannot forge new MACs because the key is unknown.

We can, however, forge `M(a||x) = M(a) xor x = t xor x`. That's a length extension attack possible due to the nature of CBC-MAC.

* [x] **Using an existing cryptography library, compute the MAC of the message: `4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20` using CBC-MAC with AES and the 256-bit key: `80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`**

Requires OpenSSL > 3.0:
```sh
$ echo -n "4D4143732061726520766572792075736566756C20696E2063727970746F677261706879212020202020202020202020" | xxd -r | ~/openssl/bin/openssl mac -cipher AES-256-CBC -macopt hexkey:8000000000000000000000000000000000000000000000000000000000000001 CMAC
39522862878554F1EC6628F147B74333
```

* [ ] **For message authentication, when would you use TupleHash? ParallelHash? KMAC?**

## Extra Reading

Complementing the outdated book references:

The reading is somewhat outdated this week. Several additional reading assignments to cover bases on hash functions:

* On the history of hash function attacks: we're pretty good at pre-image resistance! Collision resistance too lately. [Lessons From The History Of Attacks On Secure Hash Functions - Electric Coin Company](https://web.archive.org/web/20220708064142/https://electriccoin.co/blog/lessons-from-the-history-of-attacks-on-secure-hash-functions/)
* SHA2 and SHA3 are quite slow for hash functions. Blake3 is a 2-year-old, extremely fast hash function, based on the ChaCha stream cipher. [BLAKE (hash function) - Wikipedia](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3).
* Analysis of collision resistance: [Analyzing the MD5 collision in Flame | Trail of Bits Blog](https://blog.trailofbits.com/2012/06/11/analyzing-the-md5-collision-in-flame/)
* Reading on Keccak/SHA-3 (standardized 2016), to understand the Sponge construction for hashes.
  * By the Keccak authors: [Keccak Sponge](https://keccak.team/sponge_duplex.html)
  * To read more, see: https://www.crypto-textbook.com/download/Understanding-Cryptography-Keccak.pdf
  * NIST on SHA3 standards [Hash Functions | CSRC](https://csrc.nist.gov/projects/hash-functions/sha-3-project)
* Keccak-derived authentication functions: [SHA-3 Derived Functions: cSHAKE, KMAC, TupleHash and ParallelHash | NIST](https://www.nist.gov/publications/sha-3-derived-functions-cshake-kmac-tuplehash-and-parallelhash)
# 2023-02-24
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-24+Session+13+Notes).

## Assigned reading

* [x] Sections 5.1 through 5.5 (basic facts about elliptic curves)
* [x] Section 5.8 on bilinear pairings

## Homework

* [x] **Migrate [this toy elliptic curve](https://github.com/cjeudy/EllipticCurves) implementation to Rust.**

* [x] **Review the [Rust Crypto elliptic-curves library](https://github.com/RustCrypto/elliptic-curves). What techniques could you use to improve your implementation?**

One thing I could use is the type argument (e.g. `PrimeCurveParams`) to abstract away the definition of the arithmetic from the actual curve. The use of traits is super impressive, especially when looking at how well they compose together. For example, see [CurveArithmetic](https://github.com/RustCrypto/traits/blob/f0dbe44fea56d4c17e625ababacb580fec842137/elliptic-curve/src/arithmetic.rs#L14) :o
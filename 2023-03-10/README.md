# 2023-03-10
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-03-10+Session+14+Notes).

## Assigned reading

* [x] Shamir's secret sharing and the [Pedersen DKG](https://thork.net/posts/2022_4_21_dkg/)
* [x] BLS signatures: read [this gist](https://gist.github.com/paulmillr/18b802ad219b1aee34d773d08ec26ca2)
* [x] [Yao's garbled circuits](https://cronokirby.com/posts/2022/05/explaining-yaos-garbled-circuits/)

## Homework

* [x] **Implement a BLS signature using the pairing library exposed in [Arkworks](https://github.com/Pratyush/algebra-intro).**

Finally done, and more! Implemented the [BLS Signature IETF draft (v5)](https://www.ietf.org/archive/id/draft-irtf-cfrg-bls-signature-05.html#section-4.2.3) using [Arkworks's algebra crates](https://github.com/arkworks-rs/algebra).

Source: [here](./bls12-381/)
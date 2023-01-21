# 2023-01-20
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-01-20+Session+8+Notes).

## Assigned reading

* [ ] Most of chapter 13 may be skipped, though 13.5.6 on replay attacks is worth covering.
* [ ] Chapter 14 on key negotiation explores a protocol to obtain a secret session key from an existing secret shared key, for forward secrecy
* [ ] It's worth a light read of chapter 15 & 16. 

## Homework related to number theory

* [ ] **Suppose that `x = 2 (mod 30)`. What is obtained about `x (mod 2)`, `x (mod 3)`, and `x (mod 5)`? What about `x (mod 7)`?**

* [ ] **You're Eve, intercepting a message from Alice to Bob. Alice asked Bob to choose a prime larger than 30 to construct a prime field. You choose `p=31` and `g=2`. How many unique choices of exponent x in g<sup>x</sup> = a (mod p) does Alice now have? A unique choice is any uniquely obtainable values for a. For instance, 2<sup>17</sup> = 4 and 2<sup>32</sup> = 4 are not unique.**

* [ ] **Bob would have chosen `p=83` and `g=2`. How many unique choices for does Alice have now? What if Bob choses `p=83` and `g=3`?**

* [ ] **576001 is the prime with factorization p-1 = 2<sup>9</sup> * 3<sup>2</sup> * 5<sup>3</sup>. Find a generator $g \ne 1$ with order |g| < 10**

## Extra reading

* [x] Bonus reading for this week: [Don't use RSA](https://raw.githubusercontent.com/trailofbits/publications/master/papers/rsagtfo.pdf). This PDF is the written version of [this talk](https://blog.trailofbits.com/2019/07/08/fuck-rsa/).
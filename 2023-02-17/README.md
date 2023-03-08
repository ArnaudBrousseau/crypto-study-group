# 2023-02-17
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-17+Session+12+Notes).

## Assigned reading

* [x] Chapter 4.3 on Probability Theory
* [x] Sections 7.1-3 on digital signatures
* [x] (Optional) Chapter 4 gives a survey of a wide range of cryptography-related concepts

## Homework related to Chapter 2

* [x] **What kind of security would you expect an encryption scheme to obtain?**

The encryption scheme "kinds" are a reference to [this discussion](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-17+Session+12+Notes#Discussion). In short:
* Perfect security is rare and requires (for encryption schemes) an inconveniently big amount of key material. [One-time pads](https://en.wikipedia.org/wiki/One-time_pad) are an example of perfect security, and require 1 bit of key material per bit of plaintext.
* Statistical security requires a process output to be statistically the same than a target. If used in the context of encryption, we have to study how random-looking the resulting ciphertext is, given enough plaintexts and keys.
* Computational security defines the number of CPU cycles or operations that a computer needs to perform to attack a scheme. In the context of encryption, we have to study how many operations are needed to find collisions (collision attacks), or find plaintexts given ciphertexts (ciphertext-only attack), or find a key given pairs of plaintexts/ciphertexts (known-plaintext, chosen-plaintext, or chosen-ciphertext attacks)

In summary: encryption can be perfectly secure in the case of one-time pads, statistically secure if the ciphertexts are perfectly random, and computationally secure if an adversary has to spend 2^128 cycles or more to attack the encryption (if we're targeting 128 bit security, as usual)

Hence: it's reasonable to expect computational security, a bit more ambitious to expect statistical security, and unreasonable to expect perfect security from an encryption scheme (although: it's possible to achieve with OTPs!)

* [x] **Win the [Kelly Criterion Game](https://explore.paulbutler.org/bet/). Then read the article and re-derive the Kelly criterion equation with probability of winning `p`, bet size `l`. Now suppose your bet pays out with some ratio `r`; e.g. set `r=1` for the prior game, but if `r=2`, a bet size of `l` pays back out `(r+1)l=3l` for victory, and nothing for a loss. Adjust your derivation to account for `r`.**

Re-deriving the base equation: we're placing a bet sized `l` (expressed in % of money pool), winning with probability `p`, and losing with probability `(1-p)`.

If we start with a single dollar and play n rounds, we expect `n*p` rounds to be winning rounds (multiplying our money pool by `1+l`), and `n*(1-p)` rounds to be losing rounds (multiplying our money pool by `1-l`).

Hence, the expected amount of money after `n` rounds is:

$$(1+l)^{np}(1-l)^{n*(1-p)}$$

Normalizing for a single round, we have the following function:

$$f(l) = (1+l)^{p}(1-l)^{1-p}$$

We're looking to maximize this function: what value of $l$ does this (with $0 < l < 1$)?

Instead of maximizing this function, let's consider $g = log(f)$:
    $$g(l) = log((1+l)^{p}(1-l)^{1-p})$$
    $$g(l) = p*log(1+l) + (1-p)*log(1-l)$$

Now, to maximize `g`, let's find points were $g'(l) = 0$. 

Given $\frac{d}{dx}log(f(x)) = \frac{f'(x)}{f(x)}$, we have:

$$g'(x) = p*\frac{1}{1+l}+(1-p)*\frac{-1}{1-l}\$$
$$= \frac{p}{1+l}-\frac{1-p}{1-l}$$

Now finding points where $g'(x)$ is 0:

$$g'(x) = 0$$
$$\frac{p}{1+x}-\frac{1-p}{1-x} = 0$$
$$\frac{p}{1+x}=\frac{1-p}{1-x}$$
$$p*(1-x) = (1-p)(1+x)$$
$$p - px = 1 + x - p - px$$
$$x = 2p - 1$$

This is the result shown in the blog post $\square$

----

Now let's take into account a new parameter `r`, whereby a win returns `rl`, and a loss cost `l`. The function to consider becomes:

$$f(l) = (1+rl)^{p}(1-l)^{1-p}$$

And maximizing the derivative of the log of this function gives:

$$\frac{d(log(f(x)))}{dx} = 0$$
$$\frac{d(log((1+rx)^{p}(1-x)^{1-p}))}{dx} = 0$$
$$\frac{d(p*log((1+rx)+(1-p)*log(1-x)))}{dx} = 0$$
$$p\frac{d(log(1+rx))}{dx}+(1-p)\frac{d(log(1-x))}{dx} = 0$$
$$p*\frac{r}{1+rx}+(1-p)\frac{-1}{1-x} = 0$$
$$\frac{pr}{1+rx} = \frac{1-p}{1-x}$$
$$pr*(1-x) = (1+rx)*(1-p)$$
$$pr - prx = 1 - p + rx - prx$$
$$x = \frac{pr + p - 1}{r}$$

* [x] **Spend 30 minutes reading the [RustCrypto Schnorr implementation over k256](https://github.com/RustCrypto/elliptic-curves/blob/master/k256/src/schnorr.rs). You may see words you do not recognize, like `affine` and `projective` points. Post a comment in the discord about any new words you learned to discuss with the group.**

## Extra: nonce reuse attacks in ECDSA

Let's demonstrate how one can extract the private key!

If we reuse $k$ and sign 2 different messages under ECDSA we have:
$$
r_1 = g^k \\
s_1 = k^{-1}(H(m_1)+xr_1) \\
and\\
r_2 = g^k \\
s_2 = k^{-1}(H(m_2)+xr_2)
$$

Note that $r_1 = r_2 = g^k$ (I simply call this $r$ from now on). We can compute $k^{-1}$ with the definition of $s_1$:
$$
k^{-1} = \frac{s_1}{H(m_1) + xr}
$$

And inject this in the definition of $s_2$:
$$
s_2 =  \frac{s_1}{H(m_1) + xr} * (H(m_2)+xr_2)
$$

Hence:
$$
s_2H(M_1)+s_2xr = s_1H(m_2) - s_1xr_2
$$

And the private key is defined fully in terms of public values! :warning:
$$
x = \frac{s_1H(m_2) - s_2H(m_1)}{r(s_2-s_1)}
$$
# 2023-02-17
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-17+Session+12+Notes).

## Assigned reading

* [ ] Chapter 4.3 on Probability Theory
* [ ] Sections 7.1-3 on digital signatures
* [ ] (Optional) Chapter 4 gives a survey of a wide range of cryptography-related concepts

## Homework related to Chapter 2

* [ ] **What kind of security would you expect an encryption scheme to obtain?**

* [ ] **Win the [Kelly Criterion Game](https://explore.paulbutler.org/bet/). Then read the article and re-derive the Kelly criterion equation with probability of winning `p`, bet size `l`. Now suppose your bet pays out with some ratio `r`; e.g. set `r=1` for the prior game, but if `r=2`, a bet size of `l` pays back out `(r+1)l=3l` for victory, and nothing for a loss. Adjust your derivation to account for `r`.**

* [ ] **Spend 30 minutes reading the [RustCrypto Schnorr implementation over k256](https://github.com/RustCrypto/elliptic-curves/blob/master/k256/src/schnorr.rs). You may see words you do not recognize, like `affine` and `projective` points. Post a comment in the discord about any new words you learned to discuss with the group.**

## Nonce reuse attacks in ECDSA

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
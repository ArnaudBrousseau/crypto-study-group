# 2023-02-03
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-03+Session+10+Notes).

## Assigned reading

* [x] Chapter 1 of [An Introduction to Mathematical Cryptography](https://www.amazon.com/Introduction-Mathematical-Cryptography-Undergraduate-Mathematics/dp/1441926747/ref=sr_1_3?keywords=an+introduction+to+mathematical+cryptography&qid=1675619872&sprefix=an+introduction+to+mat%2Caps%2C141&sr=8-3&ufe=app_do%3Aamzn1.fos.006c50ae-5d4c-4777-9bc0-4513d670b6bc)

## Homework related to Chapter 1

* [x] **Re-calculate by EEA 397<sup>-1</sup> in the group modulo 2357, with the tabular method given on page 19.**

Let's apply the Euclidian algorithm first:
```
2357 = 5  * 397 + 372
397  = 1  * 372 + 25
372  = 14 * 25  + 22
25   = 1  * 22  + 3
22   = 7  * 3   + 1
3    = 3  * 1   + 0   <- END
```
The $Q_n$ series is thus (5, 1, 14, 1, 7, 3):

| | |5|1|14|1|7|3|$q_n$|
|-|-|-|-|-|-|-|-|-|
|0|1|.|.|.|.|.|.|$P_n$|
|1|0|.|.|.|.|.|.|$Q_n$|

Completing the table by computing
* $P_n = P_{n-1}*q_n + P_{n-2}$
* $Q_n = Q_{n-1}*q_n + Q_{n-2}$

This results in:

| | |5|1|14|1|7|3|$q_n$|
|-|-|-|-|-|-|-|-|-|
|0|1|5|6|89|95|754|2357|$P_n$|
|1|0|1|1|15|16|127|397|$Q_n$|

Hence: $2357*127-754*397 = 1$

And so $397^{-1} = -754+2357 = 1603$. This agrees with what we computed and verified [previously](https://github.com/ArnaudBrousseau/crypto-study-group/blob/9b4ac6957c2f9483fc86ea06f3dfdc89b4215be7/2023-01-13/README.md?plain=1#L129) :white_check_mark:

* [x] **Implement the modular Fast Powering Algorithm for big ints**

Already done as part of my Rabbin-Miller implementation! See [this](https://github.com/ArnaudBrousseau/crypto-study-group/blob/9b4ac6957c2f9483fc86ea06f3dfdc89b4215be7/2023-01-13/miller_rabin/src/lib.rs#L88-L109)!

* [x] **For positive $a,b \in \Z$, suppose $\exist u,v$ satisfying $au+bv=1$. Prove GCD(a, b) = 1.**

Suppose:
1. $GCD(a, b) = d$
2. $au+bv=1$
3. $d>1$

$=> d|a$ and $d|b$ [by (1) and definition of GCD]  
$=> \exist k_{a} / a = k_{a}*d$ and $ \exist k_{a} / a = k_{a}*d$ [by definition of divisor]  
$=> k_{a}.d.u + k_{b}.d.v = 1$ [by substitution into (2)]  
$=> d.(ka.u + kb.v) = 1$ [by distribution]  
$=> d = 1$ and $ka.u + kb.v=1$ [because $d>0$ and $d\in\Z$]  
$=> Contradicts (3)!$   
$=> GCD(a, b) = 1$  $\space \square$

* [x] **Suppose $g^a \equiv 1$ (mod m), and $g^b \equiv 1$ (mod m). Prove that $g^{gcd(a,b)} \equiv 1$ (mod m)**

The Extended Euclidian Algorithm gives:

$gcd(a, b) = au + bv$

Hence:

$ g^{gcd(a,b)} = g^{au + bv} = g^{au} * g^{bv} = (g^a)^u * (g^b)^v$

Now, reducing modulo m:

$ (g^a)^u * (g^b)^v \equiv (1)^u*(1)^v \equiv 1 \space \square$

(props to `dreadloaf` on Discord for this proof!)

----

For an alternative proof that I do not fully grasp, see the [wiki solution](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-03+Session+10+Solutions). Discussed below.

This isn't stated explicitly here, but assuming `g` is a generator of the group $\Z/m\Z$? Maybe?

By Euler's Theorem: $g^{\phi(n)} \equiv 1$ (mod n) **if g and m are co-prime** (again: fair assumption? Maybe?)

Then $\exist k_{a} / a = k_{a}*\phi(m)$ and $ \exist k_{b} / b = k_{b}*\phi(n)$ [TODO: prove why?]

Hence, $\phi(m)$ divides both a and b; it's a common divisor. Hence, $\phi(m)|GCD(a,b)$ (by definition of GCD? TODO: prove why?)

Hence $\exist k \in \N / k*\phi(m) = GCD(a,b)$

Hence $g^{gcd(a,b)} \equiv g^{k*\phi(m)} \equiv (g^{\phi(m)})^k \equiv 1^k = 1 $

TODO: prove that $g^{\phi(m)} \equiv 1$? Where does this come from? (I think this because g is a generator, and $\phi(m)$ is the order of the group)

* [x] **Using a program, obtain a generator for the group of integers in $\Z/1009\Z$ and $\Z/2357\Z$. Both values are prime. What method did you use to check if the candidate was a generator?**

See [generators](./generators/). Answers:
```
Generator for Z/1009Z: 11
Generator for Z/2357Z: 2
```

This program is sub-optimal: it actually checked the full set of multiplications. Starting with a candidate g, and repeatedly multiplying (n times). If the result is 1 before n-1 times, this isn't a generator. If the result isn't 1 after n time, this isn't a generator either. The result has to be 1 after n-1 times, and not before.

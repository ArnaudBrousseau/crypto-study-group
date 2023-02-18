# 2023-02-10
The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2023-02-10+Session+11+Notes).

## Assigned reading

* [x] Sections 2.1 through 2.7 of [An Introduction to Mathematical Cryptography](https://www.amazon.com/Introduction-Mathematical-Cryptography-Undergraduate-Mathematics/dp/1441926747)

## Homework related to Chapter 2

* [x] **Compute with pen and paper, the ElGamal algorithm for Alice sending Bob a message. Use parameters: p=19, g=2, a=3, b=5, m=12. You may choose any k. If an adversary obtained the computed ciphertext (c<sub>1</sub>, c<sub>2</sub>), and obtained access to k, how could they decrypt the ciphertext?**

In order for Alice to send a message to Bob, she has to know his public key $B = g^b$ (mod p)

$B = 2^5 = 13$ (mod 19)

Now, Alice needs to choose a $k$ value to compute $(c_1, c_2)$. Let's pick $k=18$.

$c_1 = g^k = 2^{18} = 1$ (mod 19)

$c_2 = m*B^k = 12*13^{18} = 12*1 = 12$ (mod 19)

When bob receives $(c_1, c_2)$, he can decrypt the message with:

$x = c_1^b$, and $m = x^{-1}.c_2$

In this case, $x = 1^5 = 1$, $x^{-1}=1$, and $m= 1*12 = 12$

----

Now, the second part of the question: given 
$(c_1, c_2, k)$, how can one recover m?

An attacker can simply compute $x=(B^{-1})^k$, and then compute:
$c_2*x = m*B^k*x = m*B^k*(B^{-1})^k = m * (B*B^{-1})k = m * 1^k = m$

In the above example, $B^{-1} = 13^{-1} = 3$ (mod 19). So $x = 3^18 = 1$ (mod 19). And $c_2*x = 12*1 = 12$ (mod 19).

* [x] **Compute with Shank's algorithm, on pen and paper, the discrete log of 2<sup>x</sup> = 33 mod 83. Note that is 2 is a generator, which you may check by verifying 2<sup>41</sup>=82 mod 83 (check that you understand why this works)**

"2 is a generator, which you may check by verifying $2^{41}=82$ mod 83".

To prove that 2 is a generator, let's prove that it has order 82 in the group $\Z/83\Z$.

We have $2^{82}=2^{41}*2^{41} = 82*82 = 1$ mod 83

Now, we need to prove that there is no smaller positive power $k<82$ such that $2^k=1$(mod 83).

This is given by Lagrange's Theorem: if there was a smaller positive power $k<82$ such that $2^k=1$(mod 83), then we would have $k|82$: k must be a divisor of 82.

But 82 only has 2 divisors: 2 (prime) and 41 (prime). We know that $2^{41} \ne 1$ and $2^2 \ne 1$ (mod 83), hence there is no smaller positive power $k<82$ such that $2^k=1$(mod 83). Hence 2 has order 82 in $\Z/83\Z$. $\square$

----

Now onto Shank's algorithm! We make two lists to solve $g^x = h$ (mod p):
1. $e, g, g^2,...g^n$
2. $h, h*g^{-n}, h*g^{-2n}, ..., h*g^{-n^2}$

These lists go until $n = 1+round(\sqrt{p})$

We're looking to solve $2^x = 33$ (mod 83):
* $n=1+round(sqrt{83}) = 10$
* $h=33$
* $g=2$
* To find $g^{-1}$ we need to solve $2u + 83v = 1$. $(u=-41, v=1)$ are solutions, so $g^{-1} = 42$
* $g^{-n}=(g^{-1})^n=(42)^{10}=3$

| n | List 1    | List 2                                 |
|---|-----------|----------------------------------------|
|1  | $e=1$     | $h=33$                                 |
|2  | $2^1=2$   | $h*g^{-n}=33*3=16$                     |
|3  | $2^2=4$   | $h*g^{-2n}=h*(g^{-n})^2=33*(3)^2=48$   |
|4  | $2^3=8$   | $h*g^{-3n}=h*(g^{-n})^3=33*(3)^3=61$   |
|5  | $2^4=16$  | $h*g^{-4n}=h*(g^{-n})^4=33*(3)^4=17$   |
|6  | $2^5=32$  | $h*g^{-5n}=h*(g^{-n})^5=33*(3)^5=51$   |
|7  | $2^6=64$  | $h*g^{-6n}=h*(g^{-n})^6=33*(3)^6=70$   |
|8  | $2^7=45$  | $h*g^{-7n}=h*(g^{-n})^7=33*(3)^7=44$   |
|9  | $2^8=7$   | $h*g^{-8n}=h*(g^{-n})^8=33*(3)^8=49$   |
|10 | $2^9=14$  | $h*g^{-9n}=h*(g^{-n})^9=33*(3)^9=64$   |

Note that List 2 for n=2 has a collision with List 1 for n=5, which means $2^4 = 33*2^{-10}$, and thus $2^{4+10} = 33$ (mod 83)

Solution: $x = 14$ $\square$

* [ ] **Optional: Implement one or both of ElGamal encryption and/or shank's algorithm.**
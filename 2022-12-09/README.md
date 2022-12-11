# 2022-12-09

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-09+Session+4+Notes).

## Homework related to Chapter 7

* [ ] **Justify or disqualify each of the following schemes, with message `m`, tag `t`, and ciphertext `c`.**
  * `t = MAC(m)`, `c = E(m)`, send `(c, t)`
  * `t = MAC(m)`, `c = E(m||t)`, send `c`
  * `c = E(m)`, `t = MAC(c)`, send `(c, t)`

* [ ] **You're the adversary, watching a TLS handshake. Pick three steps from [TLS Handshake - OSDev Wiki](https://wiki.osdev.org/TLS_Handshake#Handshake_Overview), and describe how the step prevents you from (pick one):**
  * reading message content (confidentiality)
  * tampering with message content (integrity)
  * impersonating either party (authenticity)
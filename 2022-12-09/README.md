# 2022-12-09

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-09+Session+4+Notes).

## Homework related to Chapter 7

* [x] **Justify or disqualify each of the following schemes, with message `m`, tag `t`, and ciphertext `c`.**
  * `t = MAC(m)`, `c = E(m)`, send `(c, t)`
  * `t = MAC(m)`, `c = E(m||t)`, send `c`
  * `c = E(m)`, `t = MAC(c)`, send `(c, t)`

  The first scheme seems secure. This is a typical encrypt-and-MAC scheme. An attacker cannot read the message because it's encrypted, cannot tamper with the message content because the MAC ensures integrity.

  The second scheme might have an integrity problem: an attacker could modify `c` in transit, and decryption must take place before the MAC construction can be checked. But that's a typical trade-off in MAC-then-encrypt schemes. This can be made secure.

  The third scheme feels secure as well: this is a typical encrypt-then-MAC scheme. If the underlying MAC function is strong, this scheme is secure.

* [x] **You're the adversary, watching a TLS handshake. Pick three steps from [TLS Handshake - OSDev Wiki](https://wiki.osdev.org/TLS_Handshake#Handshake_Overview), and describe how the step prevents you from reading message content (confidentiality), tampering with message content (integrity), and impersonating either party (authenticity).

**Reading message content (confidentiality)**: prevented by the Key Exchange protocol, which yields a session key under which all messages are encrypted. The session key is derived from a client random, a server random, and a pre-master secret.

**Tampering with message content (integrity)**: prevented with MACs in the encrypted payloads, when using authenticated encryptions as the cipher suite. For example, `TLS_CHACHA20_POLY1305_SHA256` or `TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 ` use Poly1305 and GCM to provide integrity. It's funny that these schemes are generally called "authenticated encryption": really they provide integrity!

**Impersonating either party (authenticity)**: this is prevented by the Certificate Message, when the server sends its certificates. The client can then verify that it's actually talking to the right server as opposed to a malicious one.

## Extra Reading

Heavier than usual given that the book (Chapter 7) is severely out-of-date.

* [Authenticated encryption - Wikipedia](https://en.wikipedia.org/wiki/Authenticated_encryption)
* skim the following Wikipedia articles on the two most commonly used AEADs:
  * [Galois/Counter Mode - Wikipedia](https://en.wikipedia.org/wiki/Galois/Counter_Mode#cite_note-1) - GCM is older than the next entry, generally regarded secure, but less secure and slower than ChaCha-Poly. Some machines have special instruction sets for computing AES which makes GCM faster. AES-GCM can only be securely implemented at the hardware-level, if timing side channel attacks are to be avoided. There are also concerns about the 128-bit AES block size, reducing security against collision resistance.
  * [ChaCha20-Poly1305 - Wikipedia](https://en.wikipedia.org/wiki/ChaCha20-Poly1305#XChaCha20-Poly1305_%E2%80%93_extended_nonce_variant) - is a faster, more secure AEAD, standardized in 2015. Both algorithms are standardized for use in TLS/SSL, and widely used. The algorithm takes a 256-bit-key, and software implementations are less vulnerable to timing attacks.
  * [It takes two to ChaCha (Poly)](https://blog.cloudflare.com/it-takes-two-to-chacha-poly/) - Cloudflare ChaCha20-Poly and AE explainer.
* TLS - familiarize yourself with TLS handshakes, and the Rustls library. You are welcome to continue to use OpenSSL, though all future examples will be given with Rustls and exclusively Rust-based implementations, where possible.
  * [What happens in a TLS handshake? | SSL handshake | Cloudflare](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/)
  * [How does SSL work? | SSL certificates and TLS | Cloudflare](https://www.cloudflare.com/learning/ssl/how-does-ssl-work/)
  * [Transport Layer Security - Wikipedia](https://en.wikipedia.org/wiki/Transport_Layer_Security#TLS_1.0)
* Recommended libraries for this week:
  * [ChaCha20Poly1305 — Rust crypto library // Lib.rs](https://lib.rs/crates/chacha20poly1305)
  * [AES-GCM — Rust crypto library // Lib.rs](https://lib.rs/crates/aes-gcm)
  * [AEAD — Rust crypto library // Lib.rs](https://lib.rs/crates/aead)
* 2007 paper introducing authenticated encryption as a primitive: [PDF link](https://eprint.iacr.org/2000/025.pdf)
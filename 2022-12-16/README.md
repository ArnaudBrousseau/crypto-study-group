# 2022-12-16

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-30+Session+5+Notes).

## Homework related to Chapter 8

* [x] **Write an Error type with [anyhow](https://docs.rs/anyhow/latest/anyhow/) and [thiserror](https://docs.rs/thiserror/1.0.37/thiserror/).**

Pretty simple implementation, but I don't feel like arbitrarily getting fancy just to use more `anyhow` or `thiserror` features.

* [ ] **Implement a type-level program using `PhantomData` to parameterize the state of a struct.**

I haven't finished yet but I've started at [ed25519_signer](./ed25519_signer/). Idea being to encode the state machine of message signing into types: first we have to construct a transaction, then sign it, then broadcast it. Broadcast can't be called on an unsigned transaction for example.


* [x] **Put CI on a project. Deny lints, failing tests, and failing formatting.**

CI is in place for this project now. `cargo fmt`, `clippy`, and tests.

* [ ] **Compile your codebase with a malicious feature. Test against it in an integration test.**

* [ ] **Write an implementation failing one**

* [ ] **Implement a program that loops 1000 times, repeatedly branching on secret data (say, equality to number 123456789012345678), taking the left path in execution A and the right path in execution B. Benchmark your program. Determine if your benchmarks are statistically different. Try this first with a single u64, then repeat the experiment making your secret data a vector of length 100 u64's.**

* [x] **Perform the test demonstrated in [the zeroize blog post](https://benma.github.io/2020/10/16/rust-zeroize-move.html). For bonus credit, use a debugger ([`lldb`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for instance), to insert break points into your code. See if you can determine where the secret value remains within memory, either with lldb, or even with dd. See [this documentation page](https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html) for the state of debugger support in Rust.**

See [zeroize_bug](./zeroize_bug/). Running the program gives:
```
Secret locus: 0x16b00a91c
Keypair erased...or is it?
Reading secret locus: "PRIV"
```
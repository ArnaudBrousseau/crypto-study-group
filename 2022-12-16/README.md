# 2022-12-16

The notes & assigned homework are [here](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-12-30+Session+5+Notes).

## Homework related to Chapter 8

* [ ] **Write an Error type with [anyhow](https://docs.rs/anyhow/latest/anyhow/) and [thiserror](https://docs.rs/thiserror/1.0.37/thiserror/).**

* [ ] **Implement a type-level program using `PhantomData` to parameterize the state of a struct.**

* [x] **Put CI on a project. Deny lints, failing tests, and failing formatting.**

CI is in place for this project now. `cargo fmt`, `clippy`, and tests.

* [ ] **Compile your codebase with a malicious feature. Test against it in an integration test.**

* [ ] **Write an implementation failing one**

* [ ] **Implement a program that loops 1000 times, repeatedly branching on secret data (say, equality to number 123456789012345678), taking the left path in execution A and the right path in execution B. Benchmark your program. Determine if your benchmarks are statistically different. Try this first with a single u64, then repeat the experiment making your secret data a vector of length 100 u64's.**

* [ ] **Perform the test demonstrated in the zeroize blog post. For bonus credit, use a debugger (`lldb` for instance), to insert break points into your code. See if you can determine where the secret value remains within memory, either with lldb, or even with dd. See this documentation page for the state of debugger support in Rust.**
# `env`

![rust 2024](https://img.shields.io/badge/rust-2024-blue?style=for-the-badge&logo=rust&logoColor=white)

Provides a safe interface for <code>[std::env](http://doc.rust-lang.org/std/env)::{[set_var](https://doc.rust-lang.org/std/env/fn.set_var.html), [remove_var](https://doc.rust-lang.org/std/env/fn.remove_var.html)}</code>.

## Rationale

Since [#124636](https://github.com/rust-lang/rust/pull/124636), `std::env::set_var` and `std::env::remove_var}` have become unsafe, due to their being unsafe when in a multi-threaded unix context[^1].

This crate wraps these functions, checking if these conditions are met at runtime, such that these functions are safe to call.

[^1]: https://github.com/rust-lang/rust/issues/27970

# imperative

> **Check the mood of a word**

[![codecov](https://codecov.io/gh/crate-ci/imperative/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/imperative)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/imperative.svg)
[![Crates Status](https://img.shields.io/crates/v/imperative.svg)][Crates.io]


## [Contribute](CONTRIBUTING.md)

## Special Thanks

Thank you to [pydocstyle](https://github.com/PyCQA/pydocstyle/) for the algorithm and data set.

### Regenerating the wordlist

If you change `assets/imperatives.txt` or `assets/imperatives_blacklist.txt`, run

```bash
env SNAPSHOTS=overwrite cargo test
```
to regenerate the `wordlist_codegen.rs` file while running tests.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.

[Crates.io]: https://crates.io/crates/imperative
[Documentation]: https://docs.rs/imperative

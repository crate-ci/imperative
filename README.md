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

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[Crates.io]: https://crates.io/crates/imperative
[Documentation]: https://docs.rs/imperative

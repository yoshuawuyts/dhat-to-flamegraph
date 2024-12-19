<h1 align="center">dhat-to-flamegraph</h1>
<div align="center">
  <strong>
    Convert dhat JSON output to a collapsed flamegraph format
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dhat-to-flamegraph">
    <img src="https://img.shields.io/crates/v/dhat-to-flamegraph.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dhat-to-flamegraph">
    <img src="https://img.shields.io/crates/d/dhat-to-flamegraph.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/dhat-to-flamegraph">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/dhat-to-flamegraph">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/dhat-to-flamegraph/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/dhat-to-flamegraph/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Installation
```sh
$ cargo add dhat-to-flamegraph
```

## Usage

```text
Usage: dhat-to-flamegraph <INPUT> [OUTPUT]

Arguments:
  <INPUT>   The dhat JSON file
  [OUTPUT]  Where to write the output file [default: dhat.folded]

Options:
  -h, --help  Print help
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/dhat-to-flamegraph/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/dhat-to-flamegraph/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/dhat-to-flamegraph/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

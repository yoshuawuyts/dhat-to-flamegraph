<h1 align="center">dhat-to-flamegraph</h1>
<div align="center">
  <strong>
    Convert dhat JSON output to a flamegraph
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

## About

[DHAT] is a dynamic heap analysis tool.
It is part of the [Valgrind](https://valgrind.org/) instrumentation framework.
[dhat-rs] provides a similar implementation for direct integration with Rust.
Both tools produce JSON output, describing the tracked heap allocations.
DHAT provides a web-based tool to view the generated data.

`dhat-to-flamegraph` converts this JSON data into a flamegraph.

[dhat]: https://www.valgrind.org/docs/manual/dh-manual.html

## Installation
```sh
$ cargo install dhat-to-flamegraph
```

## Usage

```text
Convert dhat JSON output to a flamegraph

Usage: dhat-to-flamegraph [OPTIONS] <INPUT>

Arguments:
  <INPUT>
          The dhat JSON file to process

Options:
  -o, --output <OUTPUT>
          Where to place the output
          
          If not provided then stdout is used.

  -f, --format <FORMAT>
          Which output format to use

          Possible values:
          - svg:    Format as svg (default)
          - folded: Format as folded stack traces

  -h, --help
          Print help (see a summary with '-h')
```

Usage example:

```bash
dhat-to-flamegraph fixtures/dhat-heap.json > out.svg
open out.svg
```

## See Also

- [nnethercote/dhat-rs][dhat-rs]

[dhat-rs]: https://github.com/nnethercote/dhat-rs

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

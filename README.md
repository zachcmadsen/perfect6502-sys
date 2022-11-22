# perfect6502-sys

[![CI][ci_badge]][ci]
[![crates.io][crate_badge]][crate]
[![docs.rs][docs_badge]][docs]
[![license][license_badge]][license]

`perfect6502-sys` provides bindings to [`perfect6502`][perfect6502].

## Bindings

The bindings were manually generated with

`bindgen --use-core -o src/bindings.rs perfect6502/perfect6502.h`

## Contributing

Feel free to open an issue if you have ideas for improvements.

## Resources

The following were helpful for learning about how to create a sys crate:

- https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65
- https://kornel.ski/rust-sys-crate

<!-- Badges -->

[ci_badge]: https://github.com/zachcmadsen/perfect6502-sys/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/zachcmadsen/perfect6502-sys/actions?query=branch%3Amain
[crate_badge]: https://img.shields.io/crates/v/perfect6502-sys.svg
[crate]: https://crates.io/crates/perfect6502-sys
[docs_badge]: https://img.shields.io/docsrs/perfect6502-sys/latest.svg
[docs]: https://docs.rs/perfect6502-sys
[license_badge]: https://img.shields.io/crates/l/perfect6502-sys.svg
[license]: https://github.com/zachcmadsen/perfect6502-sys/blob/main/LICENSE

<!-- Links -->

[perfect6502]: https://github.com/mist64/perfect6502

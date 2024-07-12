# perfect6502-sys

[![CI][ci_badge]][ci]
[![crates.io][crate_badge]][crate]
[![docs.rs][docs_badge]][docs]
[![license][license_badge]][license]

Bindings to [`perfect6502`][perfect6502].

## Limitations

The only actively tested targets are `x86_64-unknown-linux-gnu` and
`x86_64-pc-windows-msvc`. The bindings are generated manually
(see [`generate_bindings.sh`][generate_bindings]) so they might be incorrect for
other targets.

## Contributing

Feel free to open an issue if you have ideas for improvements.

## License

[MIT](https://github.com/zachcmadsen/perfect6502-sys/blob/main/LICENSE)

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
[generate_bindings]: https://github.com/zachcmadsen/perfect6502-sys/blob/main/generate_bindings.sh
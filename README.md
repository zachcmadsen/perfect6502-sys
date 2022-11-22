# perfect6502-sys

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

[perfect6502]: https://github.com/mist64/perfect6502

# Slicing for custom types

Rust's built-in slices are great, however they impose a number of restrictions on how the data is laid out in memory. Namely, the items must be contiguous (right next to each other). This means you can't use the built in slices for structures such as `VecDeque` and many more. This crate is basically a polyfill/alternative implementation of Rust's built-in slices, however they can be used for any type that implements `std::ops::Index` or `std::ops::IndexMut`. This probably means they aren't quite as performant, although liberal sprinkling of parametric polymorphism throughout this codebase means that hopefully `rustc` will optimise most of it away to a specialised, speedy implementation.

[**API Documentation**](https://docs.rs/owned_slice)

## Contributing

- This crate implements the bare-minimum functionality at the moment. If you want more advanced features that mimic Rust's built-in slices, then please open an issue :D. Off the top of my head, one such feature would be taking a slice of a slice.
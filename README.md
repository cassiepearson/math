# Computational Math Repository

Each module has its own README with details on what it does.

## Methods

The math modules, such as the number_theory module, define a set of structures and/or traits using macro invocations. This is done to allow different supertraits to be defined and the traits to be bound by them. This will allow the supertraits to be changed but the same implementations used. For instance, switching between different integer and floating point supertrait bounds or changing the supertrait to only capture unsigned integers.

### Testing

To see the results of any functions, run the unit tests. You can use the following commands to do so:

```
cargo test                            // Run all tests in debug mode
cargo test --release                  // Run all tests in release (optimized) mode
```

### Documentation

Rust has an auto-documentation system built in. To see the documentation run:

```
cargo doc --document-private-items --open
```

<!-- ## Citations
[AKS Primality Test](https://www.cse.iitk.ac.in/users/manindra/algebra/primality_v6.pdf) -->

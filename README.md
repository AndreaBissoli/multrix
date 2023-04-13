# Multrix

Multrix is a simple matrix multiplication and manipulation library for rust.

It is a work in progress, with many features still to be added.

The objective is for it to be fast, easy to use, and easy to extend, with multithreading capabilities.

## Examples

### Matrix multiplication

```rust
use multrix::multrix::Matrix;
fn main() {
    let a = Matrix::new_rand(100, 100);
    let b = Matrix::new_rand(100, 100);
    a.parallel_product(&b).print();
}
```

## Documentation

All the functionalities are documented, and the documentation can be found [here](https://docs.rs/multrix/0.1.0/multrix/).
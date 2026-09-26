A logging backend for defmt that does nothing but does provide the linker symbols required by
defmt, avoiding the linker error
```
mold: error: undefined symbol: _defmt_release
```
Which is useful if you're building your `no_std` application out of libraries which you want to test
using regular std unit tests, without having to feature gate defmt, so instead of

`Cargo.toml`:
```toml
[features]
defmt = ["dep:defmt"]
[dependencies]
defmt = { version = "..", optional = true }
```
`lib.rs`:
```rust
#[cfg(feature = "defmt")]
use defmt::debug
// <code>
#[cfg(feature = "defmt")]
debug!("not going to link defmt during test")
// <code>
```
You can do just this:

`Cargo.toml`:
```toml
[build-dependencies]
defmt-nop = "0.1.0"
```
This approach also has the benefit that your defmt invocations are type checked during testing and not only when you build your final application with the `defmt` feature enabled
                                                                                              
# Usage
```rust
#[cfg(test)]
use defmt_nop as _;
```

once somewhere in your lib

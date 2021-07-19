# rustpostal

Rust bindings to [libpostal][], a fast statistical parser/normalizer
for street addresses anywhere in the world.

## Usage


```rust
use rustpostal::address;
use rustpostal::expand;
use rustpostal::LibModules;

fn main() -> Result<(), rustpostal::error::RuntimeError> {
    let postal_module = LibModules::All;
    postal_module.setup()?;

    let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";

    let labeled_tokens = address::parse_address(address, None, None)?;

    for (token, label) in &labeled_tokens {
        println!("{}: {}", token, label);
    }

    let expanded = expand::expand_address_with_options(address, Some(["en"].iter()))?;

    for expansion in &expanded {
        println!("{}", expansion);
    }
    Ok(())
}
```

## Setup

1. Install the C library: See [installation instructions][linux-install].

2. Export the installation path to `LD_LIBRARY_PATH`.

## Tests

```
$ cargo test -- --test-threads 1
```


[libpostal]: https://github.com/openvenues/libpostal
[linux-install]: https://github.com/openvenues/libpostal#installation-maclinux

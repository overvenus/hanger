# hanger

Call hooks before/after your functions.

Add this to your Cargo.toml:

```toml
[dependencies]
hanger = "0.1"
```

Add hooks to any functions.

```
fn hello() { print!("hello") }

#[hanger::hook(hello)]
fn world() { print!("world") }

world(); // It prints "helloworld".
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in hanger by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

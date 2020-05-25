# Configstore [![crates.io](https://img.shields.io/crates/v/configstore)](https://crates.io/crates/configstore) [![status](https://github.com/tarikeshaq/configstore/workflows/Rust/badge.svg)](https://github.com/tarikeshaq/configstore/actions)

Configstore is a library that allows you to store your configurations locally without having to worry about the directories or the platform

* [API Documentation](https://docs.rs/configstore/)
* Cargo package: [configstore](https://crates.io/crates/configstore)

## Usage

To use `configstore`, add this to your `Cargo.toml`:

```toml
[dependencies]
configstore = "0.1"
```

### Initialize your Configstore

```rust,ignore
use configstore::{Configstore, AppUI};
fn main() {
    let config_store = Configstore::new("myApp", AppUI::CommandLine).unwrap();
}
```


### Set and get values

Configstore supports any value that implements [Deserialize](https://docs.serde.rs/serde/trait.Deserialize.html) and [Serialize](https://docs.serde.rs/serde/trait.Serialize.html)


```rust
use serde_derive::*;
use configstore::{Configstore, AppUI};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
struct Value {
    text: String,
    num: u32
}

let config_store = Configstore::new("myApp", AppUI::CommandLine).unwrap();

let value = Value {text: "hello world".to_string(), num: 4343};
config_store.set("key", value.clone()).unwrap();

let same_value: Value = config_store.get("key").unwrap();
assert_eq!(value, same_value);
```

Configstore will store the configuration files under your platforms native config directory based on [platform-dirs](https://crates.io/crates/platform-dirs)


## Contributing

All contributions are welcome, feel free to file an issue or even a pull-request 🤝

## License

This project is licensed under the [Mozilla Public License 2.0](https://github.com/tarikeshaq/configstore/blob/master/LICENSE)

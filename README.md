<p align="center">
    <img src=".github/assets/header.png" alt="Rust Uzbekistan's {Maydon}">
</p>

<p align="center">
    <h3 align="center">Field enum generation for struct.</h3>
</p>

<p align="center">
    <a href="https://t.me/rustlanguz"><img align="center" src="https://img.shields.io/badge/Chat-grey?style=flat&logo=telegram&logoColor=ffffff&labelColor=dea584&color=dea584" alt="Telegram Chat"></a>
    <a href="https://github.com/rust-lang-uz/maydon/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/rust-lang-uz/maydon/test.yml?style=flat&logo=github&logoColor=ffffff&labelColor=dea584&color=dea584" alt="Test CI"></a>
</p>

## Motivation

Upon writing some rusty code for CLI config management, I simply declared a struct like this:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub port: u16,
    pub database_url: String,
    pub threads: u16,
}
```

Also, in order to have set function with proper typed selection, I declared another enum named `Field`:

```rust
pub enum Field {
    Url,
    Port,
    Database,
    Unknown,
    Threads,
}
```

The thing is, I wanted to automate creation of enum generation. Ordinary macros wouldn't be a good viable solution, so I wrote maydon from ground up. I might be missing something or prequisites, but this is what I came up with:

```rust
use maydon::Maydon;

#[derive(Maydon)]
#[field_name = "Selector"]
pub struct Config {
    pub url: String,
    pub port: u16,
}

// The macro should generate:
fn main() {
    let _ = Selector::Url;
    let _ = Selector::Port;
    let _ = Selector::Unknown;
}
```

## License

This project is licensed under the MIT or Apache-2.0 license - see the [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Rust Uzbekistan's {Maydon}">
</p>

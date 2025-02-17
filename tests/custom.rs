use maydon::Maydon;

#[derive(Maydon)]
#[field_name = "ConfigField"]
pub struct Config {
    pub url: String,
    pub port: u16,
}

// The macro should generate:
fn main() {
    let _ = ConfigField::Url;
    let _ = ConfigField::Port;
    let _ = ConfigField::Unknown;
}

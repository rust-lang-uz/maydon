use maydon::Maydon;

#[derive(Maydon)]
pub struct Config {
    pub url: String,
    pub port: u16,
    pub database_url: String,
    pub threads: u16,
}

// The macro should generate:
fn main() {
    let _ = Field::Url;
    let _ = Field::Port;
    let _ = Field::DatabaseUrl;
    let _ = Field::Threads;
    let _ = Field::Unknown;
}

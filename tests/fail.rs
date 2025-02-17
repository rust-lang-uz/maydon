use maydon::Maydon;

#[derive(Maydon)]
enum InvalidEnum {
    // ❌ Macros should only be used on structs
    A,
    B,
    C,
}

fn main() {}

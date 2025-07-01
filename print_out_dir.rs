fn main() { println!("OUT_DIR={}", std::env::var("OUT_DIR").unwrap_or_else(|_| "not set".to_string())); }

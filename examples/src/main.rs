use std::env;
use multi_env::set_ok;

fn main() {
    // "env"
    set_ok("env");

    // "yaml" | "yml"
    set_ok("yaml");
    set_ok("yml");

    // USE ENV: "RUST.ENV"
    env::var("RUST.ENV").unwrap_or_else(|_| String::from(""));
}
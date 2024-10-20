use multi_env::set_ok;

fn main() {
    // "env"
    set_ok("env");

    // "yaml" | "yml"
    set_ok("yaml");
    set_ok("yml");
}
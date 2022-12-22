use std::env;

fn main() {
    if let Ok(homebrew_prefix) = env::var("HOMEBREW_PREFIX") {
        println!("cargo:rustc-link-search={}/lib", homebrew_prefix);
    }
}

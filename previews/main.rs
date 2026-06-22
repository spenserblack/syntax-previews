//! This is a doc comment.
use std::env;

macro_rules! env_or {
    ($main:literal, $fallback:literal) => {
        env::var_os($main).or(env::var_os($fallback))
    };
}

/// This is also a doc comment.
///
/// Rust doc comments can contain embedded code, like this:
///
/// ```rust
/// assert_eq!(3, 1 + 3);
/// ```
fn main() {
    // Normal comment
    let home: Option<_> = env_or!("HOME", "USERPROFILE");
    let _is_example: bool = true;
    let _three = 1 + 3;
    match home {
        Some(p) => println!("home is {p:?}"),
        None => println!("home not set"),
    }
}

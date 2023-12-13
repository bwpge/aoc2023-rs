//! Shared library for Advent of Code 2023.

pub mod cli;

pub fn say_hello(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        assert!(true);
    }
}

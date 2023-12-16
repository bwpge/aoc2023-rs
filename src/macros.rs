#![doc(hidden)]

/// Prints an error message to `stderr`.
#[macro_export]
macro_rules! error {
    ($($tt:tt)*) => {{
        eprint!("error: ");
        eprintln!($($tt)*);
    }};
}

/// Prints a warning message to `stderr`.
#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {{
        eprint!("warning: ");
        eprintln!($($tt)*);
    }};
}

/// Creates a [`HashSet`][std::collections::HashSet] from a list of items.
///
/// Taken verbatim from [`map_macro`][map_macro] crate.
///
/// [map_macro]: https://crates.io/crates/map-macro
#[macro_export]
macro_rules! set {
    {$($v: expr),* $(,)?} => {
        std::collections::HashSet::from([$($v,)*])
    };
}

/// Creates a [`HashMap`][std::collections::HashMap] from a list of key-value
/// pairs.
///
/// Taken verbatim from [`map_macro`][map_macro] crate.
///
/// [map_macro]: https://crates.io/crates/map-macro
#[macro_export]
macro_rules! map {
    {$($k: expr => $v: expr),* $(,)?} => {
        std::collections::HashMap::from([$(($k, $v),)*])
    };
}

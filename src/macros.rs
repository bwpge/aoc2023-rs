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

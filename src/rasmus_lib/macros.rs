#[macro_export]
macro_rules! print_crlf {
    () => {
        $crate::print!("\r\n")
    };
    ($($arg:tt)*) => {{
        print!($($arg)*);
        // This is probably really bad ._.
        print!("\r\n")

    }};
}

#[macro_export]
macro_rules! eprint_crlf {
    () => {
        $crate::eprint!("\r\n")
    };
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        // This is probably really bad ._.
        eprint!("\r\n")
    }};
}

#[macro_export]
macro_rules! dbg_crlf {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::eprint_crlf!("[{}:{}]", $crate::file!(), $crate::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprint_crlf!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_crlf!($val)),+,)
    };
}

#[macro_export]
macro_rules! dbgts {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {

                let prefix = "    ";
                let value = format!("{}", &tmp)
                    .lines()
                    .map(|line| format!("{}{}", prefix, line))
                    .collect::<Vec<_>>()
                    .join("\n");

                eprintln!("[{}:{}:{}] {} =\n{}",
                    file!(), line!(), column!(), stringify!($val), &value);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

#[macro_export]
macro_rules! get_env {
    ($var:expr) => {
        std::env::var($var).expect(&format!("Environment variable '{}' not found", $var))
    };

    ($var:expr, $default:expr) => {
        std::env::var($var).unwrap_or_else(|_| $default.to_string())
    };

    ($var:expr, $default:expr, $type:ty) => {
        std::env::var($var)
            .ok()
            .and_then(|v| v.parse::<$type>().ok())
            .unwrap_or($default)
    };
}

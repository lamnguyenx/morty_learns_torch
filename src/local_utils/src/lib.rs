#[macro_export]
macro_rules! dbgts {
    ($val:expr) => {{
        // Get the file and line information
        let file = std::file!();
        let line = std::line!();

        // Create a reference to the value

        let prefix = "    ";
        let value = format!("{}", &$val)
            .lines()
            .map(|line| format!("{}{}", prefix, line))
            .collect::<Vec<_>>()
            .join("\n");

        // Print the variable name, value, file, and line number
        println!("\n{}:{}\n{} =\n {}", file, line, stringify!(&$val), value);
    }};
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

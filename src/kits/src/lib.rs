use std::fmt::Display;

#[cfg(feature = "candle")]
pub mod candle_kit;

#[cfg(feature = "tch")]
pub mod tch_kit;

pub fn format_tensor<T: Display>(tensor: T, indent: &str) -> String {
    return tensor
        .to_string()
        .lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n");
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

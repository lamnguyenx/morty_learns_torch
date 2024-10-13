#![cfg(feature = "candle")]

use candle_core::Device;

#[derive(Debug, thiserror::Error)]
pub enum GetDeviceError {
    #[error("Unsupported device: {0}")]
    InvalidDevice(String),
    #[error("Candle core error: {0}")]
    CandleCoreError(#[from] candle_core::Error),
}

pub fn get_device_from_current_env() -> Result<Device, GetDeviceError> {
    let device_str = crate::get_env!("DEVICE", "cpu:0").to_lowercase();
    let parts: Vec<&str> = device_str.split(':').collect();
    let device = *parts.get(0).unwrap_or(&"cpu");
    let index = match parts.get(1) {
        Some(index_str) => index_str.parse::<usize>().unwrap_or(0),
        None => 0,
    };
    return match device {
        "cpu" => Ok(Device::Cpu),
        "mps" => Device::new_metal(index).map_err(GetDeviceError::CandleCoreError),
        "metal" => Device::new_metal(index).map_err(GetDeviceError::CandleCoreError),
        "cuda" => Device::new_cuda(index).map_err(GetDeviceError::CandleCoreError),
        _ => Err(GetDeviceError::InvalidDevice(
            device_str + "(from env:DEVICE)",
        )),
    };
}

pub fn stringify_tensor(tensor: &candle_core::Tensor) -> String {
    let indent = "    ";
    let value = crate::format_tensor(&tensor, indent);
    let device = &tensor.device();
    return format!(
        "[{}:{}:{}] {} =\n{} (device: {:?})",
        file!(),
        line!(),
        column!(),
        stringify!($val),
        &value,
        &device
    );
}

#[macro_export]
macro_rules! candle_kit__dbgts_candle {
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {

        match $val {
            tmp => {
                let printed = $crate::candle_kit::stringify_tensor(&tmp);
                eprintln!("{}", printed);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
pub use crate::candle_kit__dbgts_candle as dbgts;

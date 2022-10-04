#[cfg(any(target_os = "macos", target_os = "ios"))]
mod macos;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::macos::Monotonic;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::Monotonic;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use self::wasm::Monotonic;

#[cfg(any(target_vendor = "teaclave", target_env="sgx"))]
mod sgx;
#[cfg(any(target_vendor = "teaclave", target_env="sgx"))]
pub use self::sgx::Monotonic;

#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "windows",
    target_arch = "wasm32",
    target_env = "sgx",
    target_vendor= "teaclave"
)))]
mod unix;
#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "windows",
    target_arch = "wasm32",
    target_env = "sgx",
    target_vendor= "teaclave"
)))]
pub use self::unix::Monotonic;

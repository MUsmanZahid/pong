#[cfg(target_os = "linux")]
pub mod xcb;

#[cfg(target_os = "linux")]
pub use xcb::Window;

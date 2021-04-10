#![allow(dead_code)]

/// Wrapper macro around `concat!` that appends the NULL-terminating character, "\0" to a string
/// literal in order for use in ffi.
#[macro_export]
macro_rules! nt_str {
    ( $s:literal ) => {
        concat!($s, "\0")
    };
}

/// This macro defines a handle - an opaque structure which is passed around as a pointer in
/// ffi code.
macro_rules! define_handle {
    ( $($name: ident),* ) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            pub struct $name {
                _private: [u8; 0],
            }
        )*
    };
}

#[cfg(target_os = "linux")]
pub use linux::Library;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod vk;

#[cfg(target_os = "linux")]
pub mod xcb;


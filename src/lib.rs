//! `Ghost` is a library that can delete exe files on the fly
//! 
//! Windows implementation heavily refrences [@bytebl33der] NIM 
//! implementation in OffensiveNim


#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::unix::ninja as ninja_impl;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use crate::unix::errors::GhostError;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use crate::windows::{ninja as ninja_impl, DEFAULT_PLACEHOLDER};

#[cfg(target_os = "windows")]
pub use crate::windows::errors::GhostError;

pub fn ninja() -> Result<(), GhostError> {
    #[cfg(target_os = "windows")]
    {
        return ninja_impl(DEFAULT_PLACEHOLDER);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        return ninja_impl();
    }
}


#[cfg(target_os = "windows")]
pub fn ninja_with_placeholder<S: Into<String>>(placeholder: S) -> Result<(), GhostError> {
    let mut p: String = placeholder.into();

    if p.starts_with(":") {
        p = format!("{:s<9}", p)
    } else {
        p = format!(":{:s<8}", p);
    }
    let mut placeholder_bytes: [u8; 9] = Default::default();
    placeholder_bytes.copy_from_slice(&p.into_bytes()[..9]);

    return ninja_impl(placeholder_bytes);
}
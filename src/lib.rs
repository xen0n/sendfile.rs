#![crate_name="sendfile"]

#[macro_use]
extern crate syscall;

pub use platform::*;


#[cfg(target_os = "linux")]
#[path="linux.rs"]
pub mod platform;

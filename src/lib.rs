#[cfg(target_os = "linux")]
#[path = "cairo.rs"]
mod backend;

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[path = "null.rs"]
mod backend;

pub use backend::*;

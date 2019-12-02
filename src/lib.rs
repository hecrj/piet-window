#[cfg_attr(
    any(
        target_os = "macos",
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ),
    path = "cairo.rs"
)]
mod backend;

pub use backend::*;

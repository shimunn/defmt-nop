#![doc = include_str!("../README.md")]

use defmt::{global_logger};
#[cfg(feature = "timestamp")]
use defmt::{ timestamp};
/// A logging backend for defmt that does nothing but does provide the linker symbols required by
/// # Usage
/// ```rust
/// use defmt_nop as _;
/// ```
#[global_logger]
struct NopLogger;

#[cfg(feature = "timestamp")]
timestamp!("");

unsafe impl defmt::Logger for NopLogger {
    fn acquire() {
        // ...
    }
    unsafe fn flush() {
        // ...
    }
    unsafe fn release() {
        // ...
    }
    unsafe fn write(_bytes: &[u8]) {
        // ...
    }
}

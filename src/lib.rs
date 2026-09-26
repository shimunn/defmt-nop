#![doc = include_str!("../README.md")]

use defmt::{global_logger, timestamp};
/// A logging backend for defmt that does nothing but does provide the linker symbols required by
/// # Usage
/// ```rust
/// use defmt_nop as _;
/// ```
#[global_logger]
struct NopLogger;

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
    unsafe fn write(bytes: &[u8]) {
        // ...
    }
}

use hyperlight_host::func::HostFunction;

/// Module for sandboxed Python execution.
pub mod sandbox;

/// Type alias for a host function that prints a string from the sandboxed environment.
pub type HostPrintFn = HostFunction<i32, (String,)>;

/// Re-exporting Result type from hyperlight_host for downstream usage.
pub use hyperlight_host::Result;

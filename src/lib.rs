pub mod errors;
pub mod instructions;
pub mod processor;
pub mod state;

#[cfg(not(feature="no-entrypoint"))]
pub mod entrypoint;

pub const COUNTER_SEED: &str = "counter";
pub const SETTINGS_SEED: &str = "settings";
pub const DATA_SEED: &str = "data";

solana_program::declare_id!("9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y");

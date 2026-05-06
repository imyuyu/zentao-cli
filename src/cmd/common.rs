use crate::core::logging::{log, LogLevel};
use crate::safe_println;
use serde::Serialize;

pub fn log_command(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Info, scope, message);
}

pub fn log_debug(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Debug, scope, message);
}

pub fn print_json<T: Serialize>(value: &T) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).unwrap_or_default()
    );
}

pub fn print_error(err: &anyhow::Error) {
    eprintln!("Error: {}", err);
}

pub fn print_deleted(label: &str, id: u64) {
    println!("{} {} deleted successfully", label, id);
}

pub fn print_dry_run(scope: &str, url: &str) {
    safe_println(&format!("[DRY-RUN] Would call {}", scope));
    println!("  URL: {}", url);
}

pub fn print_dry_run_with_body<T: Serialize>(scope: &str, url: &str, body: &T) {
    print_dry_run(scope, url);
    println!(
        "  Body: {}",
        serde_json::to_string_pretty(body).unwrap_or_default()
    );
}

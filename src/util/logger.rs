use ansi_term::Colour;

/// Logs to the console with the cyan color. Newline
pub fn log(data: &str) {
    println!("[Logger] {}", Colour::Cyan.paint(data))
}

/// Logs to the console with the cyan color. No Newline
pub fn log_raw(data: &str) {
    print!("[Logger] {}", Colour::Cyan.paint(data))
}
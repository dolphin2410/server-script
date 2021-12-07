use ansi_term::Colour;

pub fn log(data: &str) {
    println!("[Logger] {}", Colour::Cyan.paint(data))
}

pub fn log_raw(data: &str) {
    print!("[Logger] {}", Colour::Cyan.paint(data))
}
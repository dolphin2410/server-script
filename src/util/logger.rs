use termcolor::{StandardStream, WriteColor, ColorSpec, Color};
use std::io::Write;


/// Logs to the console with the cyan color. Newline
pub fn log(data: &str, fg: Option<Color>, bg: Option<Color>) {
    log_raw(&format!("{}\n", data), fg, bg);
}

/// Logs to the console with the cyan color. No Newline
pub fn log_raw(data: &str, fg: Option<Color>, bg: Option<Color>) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    let mut spec = ColorSpec::new();
    spec.set_fg(fg).set_bg(bg);
    stdout.set_color(&spec).unwrap();
    write!(&mut stdout, "{}", data).unwrap();

    let mut reset_stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    stdout.reset().unwrap();
    write!(&mut reset_stdout, "").unwrap();
}
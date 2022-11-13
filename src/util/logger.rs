use termcolor::{StandardStream, WriteColor, ColorSpec, Color};
use std::{io::Write, fmt::Display};


/// Prints data to the console with the specified forground and background color
/// This logs with a newline
/// For example
/// ```rust
/// # use termcolor::Color;
/// # use server_script::util::logger;
/// 
/// fn main() {
///     logger::color_println("Hello, World", Some(Color::Blue), Some(Color::White))
/// }
/// ```
pub fn color_println<T>(data: T, fg: Option<Color>, bg: Option<Color>) where T: Display {
    color_print(data, fg, bg);
    println!();
}

/// Prints data to the console with the specified forground and background color
/// For example
/// ```rust
/// # use termcolor::Color;
/// # use server_script::util::logger;
/// 
/// fn main() {
///     logger::color_print("Hello, ", Some(Color::Red), Some(Color::White))
///     logger::color_print("World", Some(Color::Red), Some(Color::White))
/// }
/// ```
pub fn color_print<T>(data: T, fg: Option<Color>, bg: Option<Color>) where T: Display {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    let mut spec = ColorSpec::new();
    spec.set_fg(fg).set_bg(bg);

    stdout.set_color(&spec).unwrap();
    write!(&mut stdout, "{}", data).unwrap();

    stdout.reset().unwrap();
}

/// The LogStream struct is for reducing cumbersome subsequent color_print functions with the same foreground and background colors.
pub struct LogStream {
    pub(self) out: StandardStream,
    header: Option<String>,
    pub(self) color: Option<ColorSpec>
}

impl LogStream {
    /// Creates a default LogStream. Links to stdout with no headers.
    pub fn new() -> LogStream {
        LogStream { out: StandardStream::stdout(termcolor::ColorChoice::Always), header: None, color: None }
    }

    /// Creates a LogStream with specified foreground and background colors.
    pub fn with_colors(fg: Option<Color>, bg: Option<Color>) -> LogStream {
        let mut color = ColorSpec::new();
        color.set_fg(fg).set_bg(bg);
        LogStream { color: Some(color), ..Default::default() }
    }

    /// Adds a header which is printed in the same line before printing the data
    pub fn add_header(&mut self, header: String) {
        self.header = Some(header);
    }

    /// Sets the foreground and background color
    pub fn set_color(&mut self, color: ColorSpec) -> Result<(), Box<dyn std::error::Error>> {
        self.color = Some(color);
        Ok(())
    }

    /// Print using the current settings
    pub fn log<T>(&mut self, data: T) -> Result<(), Box<dyn std::error::Error>> where T: Display {
        if let Some(header) = self.header.as_ref() {
            write!(self.out, "{}", header)?;
        }

        if let Some(color) = &self.color {
            self.out.set_color(color)?;
        }

        write!(self.out, "{}", data)?;
        self.out.reset()?;
        Ok(())
    }

    /// Println using the current settings
    pub fn logln<T>(&mut self, data: T) -> Result<(), Box<dyn std::error::Error>> where T: Display {
        self.log(data)?;
        println!();
        Ok(())
    }

}

/// Default impl
impl Default for LogStream {
    fn default() -> Self {
        Self::new()
    }
}
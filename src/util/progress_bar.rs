use termcolor::Color;

use crate::util::logger;

/// ProgressBar struct
pub struct ProgressBar {
    total: u64,
    cursor: u64,
    len: usize,
}

impl ProgressBar {

    /// Create a new ProgressBar
    pub fn new(total: u64) -> ProgressBar {
        ProgressBar { total, cursor: 0, len: 50 }
    }

    /// Set the value of the ProgressBar
    pub fn set_cursor(&mut self, new: u64) {
        self.cursor = new;
    }

    /// Add the value of the ProgressBar
    pub fn add_cursor(&mut self, plus: u64) {
        self.cursor += plus;
    }

    /// Set the value of the ProgressBar to 0
    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    /// Subtracts the value from the ProgressBar
    pub fn subtract_cursor(&mut self, minus: u64) {
        self.cursor -= minus
    }

    /// Prints out the ProgressBar. Carriage return will be used.
    pub fn print(&mut self) {
        let ratio = (self.cursor as f32) / (self.total as f32); // Completion ratio

        let cursor_bar = (ratio * (self.len as f32)) as usize;
        logger::color_print("\r[", Some(Color::White), None);
        logger::color_print("#".repeat(cursor_bar), Some(Color::White), Some(Color::White));  // complete data
        logger::color_print("-".repeat(self.len - cursor_bar), Some(Color::White), None);   // incomplete data
        logger::color_print(format!("] {}% ", (ratio * 100.0) as u64), Some(Color::White), None);
    }

    /// Exits the carriage return
    pub fn clear_text(&mut self) {
        println!();
    }
}
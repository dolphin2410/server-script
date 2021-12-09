use std::cmp::max;
use ansi_term::Colour;
use crate::util::logger;

/// ProgressBar struct
pub struct ProgressBar {
    total: i32,
    finished: i32,
    len: i32,
}

impl ProgressBar {

    /// Create a new ProgressBar
    pub fn new(total: i32) -> ProgressBar {
        ProgressBar { total, finished: 0, len: 50 }
    }

    /// Set the value of the ProgressBar
    pub fn set(&mut self, new: i32) {
        self.finished = new;
    }

    /// Add the value of the ProgressBar
    pub fn add(&mut self, plus: i32) {
        self.finished += plus;
    }

    /// Set the value of the ProgressBar to 0
    pub fn reset(&mut self) {
        self.finished = 0;
    }

    /// Subtracts the value from the ProgressBar
    pub fn subtract(&mut self, minus: i32) {
        self.finished -= minus
    }

    /// Prints out the ProgressBar. Carriage return will be used.
    pub fn print(&mut self) {
        let ratio = (self.finished as f32) / (self.total as f32);

        let finished_bar = (ratio * (self.len as f32)) as i32;

        print!(
            "\r{}",
            Colour::Cyan.paint(format!(
                "[{}{}] {}% ",
                "#".repeat(max(finished_bar as usize, 0)), "-".repeat((self.len - finished_bar) as usize),
                (ratio * 100.0) as i32))
        );
    }
}
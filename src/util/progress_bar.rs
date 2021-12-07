use std::cmp::max;
use crate::util::logger;

pub struct ProgressBar {
    total: f32,
    finished: f32,
    len: i32
}

impl ProgressBar {
    pub fn new(total: f32) -> ProgressBar {
        ProgressBar { total, finished: 0.0, len: 50 }
    }

    pub fn set(&mut self, new: f32) {
        self.finished = new;
    }

    pub fn add(&mut self, plus: f32) {
        self.finished += plus;
    }

    pub fn reset(&mut self) {
        self.finished = 0.0;
    }

    pub fn subtract(&mut self, minus: f32) {
        self.finished -= minus
    }

    pub fn print(&mut self) {
        let ratio = self.finished / self.total;

        let finished_bar = (ratio * (self.len as f32)).floor() as i32;

        let str = format!("\r[{}>{}] {}%     ", "=".repeat(max(finished_bar as usize, 1) - 1), " ".repeat((self.len - finished_bar) as usize), ratio * 100.0);

        logger::log_raw(str.as_str())
    }
}
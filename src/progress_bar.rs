// progress_bar.rs

use std::io::{self, Write};
use std::time::Duration;

pub struct ProgressBar {
    bar_width: usize,
    last_printed_length: usize,
}

impl ProgressBar {
    pub fn new(bar_width: usize) -> Self {
        Self {
            bar_width,
            last_printed_length: 0,
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        // No need to hide cursor
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        // No need to show cursor
        println!(); // Add a newline after the last progress update
        Ok(())
    }

    pub fn update(&mut self, elapsed: Duration, total: Duration) -> io::Result<()> {
        let remaining = if elapsed < total {
            total - elapsed
        } else {
            Duration::from_secs(0)
        };

        let remaining_secs = remaining.as_secs();
        let remaining_mins = remaining_secs / 60;
        let remaining_secs = remaining_secs % 60;

        let progress_percent = (elapsed.as_secs_f64() / total.as_secs_f64()).min(1.0);
        let filled_width = (progress_percent * self.bar_width as f64) as usize;

        // Create the progress bar string
        let mut progress_string = String::new();

        // Add the progress bar
        progress_string.push_str("\r"); // Carriage return to beginning of line
        progress_string.push('[');
        progress_string.push_str(&"=".repeat(filled_width));
        progress_string.push_str(&" ".repeat(self.bar_width - filled_width));
        progress_string.push(']');

        // Add the time remaining
        progress_string.push_str(&format!(
            " Time remaining: {:02}:{:02}",
            remaining_mins, remaining_secs
        ));

        // Add padding to overwrite any previous longer output
        if progress_string.len() < self.last_printed_length {
            progress_string.push_str(&" ".repeat(self.last_printed_length - progress_string.len()));
        }

        // Update last printed length
        self.last_printed_length = progress_string.len();

        // Print without a newline
        print!("{}", progress_string);
        io::stdout().flush()?;

        Ok(())
    }
}

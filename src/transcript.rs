use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Saves the conversation transcript to a file
pub fn save_transcript_to_file(transcript: &str) -> io::Result<()> {
    let file_path = "conversation_transcripts.txt";
    let path = Path::new(file_path);
    let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let file_exists = path.exists();

    let mut file = if file_exists {
        // If file exists, open it in append mode
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)?
    } else {
        // If file doesn't exist, create it
        File::create(file_path)?
    };

    // Write separator and date stamp if file already existed
    if file_exists {
        writeln!(file, "\n\n——————————-NEW ROOM——————————-\n")?;
    }

    // Write date stamp and transcript
    writeln!(file, "Transcript Date: {}\n", date_time)?;
    writeln!(file, "{}", transcript)?;
    println!("Transcript saved to {}", file_path);

    Ok(())
}

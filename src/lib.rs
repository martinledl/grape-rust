use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
pub struct Args {
    /// The pattern to search for
    pub pattern: String,
    /// The file to search in
    pub path: std::path::PathBuf,
    /// Depth to search
    #[clap(short, long, default_value = "1")]
    pub depth: usize,
    /// Output statistics
    #[clap(short, long, default_value = "false")]
    pub stats: bool,
}

pub fn find_matches(args: &Args, reader: BufReader<File>, buf: &mut String) {
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if line.contains(args.pattern.as_str()) {
                    let line = line.replace(
                        args.pattern.as_str(),
                        args.pattern.bright_red().bold().to_string().as_str(),
                    );
                    buf.push_str(&line);
                    buf.push_str("\n");
                }
            }
            // Err(e) => {
            //     eprintln!("Error reading file: {}", e);
            //     return;
            // }
            Err(_) => return,
        }
    }
}

pub fn process_file(
    args: &Args,
    path: &std::path::Path,
    buf: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            return Err(e.into());
        }
    };
    let reader = BufReader::new(file);

    let mut current_file_buf = String::new();
    find_matches(args, reader, &mut current_file_buf);
    if current_file_buf.len() > 0 {
        buf.push_str(&format!("\n\n{}:\n", path.display().to_string().green()));
        buf.push_str(&current_file_buf);
    }
    Ok(())
}

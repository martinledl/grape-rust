use anyhow::Result;
use clap::Parser;
use indicatif::ProgressBar;
use rust_search::SearchBuilder;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = zedl_grep::Args::parse();
    let files: Vec<String> = SearchBuilder::default()
        .location(&args.path)
        .depth(args.depth)
        .build()
        .collect();

    let files_only = files.iter().filter(|path| {
        let path = std::path::Path::new(path);
        path.is_file()
    });

    let file_count = files_only.clone().count();
    let directory_count = files.len() - file_count + 1;

    let pb = ProgressBar::new(file_count as u64);

    let mut buf = String::new();

    for path in files_only {
        let path = std::path::Path::new(&path);
        zedl_grep::process_file(&args, path, &mut buf)?;
        pb.inc(1);
    }

    pb.finish_with_message("Done!\n\n");

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(buf.as_bytes())?;
    handle.flush()?;

    if args.stats {
        println!(
            "\nAnalyzed {} files in {} directories.",
            file_count, directory_count
        );
    }

    Ok(())
}

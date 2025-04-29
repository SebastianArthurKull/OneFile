use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::{ArgAction, Parser};
use ignore::WalkBuilder;

/// Separator written before every file’s contents
const SEP: &str = "\n\n----- {path} -----\n";

/// Flatten a folder into one text file – great for LLM code review.
///
/// Behaviour
/// ----------
/// * Respects **all** patterns in every `.gitignore` file along the walk (via the
///   [`ignore`](https://docs.rs/ignore/) crate).
/// * **Always skips** any path inside a `.git/` directory.
/// * **Always skips** `.gitignore` files themselves (so the ignore rules aren’t
///   concatenated into the final document).
///
/// Add the dependency first if you haven't:
/// ```bash
/// cargo add ignore@"^0.4"
/// ```
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Source folder to flatten
    folder: PathBuf,

    /// Output file (default: ./flattened_code.txt)
    #[arg(short, long, default_value = "flattened_code.txt")]
    output: PathBuf,

    /// Include binary files (raw bytes)
    #[arg(short = 'b', long = "binary", action = ArgAction::SetTrue)]
    include_binary: bool,

    /// Verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if !cli.folder.is_dir() {
        anyhow::bail!("‘{}’ is not a directory", cli.folder.display());
    }

    if cli.verbose {
        eprintln!(
            "Flattening ‘{}’ → ‘{}’ (binary: {})",
            cli.folder.display(),
            cli.output.display(),
            cli.include_binary
        );
    }

    let mut writer = BufWriter::new(
        File::create(&cli.output)
            .with_context(|| format!("cannot create {}", cli.output.display()))?,
    );

    // Build an iterator that walks the directory while respecting `.gitignore`
    // files. Hidden files are included unless ignored explicitly, because they
    // may contain code worth reviewing (e.g. `.env.sample`).
    let walk = WalkBuilder::new(&cli.folder)
        .add_custom_ignore_filename(".gitignore")
        .hidden(false)
        .build();

    for result in walk {
        let entry = match result {
            Ok(e) => e,
            Err(err) => {
                if cli.verbose {
                    eprintln!("(error) {}", err);
                }
                continue;
            }
        };

        // Walker yields both files and directories.
        if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            continue;
        }

        let path = entry.path();
        let rel_path = path.strip_prefix(&cli.folder).unwrap_or(path);

        // Skip anything inside a `.git/` directory.
        if rel_path.components().any(|c| c.as_os_str() == ".git") {
            if cli.verbose {
                eprintln!("(skip – .git) {}", rel_path.display());
            }
            continue;
        }

        // Skip the `.gitignore` files themselves.
        if path.file_name().and_then(|s| s.to_str()) == Some(".gitignore") {
            if cli.verbose {
                eprintln!("(skip – .gitignore) {}", rel_path.display());
            }
            continue;
        }

        // Skip binary files unless requested.
        if !cli.include_binary && looks_binary(path)? {
            if cli.verbose {
                eprintln!("(skip – binary) {}", rel_path.display());
            }
            continue;
        }

        if cli.verbose {
            eprintln!("{}", rel_path.display());
        }

        // Write header.
        writeln!(
            writer,
            "{}",
            SEP.replace("{path}", &rel_path.to_string_lossy())
        )?;

        // Stream file contents.
        if cli.include_binary {
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut writer)?;
        } else {
            let reader = BufReader::new(File::open(path)?);
            for line in reader.lines() {
                writeln!(writer, "{}", line?)?;
            }
        }
    }

    writer.flush()?;
    if cli.verbose {
        eprintln!("✅ Done!");
    }
    Ok(())
}

/// Heuristic: treat a file as binary when the first 512 B are not valid UTF‑8.
fn looks_binary(path: &Path) -> Result<bool> {
    let mut buf = [0u8; 512];
    let mut f = File::open(path)?;
    let n = f.read(&mut buf)?;
    Ok(std::str::from_utf8(&buf[..n]).is_err())
}

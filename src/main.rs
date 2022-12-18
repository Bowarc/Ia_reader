use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

use colored::Colorize;

fn search_dir(dir: &Path, search_string: &str) -> (u32, u32, u32, u32) {
    let mut num_matches = 0;
    let mut num_files = 0;
    let mut num_dirs = 0;
    let mut num_files_with_matches = 0;
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            num_files += 1;
            let file = match fs::File::open(path.clone()) {
                Ok(file) => file,
                Err(_) => continue,
            };
            let mut reader = std::io::BufReader::new(file);
            let mut file_matches = 0;
            let mut buffer = String::new();
            loop {
                let bytes_read = match reader.read_line(&mut buffer) {
                    Ok(bytes_read) => bytes_read,
                    Err(_) => {
                        break;
                    }
                };
                if bytes_read == 0 {
                    break;
                }
                if buffer.contains(search_string) {
                    file_matches += 1;
                }
                buffer.clear();
            }

            if file_matches > 0 {
                println!(
                    "{} {}",
                    path.display().to_string().green(),
                    format!("({} occurrences)", file_matches).red().bold()
                );
                num_matches += file_matches;
                num_files_with_matches += 1;
            }
        } else if path.is_dir() {
            num_dirs += 1;
            let temp = search_dir(&path, search_string);
            num_matches += temp.0;
            num_files += temp.1;
            num_dirs += temp.2;
            num_files_with_matches += temp.3;
        }
    }
    (num_matches, num_files, num_dirs, num_files_with_matches)
}
fn main() {
    let search_string = std::env::args()
        .nth(1)
        .expect("please specify a search string");

    let start_time = Instant::now();
    let (num_matches, num_files, num_dirs, num_files_with_matches) =
        search_dir(Path::new("."), &search_string);
    let elapsed_time = start_time.elapsed();

    let mut elapsed_time_string = String::new();
    let elapsed_time_secs = elapsed_time.as_secs_f64();

    let elapsed_hours = (elapsed_time_secs / 3600.0) as u64;
    if elapsed_hours > 0 {
        elapsed_time_string.push_str(&format!("{:02}h ", elapsed_hours));
    }
    let elapsed_minutes = ((elapsed_time_secs % 3600.0) / 60.0) as u64;
    if elapsed_minutes > 0 {
        elapsed_time_string.push_str(&format!("{:02}m ", elapsed_minutes));
    }
    let elapsed_seconds = (elapsed_time_secs % 60.0) as u64;
    if elapsed_seconds > 0 {
        elapsed_time_string.push_str(&format!("{:02}s ", elapsed_seconds));
    }
    let elapsed_milliseconds = ((elapsed_time_secs - elapsed_seconds as f64) * 1000.0) as u64;
    if elapsed_milliseconds > 0 {
        elapsed_time_string.push_str(&format!("{:02}ms ", elapsed_milliseconds));
    }

    let box_width = 50;
    let divider = "".repeat(box_width);

    println!("\n{}", "-".bold().cyan().repeat(box_width));
    println!(
        "|{:^48}|",
        format!(
            "Found {} matches in {} files",
            num_matches, num_files_with_matches
        )
    );
    println!(
        "|{:^48}|",
        format!("Searched {} files in {} directories", num_files, num_dirs)
    );
    println!("|{:^48}|", format!("Took {}", elapsed_time_string));
    println!("{}", "-".bold().cyan().repeat(box_width));
    println!("{}", divider);
}

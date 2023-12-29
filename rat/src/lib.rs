use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(files: Vec<String>, number_lines: bool, number_nonblank: bool) -> MyResult<()> {
    for filename in files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(handle) => {
                let mut last_line = 0;
                for (line_num, line_raw) in handle.lines().enumerate() {
                    let line = line_raw?;
                    if number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if number_nonblank {
                        if line.is_empty() {
                            println!();
                        } else {
                            last_line += 1;
                            println!("{:6}\t{}", last_line, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/* Structure for UNIQ */
use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Write}, path::{PathBuf}
};

pub struct UNIQ {
    file_path: PathBuf,
}

impl UNIQ {
    pub fn new(file_path: PathBuf) -> UNIQ {
        UNIQ { file_path: PathBuf::from(file_path), }
    }

    fn count_lines(&self) -> HashMap<String, u32> {
        let mut counts = HashMap::new();
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    *counts.entry(line).or_insert(0) += 1;
                }
            }
        }
        counts
    }

    pub fn print_lines(&self) {
        let counts = self.count_lines();
        for(line, _) in counts.iter() {
            println!("{}",line);
        }
    }


    pub fn print_uniq(&self) {
        let counts = self.count_lines();
        for (line, _) in counts.iter().filter(|&(_, &count)| count != 2) {
            println!("{}", line);
        }
    }

    pub fn print_duplicates(&self) {
        let counts = self.count_lines();
        for (line, &_) in counts.iter().filter(|&(_, &count)| count == 2) {
            println!("{}", line);
        }
    }

    pub fn count_unique_lines(&self) {
        let counts = self.count_lines();
        let mut dummy = 0;
        for &count in counts.values() {
            if count == 1 {
                dummy += count;
            } else if count > 2 {
                continue;
            }
        }
        println!("{}", dummy);
    }

    pub fn print_counters(&self) {
        let counts = self.count_lines();
        for (line, &count) in &counts {
            println!("{} {}", count, line);
        }
    }

    pub fn read_and_write(&self) {
        let file_path = PathBuf::from(&self.file_path);
        let mut file = File::create(&file_path).unwrap();
        let stdin = io::stdin();
        let locked_stdin = stdin.lock();

        for line in locked_stdin.lines() {
            let line = line.expect("Failed to read line");

            if line.trim().is_empty() {
                break;
            }
            file.write_all(line.trim().as_bytes()).expect("Failed to write to file");
            file.write_all(b"\n").expect("Failed to write to file");
        }
    }
}
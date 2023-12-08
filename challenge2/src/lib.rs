use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.flag_name == "filepath" {
        let file = File::open(config.file_path)?;
        let lines = io::BufReader::new(file).lines();
        get_calibration_value(lines);
    }
    Ok(())
}

pub struct Config {
    pub flag_name: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let flag_name = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config {
            flag_name,
            file_path,
        });
    }
}

fn get_calibration_value(calibration_doc_lines: Lines<BufReader<File>>) {
    let mut calibration_number = 0;
    for line in calibration_doc_lines {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        if let Ok(calibration_line) = line {
            let characters: Vec<char> = calibration_line.chars().collect();
            for char in characters {
                if char.is_numeric() {
                    if first == None {
                        first = Some(char);
                    }

                    last = Some(char);
                }
            }

            let final_line_string = format!("{}{}", first.unwrap(), last.unwrap());
            let final_line_value: i32 = final_line_string.parse().unwrap();
            calibration_number = calibration_number + final_line_value;
        }
    }
    println!("calibration value: {}", calibration_number)
}

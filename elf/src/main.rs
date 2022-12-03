// Useful documentation: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let (
        mut maximum, 
        mut second_max, 
        mut third_max, 
        ): (u64, u64, u64) = (0, 0, 0);

    if let Ok(lines) = read_lines(file_path) {
        let mut current: u64 = 16;

        for lines in lines {
            if let Ok(ip) = lines {
                if ip == "" {
                    if current > maximum {
                        second_max = maximum;
                        maximum = current;
                    } else if current > second_max {
                        third_max = second_max;
                        second_max = maximum;
                    } else if current > third_max {
                        third_max = current;
                    }
                  
                    current = 0;
                } else {
                    let parsed: u64 = ip.parse().unwrap();

                    current = current + parsed;
                }
            }
        }
    } 

    println!("Max Calories: {}", maximum);
    println!("Secondary Max Calories: {}", maximum + second_max + third_max);
}

fn read_lines<P>(file: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> 
where P: AsRef<Path>, {
    let file = fs::File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}

use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day1() -> usize {
    let filepath = "../input01.txt".to_string();
    let lines = read_lines(filepath);

    let mut max = 0;
    let mut current_sum = 0;
    for line in lines {
        let line_str = line.unwrap();
        if current_sum > max {
           max = current_sum;
        }
        if line_str.is_empty() {
            current_sum = 0;
            continue;
        }
        current_sum += line_str.parse::<usize>().unwrap();
    }
    max
}

fn read_lines(filename: String) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

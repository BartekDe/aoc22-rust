use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn calculate_for_top_x(x: usize) -> usize {
    let filepath = "../input01.txt".to_string();
    let lines = read_lines(filepath);

    let mut top_x_numbers: Vec<usize> = vec![0; x];
    let mut current_sum = 0;
    for line in lines {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            if current_sum > top_x_numbers[0] {
                top_x_numbers[0] = current_sum;
                top_x_numbers.sort();
            }
            current_sum = 0;
            continue;
        }
        current_sum += line_str.parse::<usize>().unwrap();
    }

    let mut sum = 0;
    for i in top_x_numbers {
        sum += i;
    }
    sum
}

fn read_lines(filename: String) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

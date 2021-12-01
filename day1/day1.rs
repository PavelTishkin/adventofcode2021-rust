use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let contents = fs::read_to_string(input_file)
        .expect("Can't read file");
    let contents_lines = contents.split("\n");
    let contents_vec = contents_lines.collect::<Vec<&str>>();
    let mut contents_int: Vec<i32> = Vec::new();
    for str_val in contents_vec {
        contents_int.push(str_val.parse::<i32>().unwrap());
    }

    let mut prev_num = contents_int[0];
    let mut inc_count = 0;
    for val in &contents_int {
        if prev_num < *val {
            inc_count = inc_count + 1;
        }
        prev_num = *val;
    }

    let mut prev_sum = contents_int[0] + contents_int[1] + contents_int[2];
    let mut sum_inc_count = 0;
    for i in 1..contents_int.len() - 2 {
        let curr_sum = contents_int[i] + contents_int[i+1] + contents_int[i+2];
        if prev_sum < curr_sum {
            sum_inc_count = sum_inc_count + 1;
        }
        prev_sum = curr_sum;
    }

    println!("Answer 1: {}", inc_count);
    println!("Answer 2: {}", sum_inc_count);
}
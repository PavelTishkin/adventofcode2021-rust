use std::env;
use std::f32::RADIX;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    // Read input contents
    let file_contents = fs::read_to_string(input_file)
        .expect("Can't read file");
    let file_lines = file_contents.split("\n").collect::<Vec<&str>>();
    
    // Split file lines into array of chars
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for file_line in file_lines {
        matrix.push(file_line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
    }

    let (gamma_rate, eps_rate) = get_gamma_eps_rates(matrix.clone());

    //let co2_rate = 0;

    let oxy_rate = get_support_rate(matrix.clone(), true);
    let co2_rate = get_support_rate(matrix.clone(), false);

    println!("Answer 1: {}", gamma_rate * eps_rate);
    println!("Answer 2: {}", oxy_rate * co2_rate);
}

// Calculate gamma and epsilon rates of an array
fn get_gamma_eps_rates(m: Vec<Vec<u32>>) -> (u32, u32) {
    let m_t = matrix_transpose(m);
    let mut gamma_rate = 0;
    let mut eps_rate = 0;
    for row in m_t {
        let (mcb, _mcb_count) = get_mcb(row);
        let mut lcb = 0;
        if mcb == 0 {
            lcb = 1;
        }
        gamma_rate = gamma_rate << 1;
        gamma_rate += mcb;
        eps_rate = eps_rate << 1;
        eps_rate += lcb;
    }
    (gamma_rate, eps_rate)
}

// Calculate life support rating on an array
// is_oxy switches between oxygen and co2 ratings
fn get_support_rate(m: Vec<Vec<u32>>, is_oxy: bool) -> u32 {
    // Loop until just one element remains
    let mut i = 0;
    let mut m_c = m.clone();
    while m_c.len() > 1 && i < m_c[0].len() {
        let m_t = matrix_transpose(m_c.clone());
        let curr_row = m_t[i].clone();
        let row_len = curr_row.len() as u32;
        let (mcb, mcb_count) = get_mcb(curr_row);
        let mut lcb = 0;
        if mcb == 0 {
            lcb = 1;
        }
        let lcb_count = row_len - mcb_count;
        let filter;
        if is_oxy {
            if mcb_count > lcb_count {
                filter = mcb;
            } else if mcb_count < lcb_count {
                filter = lcb;
            } else {
                filter = 1;
            }
        } else {
            if mcb_count > lcb_count {
                filter = lcb;
            } else if mcb_count < lcb_count {
                filter = mcb;
            } else {
                filter = 0;
            }
        }
        m_c = m_c
        .into_iter()
        .filter(|r| r[i] == filter)
        .collect();
        i += 1;
    }
    bin_str_to_int(m_c[0].clone())
}

// Transpose rows and cols of a matrix
fn matrix_transpose(m: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

// Get most common byte of an array and number of occurrences
fn get_mcb(v: Vec<u32>) -> (u32, u32) {
    let mut one_count: u32 = 0;
    let v_len = v.len() as u32;
    for c in &v {
        one_count += *c;
    }
    if one_count > v_len - one_count {
        return (1, one_count);
    } else {
        return (0, v_len - one_count);
    }
}

// Convert array of ints into a single number using binary encoding
fn bin_str_to_int(v: Vec<u32>) -> u32 {
    let mut result = 0;
    for val in v {
        result = result << 1;
        result += val;
    }
    result
}
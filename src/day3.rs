use crate::utils;

pub fn q1() {
    let content = utils::file_to_string("src/inputs/day3.txt");
    let matrix:Vec<Vec<i32>> = utils::parse_to_int_matrix(&content);
    let row_count = matrix.len() as i32;
    let first_row_length = matrix.iter().nth(0).expect("Empty").len() as i32;
    let mut gamma:Vec<&str> = Vec::new();
    let mut epsilon:Vec<&str> = Vec::new();

    let threshold = row_count/2;

    for i in 0..first_row_length {
        let mut buckets = (0,0);
        
        let d:Vec<i32> = matrix.iter().map(|x| x[i as usize]).collect();
        let total = d.iter().fold(0, |mut acc, x| acc + x);
        match threshold < total {
            true => {
                gamma.push("1");
                epsilon.push("0");
            },
            false => {
                gamma.push("0");
                epsilon.push("1");
            }
        }
    }


    let gamma_o = isize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon_o = isize::from_str_radix(&epsilon.join(""), 2).unwrap();

    println!("Q1 answer: {}",gamma_o * epsilon_o );
}

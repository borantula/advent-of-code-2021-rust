#![allow(dead_code)]
use std::fs;

// pub fn file_to_lines(path: &str) -> Vec<String> {
//     let path = Path::new(path);
//     let file = File::open(&path).unwrap();
//     let lines = io::BufReader::new(file).lines();
//     let mut str_lines: Vec<String> = vec![];

//     for line in lines {
//         if let Ok(line) = line {
//             str_lines.push(line);
//         };
//     }
//     str_lines
// }

pub fn file_to_string(path: &str) -> String {
    let contents = fs::read_to_string(path)
                        .expect(&format!("Something went wrong reading the file: {}", path));
    contents
}

pub fn content_to_lines(content: &str) ->  Vec<&str> {
    content.lines().collect()
}

pub fn parse_by_empty_lines(content: &str) -> Vec<&str> {
    let splitted = content.split("\n\n").collect();
    splitted
}

pub fn split_each_char(s: &str) -> Vec<&str> {
    let result:Vec<&str> = s.split("").filter(|&x| x != "").collect();
    // println!("R : {:?}", &result);
    result
}

pub fn parse_to_matrix(content: &str) -> Vec<Vec<&str>> {
    let lines = content_to_lines(content);
    let matrix:Vec<Vec<&str>> = lines.iter().map(|e| split_each_char(e)).collect();
    matrix
}

pub fn parse_to_int_matrix(content: &str) -> Vec<Vec<i32>> {
    let lines = content_to_lines(content);
    let matrix:Vec<Vec<i32>> = lines.iter()
                                    .map(|e| split_each_char(e))
                                    .map(lines_to_int)
                                    .collect();
    matrix
}

pub fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().expect("not convertable to number")
}

pub fn lines_to_int(lines:Vec<&str>)->  Vec<i32> {
    lines.into_iter().map(parse_int).collect()
}
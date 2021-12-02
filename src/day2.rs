use crate::utils;

pub fn q1() {
    let content = utils::file_to_string("src/inputs/day2.txt");
    let lines:Vec<&str> = utils::content_to_lines(&content);

    let mut coord:(i32,i32) = (0,0);
    for line in lines {
        let rule:Vec<&str> = line.split(" ").collect();
        let amount = utils::parse_int(rule[1]);

        let dir = rule[0];

        match dir {
            "forward" => coord.0 += amount,
            "down" => coord.1 += amount,
            "up" => coord.1 -= amount,
            _ => println!("no dir {}",dir)
        }
    }
    println!("Day2 Q1{:?}",coord.0 * coord.1)
}


pub fn q2() {
    let content = utils::file_to_string("src/inputs/day2.txt");
    let lines:Vec<&str> = utils::content_to_lines(&content);

    // horizontal, depth, aim
    let mut coord:(i32,i32,i32) = (0,0,0);
    for line in lines {
        let rule:Vec<&str> = line.split(" ").collect();
        let amount = utils::parse_int(rule[1]);

        let dir = rule[0];

        match dir {
            "forward" => {
                coord.0 += amount;
                coord.1 += amount * coord.2;
            },
            "down" => {
                coord.2 += amount;
            },
            "up" => {
                coord.2 -= amount;
            },
            _ => println!("no dir {}",dir)
        }
        println!("Day2 Q2 {:?}",coord)
    }
    println!("Day2 Q2 {:?}",coord.0 * coord.1)
}

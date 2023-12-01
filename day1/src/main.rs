fn main() {
    part1();
    part2()
}

fn part1() {
    let inp = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut res = 0;

    inp.lines().for_each(|l| {
        for c in l.chars() {
            if c.is_numeric() {
                res += (c.to_digit(10).unwrap() as i32) * 10;
                break;
            }
        }
        for c in l.chars().rev() {
            if c.is_numeric() {
                res += c.to_digit(10).unwrap() as i32;
                break;
            }
        }
    });
    println!("Part1: {}", res);
}

fn part2() {
    let number_strings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let inp = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut res = 0;

    inp.lines().for_each(|l| {
        let mut lowest_index = usize::MAX;
        let mut highest_index = usize::MIN;
        for (i, c) in l.chars().enumerate() {
            if c.is_numeric() {
                lowest_index = i;
                break;
            }
        }
        for (i, c) in l.chars().rev().enumerate() {
            if c.is_numeric() {
                highest_index = l.len() - i - 1;
                break;
            }
        }
        let mut lowest_found = false;
        let mut highest_found = false;
        let mut line_res_first = 0;
        let mut line_res_second = 0;
        for s in number_strings.iter() {
            if let Some(i) = l.find(s) {
                if i < lowest_index {
                    lowest_found = true;
                    lowest_index = i;
                    line_res_first = match_num(s);
                }
            }
            if let Some(i) = l.rfind(s) {
                if i > highest_index {
                    highest_found = true;
                    highest_index = i;
                    line_res_second = match_num(s);
                }
            }
        }
        if !lowest_found {
            res += (l.chars().nth(lowest_index).unwrap().to_digit(10).unwrap() as i32) * 10;
        } else {
            res += line_res_first * 10;
        }
        if !highest_found {
            res += l.chars().nth(highest_index).unwrap().to_digit(10).unwrap() as i32;
        } else {
            res += line_res_second;
        }
    });
    println!("Part2: {}", res);
}

fn match_num(s: &str) -> i32 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Invalid number"),
    };
}

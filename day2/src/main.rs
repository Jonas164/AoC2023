use core::panic;

fn main() {
    part1();
    part2();
}

fn part1() {
    let inp = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut res = 0;
    for (i, l) in inp.lines().enumerate() {
        let mut max_red: i32 = 12;
        let mut max_green: i32 = max_red + 1;
        let mut max_blue: i32 = max_green + 1;
        let mut valid = true;
        let game_id = i + 1;
        let split_into_hands = &l[l.find(":").unwrap() + 1..l.len()]
            .split(";")
            .collect::<Vec<&str>>();
        for h in split_into_hands {
            let hand = h.replace(",", "");
            let tokens = hand.split_whitespace().collect::<Vec<&str>>();
            for i in (0..tokens.len()).step_by(2) {
                let count = tokens.get(i).unwrap().parse::<i32>().unwrap();
                let color = tokens.get(i + 1).unwrap();
                match *color {
                    "red" => {
                        max_red -= count;
                    }
                    "green" => {
                        max_green -= count;
                    }
                    "blue" => {
                        max_blue -= count;
                    }
                    _ => panic!("Unknown color"),
                }
            }
            if max_red < 0 || max_green < 0 || max_blue < 0 {
                valid = false;
                break;
            } else {
                max_red = 12;
                max_green = max_red + 1;
                max_blue = max_green + 1;
            }
        }
        if valid {
            res += game_id;
        }
    }
    println!("Part 1: {}", res);
}

fn part2() {
    let inp = std::fs::read_to_string("input.txt").expect("Error reading file");
    let mut res = 0;
    for l in inp.lines() {
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        let split_into_hands = &l[l.find(":").unwrap() + 1..l.len()]
            .split(";")
            .collect::<Vec<&str>>();
        for h in split_into_hands {
            let hand = h.replace(",", "");
            let tokens = hand.split_whitespace().collect::<Vec<&str>>();
            for i in (0..tokens.len()).step_by(2) {
                let count = tokens.get(i).unwrap().parse::<i32>().unwrap();
                let color = tokens.get(i + 1).unwrap();
                match *color {
                    "red" => {
                        if count > red {
                            red = count;
                        }
                    }
                    "green" => {
                        if count > green {
                            green = count;
                        }
                    }
                    "blue" => {
                        if count > blue {
                            blue = count;
                        }
                    }
                    _ => panic!("Unknown color"),
                }
            }
        }
        res += red * green * blue;
    }
    println!("Part 2: {}", res);
}

use std::env;
use std::fmt::{self, Formatter, Display};
use std::fs;

struct Move {
    direction: String,
    amount: i32,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{{} {}}}", self.direction, self.amount)
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct AimPosition {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let contents = fs::read_to_string(input_file)
        .expect("Can't read file");
    let contents_lines = contents.split("\n");
    let contents_vec = contents_lines.collect::<Vec<&str>>();
    let mut moves: Vec<Move> = Vec::new();
    for str_val in contents_vec {
        let move_str = str_val.split(" ").collect::<Vec<&str>>();
        let move_obj = Move {direction: move_str[0].to_string(),
            amount: move_str[1].parse::<i32>().unwrap()};
        moves.push(move_obj);
        // println!("{}", move_obj);
    }

    let mut pos = Position {x: 0, y: 0};
    let mut aim_pos = AimPosition {x: 0, y: 0, aim: 0};
    for mov in moves {
        if mov.direction == "up" {
            pos.y -= mov.amount;
            aim_pos.aim -= mov.amount;
        } else if mov.direction == "down" {
            pos.y += mov.amount;
            aim_pos.aim += mov.amount;
        } else if mov.direction == "forward" {
            pos.x += mov.amount;
            aim_pos.x += mov.amount;
            aim_pos.y += mov.amount * aim_pos.aim;
        }
    }

    println!("Answer 1: {}", pos.x * pos.y);
    println!("Answer 2: {}", aim_pos.x * aim_pos.y);
}
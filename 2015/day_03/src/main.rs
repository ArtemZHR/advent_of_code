use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
    //a
    let mut a = HashSet::<String>::new();
    a.insert(String::from("0x0"));
    include_str!("../input.txt")
        .split("")
        .collect::<Vec<&str>>()
        .iter()
        .fold(State(0, 0), |cur_coords, direction| {
            let mut state = State(0, 0);
            match *direction {
                ">" => state = State(cur_coords.0 + 1, cur_coords.1),
                "<" => state = State(cur_coords.0 - 1, cur_coords.1),
                "^" => state = State(cur_coords.0, cur_coords.1 + 1),
                "v" => state = State(cur_coords.0, cur_coords.1 - 1),
                _ => {}
            };
            a.insert(std::fmt::format(format_args!("{}x{}", state.0, state.1)));
            return state;
        });

    println!("{}", a.len());

    //b
    let ways = include_str!("../input.txt")
        .split("")
        .collect::<Vec<&str>>();

    let santa_ways = ways
        .iter()
        .enumerate()
        .filter(|&(index, char)| index % 2 == 0)
        .map(|(_, e)| *e)
        .into_iter()
        .collect();

    let robosanta_ways = ways
        .iter()
        .enumerate()
        .filter(|&(index, char)| index % 2 == 1)
        .map(|(_, e)| *e)
        .into_iter()
        .collect();

    let direction = |direction: Vec<&str>| {
        let mut ways = HashSet::new();
        direction.iter()
            .fold(State(0, 0), |cur_coords, direction| {
                let mut state = State(0, 0);
                match *direction {
                    ">" => state = State(cur_coords.0 + 1, cur_coords.1),
                    "<" => state = State(cur_coords.0 - 1, cur_coords.1),
                    "^" => state = State(cur_coords.0, cur_coords.1 + 1),
                    "v" => state = State(cur_coords.0, cur_coords.1 - 1),
                    _ => {}
                };
                ways.insert(std::fmt::format(format_args!("{}x{}", state.0, state.1)));
                return state;
            });
        ways
    };


    let mut santa = direction(santa_ways);
    let robosanta_ways= direction(robosanta_ways);

    santa.extend(robosanta_ways.into_iter());
    println!("{:?}", santa.len());
}

#[derive(Debug)]
struct State(i32, i32);

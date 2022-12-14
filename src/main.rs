use clap::Parser;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

mod io;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Day to run
   #[arg(short, long, default_value_t = 1)]
   day: u8,
}

fn main() {
    let args = Args::parse();

    if args.day == 1 {
        day1(String::from("data/input01.txt"));
    }
    if args.day == 2 {
        day2(String::from("data/input02.txt"))
    }
    if args.day == 3 {
        day3(String::from("data/input03.txt"));
        day3_pt2(String::from("data/input03.txt"))
    }
    if args.day == 4 {
        day4(String::from("data/input04.txt"));
    }
    if args.day == 5 {
        day5(String::from("data/input05.txt"));
    }
    if args.day == 6 {
        day6(String::from("data/input06.txt"));
    }
}

fn day1(input_file: String) {
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    let mut elf_calories: Vec<usize> = Vec::new();
    elf_calories.push(0);

    let mut curr_elf = 0;
    for snack in file_contents {
        // println!("bytes: {:?}", snack);
        if snack.len() == 0 {
            elf_calories.push(0);
            // println!("-- {}", elf_calories[curr_elf]);
            curr_elf += 1;
        } else {
            let s = match String::from_utf8(snack) {
                Ok(v) => v.parse::<usize>().unwrap(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            // println!("{}", s);
            elf_calories[curr_elf] += s;
        }
    }

    elf_calories.sort();
    println!("The total elf count is {curr_elf}");
    for ind in (curr_elf-2)..(curr_elf+1) {
       let max_calories = elf_calories[ind];
       println!("The calorie count for elf {ind} is {max_calories}");
    }
}

fn day2(input_file: String) {
    // R=1, P=2, S=3
    // L=0, D=3, W=6
    // X=L, Y=D, Z=W

    let mut outcomes: HashMap<u16, u8> = HashMap::new();
    // MAP FROM PART 1
    // outcomes.insert(6588, 4);
    // outcomes.insert(6589, 8);
    // outcomes.insert(6590, 3);
    // outcomes.insert(6688, 1);
    // outcomes.insert(6689, 5);
    // outcomes.insert(6690, 9);
    // outcomes.insert(6788, 7);
    // outcomes.insert(6789, 2);
    // outcomes.insert(6790, 6);

    outcomes.insert(6588, 3);
    outcomes.insert(6589, 4);
    outcomes.insert(6590, 8);
    outcomes.insert(6688, 1);
    outcomes.insert(6689, 5);
    outcomes.insert(6690, 9);
    outcomes.insert(6788, 2);
    outcomes.insert(6789, 6);
    outcomes.insert(6790, 7);

    let mut score = 0usize;
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    for round in file_contents {
        let result = usize::from(outcomes.get(&(u16::from(round[0])*100u16 + u16::from(round[2]))).unwrap().to_owned());
        score += result;
        println!("{:?} || result: {} || score: {score}", round, result);
    }

    println!("Total score from strategy is: {score}");

}

fn day3(input_file: String) {
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    let mut score: usize = 0;
    for sack in file_contents {
        // println!("{:?}", String::from_utf8(sack.clone()));
        let half_count = sack.len() / 2;
        let mut side1: HashSet<u8> = HashSet::new();
        let mut side2: HashSet<u8> = HashSet::new();
        // Split sack into 2 sides, do a set build + set diff, then score w/ byte
        let mut idx = 0;
        while idx < half_count {
            side1.insert(sack[idx]);
            // println!("Inserted a {} into side1...", char::from(sack[idx]));
            side2.insert(sack[idx + half_count]);
            // println!("Inserted a {} into side2...", char::from(sack[idx+half_count]));
            idx += 1;
        }
        // println!("{:?}", side1);
        // println!("{:?}", side2);
        for missing in side1.intersection(&side2) {
            // println!("Found a {} in the symmetric diff...", char::from(missing.to_owned()));
            if missing.to_owned() > 97u8 {
                score += usize::from(missing.clone() - 96);
            } else {
                score += usize::from(missing.clone() - 64 + 26);
            }
        }
        // println!("...");
    }
    println!("The total priority score is {score}");


}

fn day3_pt2(input_file: String) {
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    let mut score: usize = 0;
    for sacks in file_contents.chunks(3) {
        // println!("{:?}", String::from_utf8(sack.clone()));
        let sack1: HashSet<u8> = HashSet::from_iter(sacks[0].as_slice().iter().cloned());
        let sack2: HashSet<u8> = HashSet::from_iter(sacks[1].as_slice().iter().cloned());
        let sack3: HashSet<u8> = HashSet::from_iter(sacks[2].as_slice().iter().cloned());

        for candidate in sack1.intersection(&sack2) {
            // println!("Found a {} in the symmetric diff...", char::from(missing.to_owned()));
            if sack3.contains(candidate) {
                if candidate.to_owned() > 97u8 {
                    score += usize::from(candidate.clone() - 96);
                } else {
                    score += usize::from(candidate.clone() - 64 + 26);
                }
            }
        }
        // println!("...");
    }
    println!("The total priority score of the badges is {score}");


}

fn day4(input_file: String) {
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    let mut score: usize = 0;
    let mut intersect_score = 0;
    // let mut line = 0;
    // This is about to be ugly -- effectively doing a split -> split -> split approach :| 
    for assignments in file_contents {
        // println!("Working on Line {line} || {}", String::from_utf8(assignments.clone()).unwrap());
        // println!("Bytes: {:?}", assignments);
        let mut assignment_parts = assignments.split(|byte| byte == &44u8);
        let elf1 = assignment_parts.next().unwrap().to_owned();
        let elf2 = assignment_parts.next().unwrap().to_owned();
        
        let mut elf1_parts = elf1.split(|byte| byte == &45u8);
        let elf1_lhs_str = String::from_utf8(elf1_parts.next().unwrap().to_owned()).unwrap();
        // println!(".. Elf 1 LHS: {}", elf1_lhs_str);
        let elf1_lhs = usize::from_str_radix(&elf1_lhs_str, 10).unwrap();

        let elf1_rhs_str = String::from_utf8(elf1_parts.next().unwrap().to_owned()).unwrap();
        // println!(".. Elf 1 RHS: {}", elf1_rhs_str);
        let elf1_rhs = usize::from_str_radix(&elf1_rhs_str, 10).unwrap();
        let elf1_range: HashSet<usize> = HashSet::from_iter(elf1_lhs..(elf1_rhs+1));

        let mut elf2_parts = elf2.split(|byte| byte == &45u8);
        let elf2_lhs = usize::from_str_radix(&String::from_utf8(elf2_parts.next().unwrap().to_owned()).unwrap(), 10).unwrap();
        let elf2_rhs = usize::from_str_radix(&String::from_utf8(elf2_parts.next().unwrap().to_owned()).unwrap(), 10).unwrap();
        let elf2_range: HashSet<usize> = HashSet::from_iter(elf2_lhs..(elf2_rhs+1));

        if elf2_range.is_subset(&elf1_range) {
            score += 1;
        } else if elf1_range.is_subset(&elf2_range) {
            score += 1
        }
        let intersection: HashSet<_> = elf2_range.intersection(&elf1_range).collect();

        if !intersection.is_empty() {
            intersect_score += 1;
        }
        // line += 1;
    }
    println!("The total overlap score is {score}");
    println!("The total intersection score is {intersect_score}");
}

fn day5(input_file: String) {
    let file_contents = io::split_file(Path::new(&input_file.to_owned()), 10);
    let crate_re = Regex::new(r"\[([A-Z]){1}\]").unwrap();
    let move_re = Regex::new(r"move (\d{1,}) from (\d) to (\d)").unwrap();

    // First, let's build the input grid and create a VecDeque from it
    let mut in_input = true;
    let mut curr_ind = 0;
    while in_input {
        if file_contents[curr_ind].len() == 0 {
            in_input = false;
        } else {
            curr_ind += 1;
        }
    }

    println!("{}", curr_ind);

    // count how man stacks we have (assume < 10, for now)
    let num_stacks = file_contents[curr_ind-1].iter().filter(|&n| *n != 32).count();
    println!("{}", num_stacks);
    if num_stacks > 9 {
        panic!("Zoinks!  There's more than 10 stacks... implement that logic!");
    }

    // we have 9 or fewer stacks, let's init the crate_stacks
    let mut crate_stacks: Vec<VecDeque<u8>> = vec![VecDeque::<u8>::new(); num_stacks];

    let mut curr_col = 0;
    let mut curr_stack = 0;
    for val in file_contents[curr_ind-1].clone().into_iter() {
        // if we aren't on a space byte
        if val != 32 {
            for row in 0..(curr_ind-1) {
                if file_contents[row][curr_col] >= 65 && file_contents[row][curr_col] <= 90 {
                    crate_stacks[curr_stack].push_back(file_contents[row][curr_col].clone());
                    // println!("Pushing {} onto stack {}", file_contents[row][curr_col], curr_stack);
                }
            }
            // println!("{:?}", crate_stacks[curr_stack]);
            curr_stack += 1;
        }
        curr_col += 1;
    }

    // vec of stacks is made, now we need to mutate them
    for ind in (curr_ind+1)..file_contents.len() {
        let command_string = String::from_utf8(file_contents[ind].clone()).unwrap();
        // println!("{}", command_string);
        for cap in move_re.captures_iter(&command_string) {
            let qty_moved = usize::from_str_radix(&cap[1], 10).unwrap();
            let move_from = usize::from_str_radix(&cap[2], 10).unwrap() - 1;
            let move_to = usize::from_str_radix(&cap[3], 10).unwrap() - 1;
            // println!("qty: {} | from: {} | to: {}", qty_moved, move_from, move_to);
            // for part 2, the 9001 moves all at once, need a deque to act as move buffer
            let mut move_buffer: VecDeque<u8> = VecDeque::new();
            for op in 0..qty_moved {
                let tmp = crate_stacks[move_from].pop_front().unwrap();
                move_buffer.push_back(tmp);
            }

            for _elems in 0..move_buffer.len() {
                crate_stacks[move_to].push_front(move_buffer.pop_back().unwrap());
            }
        }
    }
    let mut top_crates = String::new();
    for mut stack in crate_stacks {
        println!("{:?}", stack);
        top_crates += &String::from_utf8(vec![stack.pop_front().unwrap()]).unwrap();
    }

    println!("{}", top_crates);

}

fn day6(input_file: String) {
    let file_contents = io::read_file(Path::new(&input_file.to_owned()));
    for i in 13..file_contents.len() {
        let tmp_set: HashSet<u8> = HashSet::from_iter(file_contents[(i-13)..(i+1)].to_vec());
        if tmp_set.len() == 14 {
            println!("First unique 14-byte signal at char: {}", i+1);
            return;
        }
    }
}
use std::fs::File;
use std::io::prelude::*;

const VERBOSE: bool = false;

fn operate_on_tape(position: usize, tape: &mut Vec<usize>) {
    let opcode = tape[position];
    match opcode {
        1 | 2 => {
            let pos1 = tape[position + 1];
            let pos2 = tape[position + 2];
            let arg1 = tape[pos1];
            let arg2 = tape[pos2];
            let output_position = tape[position + 3];

            tape[output_position] = match opcode {
                1 => arg1 + arg2,
                2 => arg1 * arg2,
                _ => panic!("Should be covered by parent match"),
            }
        }
        _ => panic!("What have you done?"),
    }
}

fn play_through_tape(program: &mut Vec<usize>) -> String {
    for position in (0..program.len()).step_by(4) {
        if program[position] == 99 {
            break;
        }

        operate_on_tape(position, program);
    }

    if VERBOSE {
        println!("Item at position 0: {}", program[0]);
    }

    program
        .clone()
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn parse_program(input: &str) -> Vec<usize> {
    input.split(",").flat_map(|c| c.parse::<usize>()).collect()
}

fn tests() {
    assert_eq!(
        play_through_tape(&mut parse_program("1,0,0,0,99")),
        "2,0,0,0,99"
    );

    assert_eq!(
        play_through_tape(&mut parse_program("2,3,0,3,99")),
        "2,3,0,6,99"
    );
}

fn part1(initial_program: &mut Vec<usize>) {
    // Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?

    initial_program[1] = 12;
    initial_program[2] = 2;

    println!("Part 1");

    play_through_tape(initial_program);
}

fn part2(initial_program: Vec<usize>) {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut program_instance = initial_program.clone();

            program_instance[1] = i;
            program_instance[2] = j;
            let result = play_through_tape(&mut program_instance)
                .split(",")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            if result == 19690720 {
                println!("Part 2");

                let noun = i;
                let verb = j;

                println!("Result: {}", 100 * noun + verb);

                return;
            }
        }
    }
}

pub fn challenge() -> Result<(), std::io::Error> {
    let mut file = File::open("./data/day-2")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    tests();
    let initial_program = parse_program(&contents);

    println!("\nDay 2");
    part1(&mut initial_program.clone());
    part2(initial_program);

    Ok(())
}

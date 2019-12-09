/* It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679). */
fn digits_never_decrease(s: &str) -> bool {
    s.as_bytes().windows(2).all(|pair| match pair {
        [a, b] => a <= b,
        _ => panic!("Unable to build window for {:?}", pair),
    })
}

fn adjacent_twins(s: &str) -> bool {
    s.as_bytes().windows(2).any(|pair| match pair {
        [a, b] => a == b,
        _ => panic!("Unable to build window for {:?}", pair),
    })
}

fn six_digits_long(s: &str) -> bool {
    s.len() == 6
}

fn is_potential_password(n: u32) -> bool {
    let predicates: Vec<&dyn Fn(&str) -> bool> =
        vec![&digits_never_decrease, &adjacent_twins, &six_digits_long];

    let as_string = n.to_string();

    predicates.iter().all(|f| f(&as_string))
}

/*
112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
 */
fn unique_doubles(s: &str) -> bool {
    let mut curr: Option<u8> = None;
    let mut curr_count = 0;

    for byte in s.as_bytes() {
        if Some(*byte) == curr {
            curr_count += 1;
        } else {
            if curr_count == 2 {
                return true;
            }

            curr = Some(*byte);
            curr_count = 1;
            continue;
        }
    }

    false
}

fn is_potential_password_2(n: u32) -> bool {
    let predicates: Vec<&dyn Fn(&str) -> bool> = vec![
        &digits_never_decrease,
        &adjacent_twins,
        &six_digits_long,
        &unique_doubles,
    ];

    let as_string = n.to_string();

    predicates.iter().all(|f| f(&as_string))
}

fn build_range(input: &str) -> (u32, u32) {
    let parsed_input = input
        .split('-')
        .flat_map(|n| n.parse::<u32>())
        .collect::<Vec<u32>>();

    let [&start, &end] = match parsed_input.as_slice() {
        [start, end] => [start, end],
        x => panic!("Invalid split for {:?}", x),
    };

    (start, end)
}

fn part_1(input: &str) {
    println!("Part 1");

    let (start, end) = build_range(input);

    println!(
        "Potential passwords {}",
        (start..end).filter(|i| is_potential_password(*i)).count()
    );
}

fn part_2(input: &str) {
    println!("Part 2");

    let (start, end) = build_range(input);

    println!(
        "{:?}",
        (start..end).filter(|i| is_potential_password_2(*i)).count()
    );
}

pub fn challenge() -> Result<(), std::io::Error> {
    println!("Day 4");

    let input = "168630-718098";

    part_1(input);
    part_2(input);

    Ok(())
}

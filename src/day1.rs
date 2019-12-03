use std::fs::File;
use std::io::prelude::*;

// part 1
fn fuel_required(mass: u64) -> Option<u64> {
    // Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
    (mass / 3).checked_sub(2)
}

struct FuelBasedReqs {
    mass: u64,
}

impl Iterator for FuelBasedReqs {
    // we will be counting with usize
    type Item = u64;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        let next_mass = fuel_required(self.mass);

        self.mass = next_mass.unwrap_or(0);

        next_mass

        // Check to see if we've finished counting or not.
    }
}

fn fuel_for_fuel_required(mass: u64) -> u64 {
    FuelBasedReqs { mass: mass }.sum()
}

pub fn challenge() -> Result<(), std::io::Error> {
    let mut file = File::open("./data/day-1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let fuel_requried_part1: u64 = contents
        .lines()
        .flat_map(|c| c.parse::<u64>())
        .flat_map(fuel_required)
        .sum();

    println!("Day 1");

    println!("part 1 {}", fuel_requried_part1);

    let fuel_requried_part2: u64 = contents
        .lines()
        .flat_map(|c| c.parse::<u64>())
        .map(fuel_for_fuel_required)
        .sum();

    assert_eq!(fuel_for_fuel_required(14), 2);
    assert_eq!(fuel_for_fuel_required(1969), 966);
    assert_eq!(fuel_for_fuel_required(100756), 50346);

    println!("part 2 {}", fuel_requried_part2);
    Ok(())
}

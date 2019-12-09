mod day1;
mod day2;
mod day3;
mod day4;
mod day6;

fn main() -> Result<(), std::io::Error> {
    let challenges: Vec<&dyn Fn() -> Result<(), std::io::Error>> = vec![
        &day6::challenge,
        &day4::challenge,
        &day3::challenge,
        &day2::challenge,
        &day1::challenge,
    ];

    for c in challenges {
        c()?;
    }

    Ok(())
}

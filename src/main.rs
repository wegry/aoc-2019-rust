mod day1;
mod day2;

fn main() -> Result<(), std::io::Error> {
    day1::challenge()?;
    day2::challenge()
}

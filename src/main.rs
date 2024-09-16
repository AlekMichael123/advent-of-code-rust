use advent_of_code_rust::use_day;

fn main() {
    println!("Hello, world!");
    match use_day(2022, 1, "I LOVE RUSTTTT!!! ... PFFTT!! NOTTTTTT!!!") {
        Ok(_) => (),
        Err(err) => println!("ERROR: {err}"),
    }
}

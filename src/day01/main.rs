use std::fs::File;
use std::io::*;
use std::error;
use std::result::Result;
use std::env;

fn process_fuel(num: i64) -> i64 {
    let out = (num / 3) - 2;
    if out < 0 {
        return 0;
    }
    out
}

fn fuel_for_fuel(num: i64) -> i64 {
    let mut out: i64 = 0;
    let mut fuel: i64 = process_fuel(num);
    while fuel > 0 {
        let extra = fuel;
        out += fuel;
        fuel = process_fuel(extra);
    }
    out
}

fn process_file(file: &str) -> Result<i64, Box<dyn error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut out: i64 = 0;
    for line in reader.lines() {
        let num: i64 = line?.trim().parse()?;
        out += fuel_for_fuel(num);
    }
    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = process_file(&args[1]);

    match result {
        Ok(out) => {
            println!("Output is {}", out);
        },
        Err(error) => {
            println!("We encountered an error {}", error);
        },
    }
}

/*
    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
*/

#[test]
fn example() {
    assert_eq!(process_fuel(12), 2);
    assert_eq!(process_fuel(14), 2);
    assert_eq!(process_fuel(1969), 654);
    assert_eq!(process_fuel(100756), 33583);

    assert_eq!(fuel_for_fuel(14), 2);
    assert_eq!(fuel_for_fuel(1969), 966);
    assert_eq!(fuel_for_fuel(100756), 50346);
}
use std::env;

fn validate_no_decrease(input: i64) -> bool {
    let mut rest = input;
    let mut prevdigit = input % 10;

    while rest > 0 {
        let digit = rest % 10;
        if digit > prevdigit {
            return false;
        }

        prevdigit = digit;
        rest = rest / 10;
    }

    true
}

fn validate_double_number(input: i64) -> bool {
    let mut rest = input / 10;
    let mut prevdigit = input % 10;

    while rest > 0 {
        let digit = rest % 10;
        if digit == prevdigit {
            return true;
        }

        prevdigit = digit;
        rest = rest / 10;
    }

    return false;
}

fn validate(input: i64) -> bool {
    if !validate_no_decrease(input) {
        return false;
    }
    if !validate_double_number(input) {
        return false;
    }
    true
}

fn process(min: i64, max: i64) -> i64 {
    let mut out: i64 = 0;
    for i in min..max {
        if validate(i) {
            out += 1;
        }
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let min: i64 = args[1].parse().unwrap();
    let max: i64 = args[2].parse().unwrap();

    let possible_answers = process(min, max);

    println!("Possible answers {}", possible_answers);
}

#[test]
fn test_validate() {
    assert!(validate_no_decrease(111111));
    assert!(validate_no_decrease(135679));
    assert!(validate_no_decrease(111123));

    assert!(validate(111111));
    assert_eq!(false, validate(223450));
    assert_eq!(false, validate(123789));
}
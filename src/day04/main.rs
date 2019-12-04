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
    let mut rest = input;

    while rest > 0 {
        let digit = rest % 10;
        let double = digit * 11;
        let triple = digit * 111;
        if (rest % 1000) == triple {
            // when we confirmed the digit appears at least 3 times in a row, we skip over all the instance of this digit
            while (rest % 10) == digit {
                rest = rest / 10;
            }
            continue;
        }
        if (rest % 100) == double {
            return true;
        }

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

    assert!(validate_double_number(123455));
    assert_eq!(false, validate_double_number(123555));
    assert!(validate_double_number(113333));

    assert_eq!(false, validate(223450));
    assert_eq!(false, validate(123789));

    assert!(validate(112233));
    assert_eq!(false, validate(123444));
    assert!(validate(111122));
}
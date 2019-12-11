use std::fs::File;
use std::io::*;
use std::error;
use std::result::Result;
use std::env;

fn process_line(line: &str) -> Result<Vec<i64>, Box<dyn error::Error>> {
    let mut out = Vec::new();
    let parser: Vec<&str> = line.split(',').collect();
    for opcode in parser {
        let i: i64 = opcode.parse()?;
        out.push(i);
    }
    Ok(out)
}

fn is_immediate_mode(opcode: i64, param: i64) -> bool {
    let mut modu = 10;

    // there's probably a better way to do this, but I'm currently in a plane so no access to
    // the internet
    for _ in 0..param {
        modu *= 10;
    }

    let rest = opcode / modu;

    (rest % 10) == 1
}

#[test]
fn test_immediate_mode() {
    assert_eq!(is_immediate_mode(1002, 1), false);
    assert_eq!(is_immediate_mode(1002, 2), true);
    assert_eq!(is_immediate_mode(1002, 3), false);
    assert_eq!(is_immediate_mode(1002, 4), false);
}

fn process_opcodes(opcodes: Vec<i64>, input: i64) -> Vec<i64> {
    let mut opcodes = opcodes.clone();
    let mut output = Vec::new();
    let mut pos: usize = 0;
    loop {
        let mode = opcodes[pos];
        let opcode = opcodes[pos] % 100;
        match opcode {
            1 => {
                let modpos = opcodes[pos+3];
                let mut val1: i64 = opcodes[pos+1];
                let mut val2: i64 = opcodes[pos+2];
                if !is_immediate_mode(mode, 1) {
                    val1 = opcodes[val1 as usize];
                }
                if !is_immediate_mode(mode, 2) {
                    val2 = opcodes[val2 as usize];
                }
                opcodes[modpos as usize] = val1 + val2;
                pos += 4;
            },
            2 => {
                let modpos = opcodes[pos+3];
                let mut val1 = opcodes[pos+1];
                let mut val2 = opcodes[pos+2];
                if !is_immediate_mode(mode, 1) {
                    val1 = opcodes[val1 as usize];
                }
                if !is_immediate_mode(mode, 2) {
                    val2 = opcodes[val2 as usize];
                }
                opcodes[modpos as usize] = val1 * val2;
                pos += 4;
            },
            3 => {
                let modpos = opcodes[pos+1];
                opcodes[modpos as usize] = input;
                pos += 2;
            },
            4 => {
                let modpos = opcodes[pos+1];
                output.push(opcodes[modpos as usize]);
                pos += 2;
            },
            99 => {
                return output;
            },
            _ => {
                println!("Unknown opcode {}", opcodes[pos]);
            },
        }
    }
}

fn process_file(file: &str) -> Result<Vec<i64>, Box<dyn error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        return process_line(&line?);
    }
    panic!("no intcodes?");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opcodes = process_file(&args[1]);
    let input: i64 = args[2].parse().unwrap();

    match opcodes {
        Ok(intcodes) => {
            let out = process_opcodes(intcodes.clone(), input);
            for i in out {
                println!("Output is {}", i);
            }
        },
        Err(error) => {
            println!("We encountered an error {}", error);
        },
    }
}

/*

    1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
    2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
    2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
    1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.

*/

#[test]
fn example() {
    let res = process_line("1,0,0,0,99");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), [1,0,0,0,99]);

    assert_eq!(process_opcodes(vec![1,0,0,0,99], 0), []);
    assert_eq!(process_opcodes(vec![2,3,0,3,99], 0), []);
    assert_eq!(process_opcodes(vec![2,4,4,5,99,0], 0), []);
    assert_eq!(process_opcodes(vec![1,1,1,4,99,5,6,0,99], 0), []);

    assert_eq!(process_opcodes(vec![3, 0, 4, 0, 99], 1337), [1337]);

    assert_eq!(process_opcodes(vec![1002, 4, 3, 4, 33], 0), []);
    assert_eq!(process_opcodes(vec![1101, 100, -1, 4, 0], 0), []);
}

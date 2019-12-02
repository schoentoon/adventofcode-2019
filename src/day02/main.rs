use std::fs::File;
use std::io::*;
use std::error;
use std::result::Result;
use std::env;

fn process_line(line: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut out = Vec::new();
    let parser: Vec<&str> = line.split(',').collect();
    for opcode in parser {
        let i: usize = opcode.parse()?;
        out.push(i);
    }
    Ok(out)
}

fn process_opcodes(opcodes: Vec<usize>) -> Vec<usize> {
    let mut opcodes = opcodes.clone();
    let mut pos: usize = 0;
    loop {
        match opcodes[pos] {
            1 => {
                let modpos = opcodes[pos+3];
                let pos1 = opcodes[pos+1];
                let pos2 = opcodes[pos+2];
                opcodes[modpos] = opcodes[pos1] + opcodes[pos2];
            },
            2 => {
                let modpos = opcodes[pos+3];
                let pos1 = opcodes[pos+1];
                let pos2 = opcodes[pos+2];
                opcodes[modpos] = opcodes[pos1] * opcodes[pos2];
            },
            99 => {
                return opcodes;
            },
            _ => {
                println!("Unknown opcode {}", opcodes[pos]);
            },
        }
        pos += 4;
    }
}

fn bruteforce_part2(opcodes: Vec<usize>) -> usize {
    for noun in 1..99 {
        for verb in 1..99 {
            let mut opcodes = opcodes.clone();
            opcodes[1] = noun;
            opcodes[2] = verb;
            if process_opcodes(opcodes)[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}

fn process_file(file: &str) -> Result<Vec<usize>, Box<dyn error::Error>> {
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

    match opcodes {
        Ok(intcodes) => {
            let out = process_opcodes(intcodes.clone());
            println!("Output is {}", out[0]);

            let result2 = bruteforce_part2(intcodes);
            println!("Output of part 2 is {}", result2);
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

    assert_eq!(process_opcodes(vec![1,0,0,0,99]), [2,0,0,0,99]);
    assert_eq!(process_opcodes(vec![2,3,0,3,99]), [2,3,0,6,99]);
    assert_eq!(process_opcodes(vec![2,4,4,5,99,0]), [2,4,4,5,99,9801]);
    assert_eq!(process_opcodes(vec![1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99]);
}
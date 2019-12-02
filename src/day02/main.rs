use std::fs::File;
use std::io::*;
use std::error;
use std::result::Result;
use std::env;

fn process_line(line: &str) -> Result<Vec<i32>, Box<dyn error::Error>> {
    let mut out = Vec::new();
    let parser: Vec<&str> = line.split(',').collect();
    for opcode in parser {
        let i: i32 = opcode.parse()?;
        out.push(i);
    }
    Ok(out)
}

fn process_opcodes(mut opcodes: Vec<i32>) -> Vec<i32> {
    let mut pos: usize = 0;
    loop {
        match opcodes[pos] {
            1 => {
                let modpos = opcodes[pos+3] as usize;
                let pos1 = opcodes[pos+1] as usize;
                let pos2 = opcodes[pos+2] as usize;
                opcodes[modpos] = opcodes[pos1] + opcodes[pos2];
            },
            2 => {
                let modpos = opcodes[pos+3] as usize;
                let pos1 = opcodes[pos+1] as usize;
                let pos2 = opcodes[pos+2] as usize;
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

fn process_file(file: &str) -> Result<i32, Box<dyn error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let opcodes = process_line(&line?)?;
        let out = process_opcodes(opcodes.clone());
        return Ok(out[0]);
    }
    Ok(0)
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
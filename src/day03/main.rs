use std::env;
use std::error;
use std::result::Result;

fn process_path(input: &str) -> Result<Vec<(i16, i16)>, Box<dyn error::Error>> {
    let mut out = Vec::new();
    out.push((0, 0));

    let parser: Vec<&str> = input.split(',').collect();
    for operation in parser {
        let pos = out.last().unwrap().clone();
        let amount: i16 = operation.get(1..).unwrap().parse()?;
        match operation.get(0..1) {
            Some("R") => {
                for i in 1..amount + 1 {
                    out.push((pos.0 + i, pos.1));
                }
            },
            Some("U") => {
                for i in 1..amount + 1 {
                    out.push((pos.0, pos.1 + i));
                }
            },
            Some("L") => {
                for i in 1..amount + 1 {
                    out.push((pos.0 - i, pos.1));
                }
            },
            Some("D") => {
                for i in 1..amount + 1 {
                    out.push((pos.0, pos.1 - i));
                }
            },
            _ => {
                // TODO We should actually error here..
            },
        }
    }

    Ok(out)
}

fn cross_points(in1: &Vec<(i16, i16)>, in2: &Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    let mut out = Vec::new();

    for point in in1 {
        if point.0 == 0 && point.1 == 0 {
            continue
        }
        for pos in in2 {
            if point.0 == pos.0 && point.1 == pos.1 {
                out.push(point.clone());
                break;
            }
        }
    }

    return out
}

fn manhatten_distance(to: &(i16, i16)) -> i16 {
    let mut x = to.0;
    if x < 0 {
        x = x / -1
    }
    let mut y = to.1;
    if y < 0 {
        y = y / -1
    }
    x + y
}

fn lowest_manhatten_distance(crosspoints: &Vec<(i16, i16)>) -> i16 {
    let mut lowest = i16::max_value();
    for step in crosspoints {
        let dis = manhatten_distance(step);
        println!("distance for step {} {} is {}", step.0, step.1, dis);
        if dis < lowest {
            lowest = dis;
        }
    }
    lowest
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path1 = process_path(&args[1]);
    let path2 = process_path(&args[2]);

    let crosspoints = cross_points(&path1.unwrap(), &path2.unwrap());
    
    let lowest = lowest_manhatten_distance(&crosspoints);
    println!("Lowest distance ended up being {}", lowest);
}

#[test]
fn test_pathing() {
    let res = process_path("R8,U5,L5,D3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (7, 5), (6, 5), (5, 5), (4, 5), (3, 5), (3, 4), (3, 3), (3, 2)]);
}

#[test]
fn example1() {
    let path1 = process_path("R8,U5,L5,D3");
    let path2 = process_path("U7,R6,D4,L4");
    assert!(path1.is_ok());
    assert!(path2.is_ok());

    let res = cross_points(&path1.unwrap(), &path2.unwrap());

    assert_eq!(res, [(6, 5), (3, 3)]);

    assert_eq!(lowest_manhatten_distance(&res), 6);
}

#[test]
fn example2() {
    let path1 = process_path("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let path2 = process_path("U62,R66,U55,R34,D71,R55,D58,R83");
    assert!(path1.is_ok());
    assert!(path2.is_ok());

    let res = cross_points(&path1.unwrap(), &path2.unwrap());

    assert_eq!(res, [(158, -12), (146, 46), (155, 4), (155, 11)]);

    assert_eq!(lowest_manhatten_distance(&res), 159);
}

#[test]
fn example3() {
    let path1 = process_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let path2 = process_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert!(path1.is_ok());
    assert!(path2.is_ok());

    let res = cross_points(&path1.unwrap(), &path2.unwrap());

    assert_eq!(res, [(107, 47), (124, 11), (157, 18), (107, 71), (107, 51)]);

    assert_eq!(lowest_manhatten_distance(&res), 135);
}
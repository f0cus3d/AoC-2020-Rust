use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::prelude::*;

fn main() {
    let mut expense_report: Vec<i64> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(expense_item) = line {
               let value  = expense_item.parse::<i64>().unwrap();
               expense_report.push(value);
            }           
        }
    }
    
    let mut rng = thread_rng();

    loop {
        let v1 = expense_report.choose(&mut rng).unwrap();
        let v2 = expense_report.choose(&mut rng).unwrap();
        let v3 = expense_report.choose(&mut rng).unwrap();

        let sum = v1 + v2 + v3;

        if sum == 2020 {
            println!("{:?} + {:?} + {:?} = {:?}",v1,v2,v3,sum);
            println!("Answer: {:?}", v1*v2*v3);
            break
        } else {
            continue
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
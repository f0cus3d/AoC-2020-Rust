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

    for n in 0..expense_report.len(){
        let mut x = 0;
        while x < expense_report.len() {
            let sum = expense_report[n] + expense_report[x];

            if sum == 2020 {
                println!("{:?} + {:?} = {:?}", &expense_report[n], &expense_report[x], &sum);
                println!("ANSWER!: {:?}", expense_report[n] * expense_report[x]);
            }
            x += 1
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
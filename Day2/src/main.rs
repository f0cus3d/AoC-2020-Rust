use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct PassData{
    complex_max: u8,
    complex_min: u8,
    complex_char: String,
    pass: String,
}

fn main() {

    let mut compliance_count = 0;
    let mut pass_map = HashMap::new();
    let mut uid: u64 = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_item) = line {
            let data = line_item.split_whitespace();

            let mut complexity_min_max: Vec<u8> = Vec::new();
            let mut complexity_char: Vec<String> = Vec::new();
            let mut password: Vec<String> = Vec::new();


            for data_item in data {
                if data_item.find("-") != None {
                    let complex_num_data = data_item.split("-");
                    for item in complex_num_data{
                       let num = item.parse::<u8>().unwrap();
                       complexity_min_max.push(num);
                    }
                } else if data_item.find(":") != None {
                    complexity_char.push(data_item.replace(":","").to_string());
                } else {
                    password.push(data_item.to_string());
                }
            }

            let password_data = PassData{
                complex_max: complexity_min_max.pop().unwrap(),
                complex_min: complexity_min_max.pop().unwrap(),
                complex_char: complexity_char.pop().unwrap(),
                pass: password.pop().unwrap(),

            };

            pass_map.insert(uid, password_data);

            uid += 1;


            }
        }
    }


    for (_, complex_data) in &pass_map {
        let pass_count = &complex_data.pass.matches(&complex_data.complex_char)
                        .count()
                        .to_string()
                        .parse::<u8>()
                        .unwrap();
        let min =  &complex_data.complex_min;
        let max =  &complex_data.complex_max;

        if !(pass_count < min) && !(pass_count > max){
            compliance_count += 1
        }

    }

    println!("Passowrds Compliant: {:?}", compliance_count);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
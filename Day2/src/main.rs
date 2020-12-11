use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// Debug for reasons....
#[derive(Debug)]
struct PassData{
    complex_max: u8,
    complex_min: u8,
    complex_char: String,
    pass: String,
}

fn main() {

    // Count all the things and make that collection
    let mut p1_compliance_count = 0;
    let mut p2_compliance_count = 0;
    let mut pass_map = HashMap::new();
    let mut uid: u64 = 0;

    // Read and process all the things
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(line_item) = line {
            let data = line_item.split_whitespace();
            
            // This is probably not needed but hey it works.
            let mut complexity_min_max: Vec<u8> = Vec::new();
            let mut complexity_char: Vec<String> = Vec::new();
            let mut password: Vec<String> = Vec::new();

            // Another unintuative way at parsing text....
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
            // Way more crafty than it needs to be but again it works.
            let password_data = PassData{
                complex_max: complexity_min_max.pop().unwrap(),
                complex_min: complexity_min_max.pop().unwrap(),
                complex_char: complexity_char.pop().unwrap(),
                pass: password.pop().unwrap(),

            };

            // Store based on UID, which will not be used later.
            pass_map.insert(uid, password_data);

            uid += 1;


            }
        }
    }

    // Check data based on the problem of part 1.
    for (_, complex_data) in &pass_map {  // Long hair don't care about the UID
        let pass_count = &complex_data.pass.matches(&complex_data.complex_char)
                        .count()
                        .to_string()
                        .parse::<u8>()
                        .unwrap();
        let min =  &complex_data.complex_min;
        let max =  &complex_data.complex_max;

        if !(pass_count < min) && !(pass_count > max){
            p1_compliance_count += 1
        }

    }
    // Out something of use to the term.
    println!("Part 1: Passowrds Compliant: {:?}", p1_compliance_count);

        // Part 2 
        // Mix and Max are positions that must contain the char therfor we need to find the occurance of the char
        // WUTANG!!!!!
        // Can't start with index zero so whatever we get will need to -1 the min or max

        for (_, complex_data) in &pass_map {
            let raw_pass: Vec<_> = complex_data.pass
                                    .match_indices(&complex_data.complex_char)
                                    .collect();
            let min =  &complex_data.complex_min - 1;
            let max =  &complex_data.complex_max - 1;

            if raw_pass.len() < max as usize {
                continue
            }

            if raw_pass[min as usize].1 == &complex_data.complex_char || raw_pass[max as usize].1 == &complex_data.complex_char {
                p2_compliance_count += 1
            }
        }
        // Attempt 1 was 467 num was too low.....
        println!("Part 2: Passowrds Compliant: {:?}", p2_compliance_count);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
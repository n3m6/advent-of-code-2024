use utils;
use regex::Regex;
mod task2;

fn main() {
    println!("Reading file...");

    let contents = utils::read_lines("./data/day3.txt");

    let pattern = r"mul\(\d+,\d+\)";
    let regex = Regex::new(pattern).unwrap();

    let number_pattern = r"\d+";
    let num_regex = Regex::new(number_pattern).unwrap();


    let mut matches: Vec<(u32, u32)> = vec![];

    for line in contents.iter() {
        let values: Vec<&str> = regex.find_iter(line).map(|m| m.as_str()).collect();
        for val in values.iter() {
            println!("{}", val);

            // get the two numbers
            let nums: Vec<&str> = num_regex.find_iter(val).map(|m| m.as_str()).collect();
            matches.push((nums[0].parse().unwrap(), nums[1].parse().unwrap()));
        }
    }

    let mut total_multiplied: u32 = 0;

    for values in matches.iter() {
        println!("{} * {} = {}", values.0, values.1, values.0 * values.1);
        total_multiplied = total_multiplied + (values.0 * values.1);
    }
    
    println!("Total = {}", total_multiplied);

    println!("------------------------");

    task2::task2();
}

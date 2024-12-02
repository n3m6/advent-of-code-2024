use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    return result;
}

fn main() {
    println!("Reading file...");
    let contents = read_lines("./data/day1.txt");
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();

    for line in contents.iter() {
        let values: Vec<&str> = line.split_whitespace().collect();
        println!("{} -> {}", values[0], values[1]);
        arr1.push(values[0].parse::<i32>().unwrap());
        arr2.push(values[1].parse::<i32>().unwrap());
    }

    arr1.sort();
    arr2.sort();

    let length = arr1.len();
    println!("array length = {}", length);
    let mut total = 0;

    for i in 0..length {
        let result = arr1[i] - arr2[i];
        total += result.abs();
    }

    println!("total = {}", total);

    let mut freq_count = 0;

    // Frequency calculation
    for i in 0..length {
        let value = arr1[i];
        let occurences: i32 = arr2.iter().filter(|&n| *n == value).count() as i32;
        println!("{} -> {} = {}", arr1[i], arr2[i], value * occurences);
        

        freq_count += value * occurences;
    }

    println!("frequency = {}", freq_count);
}

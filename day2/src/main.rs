use utils;

fn main() {
    println!("Reading file...");
    let contents = utils::read_lines("./data/day2.txt");

    let mut total_safe = 0;

    for line in contents.iter() {
        let values: Vec<&str> = line.split_whitespace().collect();

        let length = values.len();
        
        let mut increasing = false;
        let mut safe = true;

        let first: i32 = values[0].parse().unwrap();
        let second: i32 = values[1].parse().unwrap();
        if second > first { increasing = true; }

        let mut str: String = format!("{}, ", values[0]);
        let mut unsafe_counter = 0;

        for i in 1..length {
            str = str + format!("{}, ", values[i]).as_str();

            let val1: i32 = values[i].parse().unwrap();
            let val2: i32 = values[i-1].parse().unwrap();


            if !increasing && val1 > val2 {
                safe = false; 
                unsafe_counter = unsafe_counter + 1;
                continue;
            }
            if increasing && val2 > val1 {
                safe = false; 
                unsafe_counter = unsafe_counter + 1;
                continue;
            }
            if (val1 - val2).abs() > 3 {
                safe = false; 
                unsafe_counter = unsafe_counter + 1;
                continue;
            }
            if val1 == val2 { 
                safe = false; 
                unsafe_counter = unsafe_counter + 1;
                continue;
            }
        }

        if safe == true { 
            println!("Safe");
        } else {
            println!("{}", str);
            println!("Unsafe {}", unsafe_counter);
        }

        if safe == true || unsafe_counter == 1 { total_safe = total_safe + 1; }
    }

    println!("total safe {}", total_safe);
}

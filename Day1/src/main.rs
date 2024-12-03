use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{absolute, Path};

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("G:/Repos/AdventOfCode2024/Day1/src/input.txt") {

        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();

        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            for element in line.split_whitespace() {
                let int_value = element.parse::<i32>().unwrap();

                // For problem 1
                if (list1.len() > list2.len()) {
                    list2.push(int_value);

                    // For problem 2
                    if (hash_map.contains_key(&int_value)) {
                        *hash_map.get_mut(&int_value).unwrap() += 1;
                    }
                    else {
                        hash_map.insert(int_value, 1);
                    }
                }
                else {
                    list1.push(int_value);
                }
            }
        }

        list1.sort();
        list2.sort();

        let mut result1: i32 = 0;
        let mut result2: i32 = 0;


        for i in 0..list1.len() {
            // Problem 1
            result1 += (list1[i] - list2[i]).abs();

            // Problem 2
            result2 += list1[i] * hash_map.get(&list1[i]).unwrap_or(&0);
        }

        println!("{}", result1);
        println!("{}", result2);
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
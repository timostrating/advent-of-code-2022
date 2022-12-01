use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/day01.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    let mut calories = Vec::<i32>::new();

    let mut index: usize = 0;
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        if line == "" {
            index += 1;
        } else {
            // println!("{:?}", calories.get(index));
            if calories.get(index) == None {
                calories.push(line.parse::<i32>().unwrap());
            } else {
                calories[index] = calories.get(index).unwrap_or(&0i32) + (line.parse::<i32>().unwrap());
            }
        }
    }

    println!("{:?}", calories);
    println!("{:?}", index);

    let mut maximum = 0;
    for x in calories {
        if x > maximum {
            maximum = x;
        }
    }

    println!("{:?}", maximum)

}

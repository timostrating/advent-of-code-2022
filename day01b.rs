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

    let mut maximum1 = 0;
    let mut maximum2 = 0;
    let mut maximum3 = 0;
    for x in calories {
        if x > maximum1 {
            maximum3 = maximum2;
            maximum2 = maximum1;
            maximum1 = x;
        }

        else if x > maximum2 {
            maximum3 = maximum2;
            maximum2 = x;
        }

        else if x > maximum3 {
            maximum3 = x;
        }
    }

    println!("{:?}", maximum1 + maximum2 + maximum3)

}

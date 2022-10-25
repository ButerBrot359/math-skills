use std::io::{self, Read};
use std::env;
use std::fs::{File};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return
    }

    let whole_file = filename_to_string(&args[1]).expect("No such file or directory");

    let mut number_array = get_number_by_line(&whole_file);
    
    let average = find_average(&number_array).round();

    let variance = find_variance(&number_array).round();
    
    let standard_deviation = find_standard_deviation(&number_array).round();

    println!("Average: {}\nVariance: {}\nStandard Deviation: {}", average, variance, standard_deviation);

    let median = find_median(&mut number_array);

    println!("Median: {}", median);
}

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn get_number_by_line(s: &str) -> Vec<f32> {
    s.lines().map(|line| {
        line.parse::<f32>().unwrap()
    }).collect()
}

fn find_average(number_array: &Vec<f32>) -> f32 {
    let arr_len = number_array.len() as f32;

    let sum = {
        let mut acc: f32 = 0.0;
        for num in number_array.iter() {
            acc += num;
        };
        acc
    };
    
    sum/arr_len
}

fn find_median(number_array: &mut Vec<f32>) -> i32 {
    let arr_len = number_array.len() as i32;

    number_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    if arr_len%2 == 0 {
        let center_indexes = (arr_len/2, arr_len/2 + 1);

        let first_arg = number_array[center_indexes.0 as usize] as f32;

        let second_arg = number_array[center_indexes.1 as usize] as f32;
        
        ((first_arg + second_arg)/2.0) as i32
    } else {
        let center_index = (arr_len-1)/2;
        
        number_array[center_index as usize] as i32
    }

}

fn find_variance(number_array: &Vec<f32>) -> f32 {
    let arr_len = number_array.len() as f32;

    let average = find_average(&number_array);
    
    let numerator: f32 = number_array
        .iter()
        .map(|num| (num - average).powf(2.0))
        .collect::<Vec<f32>>()
        .iter()
        .sum();
    
    numerator/(arr_len-1.0)
}

fn find_standard_deviation(number_array: &Vec<f32>) -> f32{
    let variance = find_variance(number_array);

    variance.sqrt()
}
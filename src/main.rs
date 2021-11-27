mod sorters;
mod utils;

use std::time::{ Instant };
use utils::{ generate_random_arr, Logger };
use std::collections::{ HashMap };
use sorters::*;

fn run_benchmark(method: for<'r> fn(&'r mut std::vec::Vec<i32>), length_steps: u32, max_duration: f64, logger: &mut Logger) -> HashMap<u32, f64> {
    let mut timings = HashMap::new();

    let mut length = length_steps;
    loop {
        let mut arr: Vec<i32> = generate_random_arr(length);

        let now = Instant::now();
        method(&mut arr);
        let elapsed = now.elapsed();

        timings.insert(length, elapsed.as_secs_f64());
        logger.log(format!("LENGTH={};TIME={}", length, elapsed.as_secs_f64()));
        if elapsed.as_secs_f64() > max_duration {
            break;
        }
        length = length + length_steps;
    }

    timings
}

fn main() {
    let mut methods: HashMap<&str, (for<'r> fn(&'r mut std::vec::Vec<i32>), u32)> = HashMap::new();

    methods.insert(
        "bubble_sort",
        (bubble_sort, 1000),
    );

    methods.insert(
        "boggo_sort",
        (boggo_sort, 2),
    );

    methods.insert(
        "insertion_sort",
        (insertion_sort, 1000),
    );

    let method_name = "boggo_sort";
    let method = methods.get(method_name).unwrap();

    let mut logger = Logger::new(format!("logs/{}_benchmark.log", method_name), true);
    run_benchmark(method.0, method.1, 1.0, &mut logger);

    // let mut arr = generate_random_arr(10);
    // insertion_sort(&mut arr);
    // println!("{:?}", arr);
}

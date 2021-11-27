mod sorters;
mod utils;

use std::time::{ Instant };
use utils::{ generate_random_arr, Logger };
use std::collections::{ HashMap };
use sorters::*;

const MAX_DURATION: f64 = 1.0;

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

fn run_all_benchmarks() {
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

    methods.insert(
        "quick_sort",
        (quick_sort, 10_000),
    );

    for method_name in methods.keys() {
        let method = methods.get(method_name).unwrap();

        let mut logger = Logger::new(format!("benchmarks/{}_benchmark.log", method_name), true);
        run_benchmark(method.0, method.1, MAX_DURATION, &mut logger);
    }
}

fn main() {
    run_all_benchmarks();

    // let mut logger = Logger::new(String::from("logs/quicksort_benchmark.log"), true);

    // run_benchmark(quick_sort, 100_000, MAX_DURATION, &mut logger);
    
    // let mut arr = generate_random_arr(10);
    // quick_sort(&mut arr);
    // println!("{:?}", arr);
}

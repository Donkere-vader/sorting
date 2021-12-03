mod sorters;
mod utils;
mod consts;

use std::time::{ Instant, SystemTime };
use utils::{ generate_random_arr, Logger };
use std::collections::{ HashMap };
use sorters::*;
use consts::{ MAX_DURATION };


fn run_benchmark(method: for<'r> fn(&'r mut std::vec::Vec<i32>), length_steps: u32, max_duration: f64, logger: &mut Logger) -> HashMap<u32, f64> {
    let mut timings = HashMap::new();

    let mut length = length_steps.max(2);
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
    let now = SystemTime::now();
    let mut bench_logger = Logger::new(format!("logs/benchmarks_{:?}.log", now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()), true);
    let mut methods: HashMap<&str, (for<'r> fn(&'r mut std::vec::Vec<i32>), u32)> = HashMap::new();

    methods.insert(
        "bubble_sort",
        (bubble_sort, 1000),
    );

    methods.insert(
        "boggo_sort",
        (boggo_sort, 1),
    );

    methods.insert(
        "insertion_sort",
        (insertion_sort, 1000),
    );

    methods.insert(
        "quick_sort",
        (quick_sort, 100_000),
    );

    methods.insert(
        "threaded_quick_sort",
        (threaded_quick_sort, 100_000),
    );

    methods.insert(
        "merge_sort",
        (merge_sort, 10_000)
    );

    for method_name in ["merge_sort"] {
        bench_logger.log(format!("Running {}...", method_name));
        let method = methods.get(method_name).unwrap();

        let mut logger = Logger::new(format!("benchmarks/{}_benchmark.log", method_name), true);
        run_benchmark(method.0, method.1, MAX_DURATION, &mut logger);
    }
}

fn main() {
    run_all_benchmarks();

    // let mut logger = Logger::new(String::from("logs/quicksort_benchmark.log"), true);
    // run_benchmark(merge_sort, 10_000, MAX_DURATION, &mut logger);
}

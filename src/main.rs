mod sorters;
mod utils;

use std::time::{ Instant };
use utils::{ generate_random_arr, Logger };
use std::collections::{ HashMap };
use sorters::{ bubble_sort };

fn run_benchmark(method: for<'r> fn(&'r mut std::vec::Vec<i32>), max_duration: f64, logger: &mut Logger) -> HashMap<u32, f64> {
    let mut timings = HashMap::new();

    let mut length = 1000;
    loop {
        let mut arr: Vec<i32> = generate_random_arr(length);

        let now = Instant::now();
        method(&mut arr);
        let elapsed = now.elapsed();

        timings.insert(length, elapsed.as_secs_f64());
        logger.log(format!("    {{\"length\": {}, \"time\": {} }},", length, elapsed.as_secs_f64()));
        if elapsed.as_secs_f64() > max_duration {
            break;
        }
        length = length + 1000;
    }

    timings
}

fn main() {
    let mut methods: HashMap<&str, for<'r> fn(&'r mut std::vec::Vec<i32>)> = HashMap::new();

    methods.insert(
        "bubble_sort",
        bubble_sort,
    );

    let method_name = "bubble_sort";
    let method = methods.get(method_name).unwrap();

    let mut logger = Logger::new(format!("logs/{}_benchmark.log", method_name));
    logger.log(String::from("["));
    run_benchmark(*method, 1.0, &mut logger);
    logger.log(String::from("]"));
}

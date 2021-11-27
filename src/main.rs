use std::time::{ Instant };

fn bubble_sort(array: &mut Vec<u32>) {
    let mut made_changes_this_loop = true;

    while made_changes_this_loop {
        made_changes_this_loop = false;
        for i in 0..(array.len() - 1) {
            let item1 = array[i];
            let item2 = array[i + 1];

            if item2 < item1 {
                array[i] = item2;
                array[i + 1] = item1;
                made_changes_this_loop = true;
            }
        }
    }
}


fn main() {
    let array_lengths = [10, 100, 1000, 10_000, 100_000];

    for length in array_lengths {
        let now = Instant::now();

        let mut arr: Vec<u32> = Vec::new();

        bubble_sort(&mut arr);

        let elapsed = now.elapsed();
        println!("Length: {} {}s", length, elapsed.as_secs_f64());
    }

}

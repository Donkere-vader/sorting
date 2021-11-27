use rand::prelude::*;

pub fn bubble_sort(array: &mut Vec<i32>) {
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

pub fn insertion_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for i2 in 0..i {
            if array[i] < array[i2] {
                array.insert(i2, array[i]);
                array.remove(i + 1);
            }
        }
    }
}

pub fn boggo_sort(array: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    let mut sorted = false;
    while !sorted {
        array.shuffle(&mut rng);

        sorted = true;
        for i in 0..(array.len() - 1) {
            if array[i] > array[i + 1] {
                sorted = false;
                break;
            }
        }
    }
}

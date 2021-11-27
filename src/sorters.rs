use rand::prelude::*;

pub fn bubble_sort<T: std::cmp::PartialOrd>(array: &mut Vec<T>) where T: Copy {
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

pub fn insertion_sort<T: std::cmp::PartialOrd>(array: &mut Vec<T>) where T: Copy {
    for i in 0..array.len() {
        for i2 in 0..i {
            if array[i] < array[i2] {
                array.insert(i2, array[i]);
                array.remove(i + 1);
            }
        }
    }
}

pub fn quick_sort<T: std::cmp::PartialOrd>(array: &mut Vec<T>) where T: Copy {
    if array.len() == 1 {
        return;
    } else if array.len() == 2 {
        if array[0] > array[1] {
            let x = array[0];
            array[0] = array[1];
            array[1] = x;
        }
        return;
    }

    let mut left: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();

    let mut idx = 0;
    let mut value: T = array[0];
    while left.iter().len() == 0 || right.iter().len() == 0 {
        left = Vec::new();
        right = Vec::new();

        value = array[idx];

        for (cur_idx, val) in array.iter().enumerate() {
            if cur_idx == idx {
                continue;
            }
            if val <= &value {
                left.push(*val);
            } else {
                right.push(*val);
            }
        }

        idx += 1;
    }

    quick_sort(&mut left);
    quick_sort(&mut right);

    let mut idx = 0;
    for arr in [&left, &vec![value], &right] {
        for val in arr.iter() {
            array[idx] = *val;
            idx += 1;
        }
    }
}

pub fn boggo_sort<T: std::cmp::PartialOrd>(array: &mut Vec<T>) {
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

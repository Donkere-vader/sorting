use rand::prelude::*;
use std::thread;
use num_cpus;

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

pub fn threaded_quick_sort(array: &mut Vec<i32>) {
    let n_cpus = num_cpus::get();
    if array.iter().len() < (n_cpus * 2) {
        quick_sort(array);
        return;
    }

    let mut sub_vectors: Vec<Vec<i32>> = Vec::new();
    let cut_idx: Vec<usize> = (0..n_cpus).collect();
    let mut cut_values: Vec<i32> = Vec::new();
    for i in 0..(n_cpus - 1) {
        sub_vectors.push(Vec::new());
        cut_values.push(array[cut_idx[i]]);
    }
    quick_sort(&mut cut_values);

    let mut rest_arr = Vec::new();
    for num in array.iter() {
        let mut prev_cut_value = &i32::MIN;
        let mut placed = false;
        for (idx, cut_value) in cut_values.iter().enumerate() {
            if num < cut_value && num > prev_cut_value {
                sub_vectors[idx].push(*num);
                placed = true;
                break;
            }
            prev_cut_value = cut_value;
        }
        if !placed {
            rest_arr.push(*num);
        }
    }
    sub_vectors.push(rest_arr);

    let mut threads: Vec<thread::JoinHandle<Vec<i32>>> = Vec::new();

    for sub in sub_vectors {
        threads.push(
                thread::spawn(move || {
                let mut s: Vec<i32> = (*sub).to_vec();
                quick_sort(&mut s);

                s
            })
        );
    }

    array.clear();

    for handle in threads {
        match handle.join() {
            Ok(mut sub_vector) => { array.append(&mut sub_vector) },
            _ => {},
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

pub fn merge_sort(array: &mut Vec<i32>) {
    if array.len() == 1 {
        return;
    }

    let halfpoint = array.len() / 2;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for (idx, value) in array.iter().enumerate() {
        if idx < halfpoint {
            left.push(*value);
        } else {
            right.push(*value);
        }
    }

    merge_sort(&mut left);
    merge_sort(&mut right);

    array.clear();

    while left.len() > 0 && right.len() > 0 {
        if left[0] < right[0] {
            array.push(left.remove(0));
        } else {
            array.push(right.remove(0))
        }
    }

    array.append(&mut left);
    array.append(&mut right);
}

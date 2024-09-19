mod heap_sort;
mod sorting;

use rand::prelude::*;
use std::time::Instant;

fn benchmark(sort_fn: fn(&mut [u32]), array: &mut [u32], sort_name: &str) {
    let start = Instant::now();
    sort_fn(array);
    let duration = start.elapsed();
    let result = if is_sorted(array) {
        "APPROVED"
    } else {
        "REJECTED"
    };
    println!("[{}] {} took: {:?}", result, sort_name, duration);
}

fn is_sorted(array: &[u32]) -> bool {
    let mut prev = 0;
    for &value in array {
        if value < prev {
            return false;
        }
        prev = value;
    }
    true
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut array = [0; 100_000];
    for i in 0..array.len() {
        array[i] = rng.gen_range(0..=100);
    }

    let algorithms: &[(fn(&mut [u32]), &str)] = &[
        (heap_sort::heap_sort, "Heap sort"),
        (sorting::merge_sort::merge_sort, "Merge sort"),
        (sorting::bucket_sort::bucket_sort, "Bucket sort"),
        (sorting::counting_sort::counting_sort, "Counting sort"),
        (sorting::quick_sort::hoares_quick_sort, "Hoare's quick sort"),
        (
            sorting::quick_sort::lomutos_quick_sort,
            "Lomuto's quick sort",
        ),
        (sorting::insertion_sort::insertion_sort, "Insertion sort"),
        (sorting::selection_sort::selection_sort, "Selection sort"),
    ];

    for (algo, algo_name) in algorithms {
        benchmark(*algo, &mut array.clone(), algo_name)
    }

    let start = Instant::now();
    array.sort();
    let duration = start.elapsed();
    println!("The standard sorting algorithm took {:?}", duration);
}

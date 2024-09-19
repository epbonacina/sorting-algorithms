mod heap_sort;
mod sorting;

use rand::prelude::*;
use std::time::Instant;

fn benchmark(sort_fn: fn(&mut [u32]), array: &mut [u32], sort_name: &str) {
    let start = Instant::now();
    sort_fn(array);
    let duration = start.elapsed();
    // println!("{:?} sorted array: {:?}", sort_name, array);
    println!("{} took: {:?}", sort_name, duration);
    println!("============================");
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut base_array = vec![0; 1_000_000];
    for i in 0..base_array.len() {
        base_array[i] = rng.gen_range(0..=100);
    }
    // base_array = [47, 24, 22, 55, 80, 92, 99, 8, 0, 31];
    // println!("Base array: {base_array:?}");
    println!("============================");

    let mut array = base_array.clone();
    benchmark(heap_sort::heap_sort, &mut array, "Heap Sort");

    let mut array = base_array.clone();
    benchmark(sorting::merge_sort::merge_sort, &mut array, "Merge Sort");

    let mut array = base_array.clone();
    benchmark(sorting::bucket_sort::bucket_sort, &mut array, "Bucket Sort");

    let mut array = base_array.clone();
    benchmark(sorting::counting_sort::counting_sort, &mut array, "Counting Sort");

    let mut array = base_array.clone();
    benchmark(sorting::quick_sort::hoares_quick_sort, &mut array, "Hoare's Quick Sort");

    let mut array = base_array.clone();
    benchmark(sorting::quick_sort::lomutos_quick_sort, &mut array, "Lomuto's Quick Sort");

    let mut array = base_array.clone();
    benchmark(sorting::insertion_sort::insertion_sort, &mut array, "Insertion Sort");

    let mut array = base_array.clone();
    benchmark(sorting::selection_sort::selection_sort, &mut array, "Selection Sort");
}

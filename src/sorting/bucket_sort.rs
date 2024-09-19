use std::collections::LinkedList;

pub fn bucket_sort(array: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    let max_key = array.iter().max().unwrap();

    let mut buckets = Vec::new();

    for _ in 0..=*max_key {
        buckets.push(LinkedList::new());
    }

    for key in array.iter() {
        buckets[*key as usize].push_back(*key);
    }

    let mut index = 0;
    for bucket in buckets {
        for &value in bucket.iter() {
            array[index] = value;
            index += 1;
        }
    }
}

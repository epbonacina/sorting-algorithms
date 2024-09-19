use std::collections::LinkedList;


pub fn radix_sort(array: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    let radix = get_radix(array);

    for i in 0..get_number_of_iterations(radix){
        bucket_sort(array, radix.pow(i))
    }
}

fn get_radix(array: &[u32]) -> u32 {
    let power = f64::log2(array.len() as f64).floor() as u32;
    2u32.pow(power)
}

fn get_number_of_iterations(radix: u32) -> u32 {
    ((u32::BITS as f64) / f64::log2(radix as f64)).ceil() as u32
}

fn bucket_sort(array: &mut [u32], radix: u32) {
    // let mut buckets = Vec::new();


}

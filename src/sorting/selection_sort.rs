pub fn selection_sort(array: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    for partition_index in 0..array.len() {
        let mut min = partition_index;
        for i in partition_index..array.len() {
            if array[i] < array[min] {
                min = i;
            }
        }
        array.swap(partition_index, min);
    }
}

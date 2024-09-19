pub fn insertion_sort(array: &mut [u32]) {
    for partition_index in 1..array.len() {
        let value = array[partition_index];
        for candidate_index in (1..=partition_index).rev() {
            if array[candidate_index - 1] <= value {
                break;
            } else {
                array[candidate_index] = array[candidate_index - 1];
                array[candidate_index - 1] = value;
            }
        }
    }
}

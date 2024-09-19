pub fn merge_sort(array: &mut [u32]) {
    let mut aux = vec![0; array.len()];
    sort(array, &mut aux);
}

fn sort(array: &mut [u32], aux: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    let length = array.len();
    sort(&mut array[..length / 2], &mut aux[..length / 2]);
    sort(&mut array[length / 2..], &mut aux[length / 2..]);

    merge(&array[..length / 2], &array[length / 2..], aux);
    array.copy_from_slice(aux);
}

fn merge(left: &[u32], right: &[u32], aux: &mut [u32]) {
    let mut r = 0;
    let mut l = 0;

    for i in 0..aux.len() {
        if l >= left.len() {
            aux[i] = right[r];
            r += 1;
        } else if r >= right.len() {
            aux[i] = left[l];
            l += 1;
        } else if left[l] <= right[r] {
            aux[i] = left[l];
            l += 1;
        } else {
            aux[i] = right[r];
            r += 1;
        }
    }
}

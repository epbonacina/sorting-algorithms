pub fn counting_sort(array: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    let max_key = array.iter().max().unwrap();

    let mut aux = vec![0; *max_key as usize + 1];

    for key in array.iter() {
        aux[*key as usize] += 1;
    }

    let mut prev = 0;
    for i in 0..aux.len() {
        let tmp = prev;
        prev += aux[i];
        aux[i] = tmp;
    }

    let mut output = vec![0; array.len()];
    for value in array.into_iter() {
        let index = aux[*value as usize];
        aux[*value as usize] += 1;
        output[index] = *value;
    }

    array.copy_from_slice(&output);
}

use rand::Rng;

pub fn hoares_quick_sort(array: &mut [u32]) {
    quick_sort(array, 0, array.len() - 1, hoares_partitioning_scheme);
}

pub fn lomutos_quick_sort(array: &mut [u32]) {
    quick_sort(array, 0, array.len() - 1, lomutos_partitioning_scheme);
}

fn quick_sort(
    array: &mut [u32],
    low: usize,
    high: usize,
    partitioning_scheme: fn(&mut [u32], usize, usize) -> usize,
) {
    if low >= high {
        return;
    }

    let mut rng = rand::thread_rng();
    let pivot_index = rng.gen_range(low..=high);
    array.swap(pivot_index, high);

    let partition_index = partitioning_scheme(array, low, high);

    if partition_index > 1 {
        quick_sort(array, low, partition_index - 1, partitioning_scheme);
    }
    quick_sort(array, partition_index + 1, high, partitioning_scheme);
}

fn hoares_partitioning_scheme(array: &mut [u32], low: usize, high: usize) -> usize {
    let mut i = low;
    let mut j = high - 1;

    loop {
        while array[j] > array[high] && i <= j {
            if let Some(new_j) = j.checked_sub(1) {
                j = new_j;
            } else {
                array.swap(j, high);
                return j;
            }
        }

        while array[i] < array[high] && i <= j {
            i += 1;
        }

        if i >= j {
            break;
        }

        array.swap(i, j);
        i += 1;
        j = j.saturating_sub(1);
    }

    array.swap(high, j + 1);
    j + 1
}

fn lomutos_partitioning_scheme(array: &mut [u32], low: usize, high: usize) -> usize {
    let mut i = low;
    let mut j = low;

    while j < high {
        if array[j] < array[high] {
            array.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    array.swap(high, i);
    i
}

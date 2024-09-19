struct MaxHeap<'a> {
    elements: &'a mut [u32],
    size: usize,
}

impl<'a> MaxHeap<'a> {
    fn new(array: &'a mut [u32]) -> MaxHeap<'a> {
        let size = array.len();
        MaxHeap { elements: array, size }
    }

    fn build_heap(&mut self) {
        if self.size == 0 {
            return;
        }
        
        let mut index = (self.size / 2) - 1;
        loop {
            self.sift_down(index);
            if index == 0 {
                break;
            }
            index -= 1;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left_index = 2 * index + 1;
            let right_index = 2 * index + 2;
            
            if left_index >= self.size {
                break;
            }

            let chosen_index = if right_index < self.size && self.elements[right_index] > self.elements[left_index] {
                right_index
            } else {
                left_index
            };

            if self.elements[chosen_index] > self.elements[index] {
                self.elements.swap(chosen_index, index);
                index = chosen_index;
            } else {
                break;
            }
        }
    }

    fn move_max(&mut self) {
        if self.size == 0 {
            return;
        }

        self.elements.swap(0, self.size - 1);
        self.size -= 1;

        if self.size > 0 {
            self.sift_down(0);
        }
    }
}

pub fn heap_sort(array: &mut [u32]) {
    let length = array.len();
    let mut heap = MaxHeap::new(array);
    heap.build_heap();
    
    for _ in (0..length).rev() {
        heap.move_max();
    }
}

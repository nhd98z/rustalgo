struct Heap<T, F>
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    data: Vec<T>,
    comparator: F,
}

impl<T, F> Heap<T, F>
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    // Create a new heap with a custom comparator
    pub fn new(comparator: F) -> Self {
        Self {
            data: Vec::new(),
            comparator,
        }
    }

    // Get the number of elements in the heap
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Check if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Get the top element of the heap
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    // Push a new element onto the heap
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.len() - 1);
    }

    // Remove and return the top element from the heap
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // Swap the first with the last element
        let last_idx = self.len() - 1;
        self.data.swap(0, last_idx);

        // Remove and get the top element (which is now at the end)
        let top = self.data.pop();

        // If the heap is not empty, fix the heap property
        if !self.is_empty() {
            self.heapify_down(0);
        }

        top
    }

    // Fix the heap property by moving the element up
    fn heapify_up(&mut self, mut idx: usize) {
        // While not at the root
        while idx > 0 {
            let parent = (idx - 1) / 2;

            // Use the comparator to check if the heap property is satisfied
            if (self.comparator)(&self.data[parent], &self.data[idx]) {
                break;
            }

            // Otherwise, swap the parent and child
            self.data.swap(parent, idx);

            // Move up to the parent
            idx = parent;
        }
    }

    // Fix the heap property by moving the element down
    fn heapify_down(&mut self, mut idx: usize) {
        let len = self.len();

        loop {
            // Get the left and right children
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut target = idx;

            // Find the target element among the current and children
            if left < len && !(self.comparator)(&self.data[target], &self.data[left]) {
                target = left;
            }

            if right < len && !(self.comparator)(&self.data[target], &self.data[right]) {
                target = right;
            }

            // If the target is the current element, we're done
            if target == idx {
                break;
            }

            // Otherwise, swap the current element with the target
            self.data.swap(idx, target);

            // Move down to the target
            idx = target;
        }
    }
}

fn main() {
    // Create a min heap (smaller elements have higher priority)
    let mut min_heap = Heap::new(|a: &i32, b: &i32| a <= b);

    // Add some elements
    min_heap.push(5);
    min_heap.push(3);
    min_heap.push(8);
    min_heap.push(1);
    min_heap.push(10);
    min_heap.push(7);
    min_heap.push(2);

    println!("Min heap:");
    // Get the top element
    println!("Peek: {}", min_heap.peek().unwrap());
    // Pop elements in order (should be in ascending order)
    while let Some(val) = min_heap.pop() {
        println!("Popped: {}", val);
    }

    // Create a max heap (larger elements have higher priority)
    let mut max_heap = Heap::new(|a: &i32, b: &i32| a >= b);

    // Add some elements
    max_heap.push(5);
    max_heap.push(3);
    max_heap.push(8);
    max_heap.push(1);
    max_heap.push(10);
    max_heap.push(7);
    max_heap.push(2);

    println!("\nMax heap:");
    // Get the top element
    println!("Peek: {}", max_heap.peek().unwrap());
    // Pop elements in order (should be in descending order)
    while let Some(val) = max_heap.pop() {
        println!("Popped: {}", val);
    }
}

struct MyHashMap {
    items: [i32; 1000000 + 1],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            items: [-1; 1000000 + 1],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.items[key as usize] = value
    }

    fn get(&self, key: i32) -> i32 {
        self.items[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.items[key as usize] = -1
    }
}

fn main() {
    let mut obj = MyHashMap::new();
    obj.put(2, 3);
    let ret_2: i32 = obj.get(2);
    dbg!(ret_2);
    obj.remove(2);
}

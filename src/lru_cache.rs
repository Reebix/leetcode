pub struct LRUCache {
    cache: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    key: i32,
    value: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub(crate) fn new(capacity: i32) -> Self {
        LRUCache { cache: Vec::with_capacity(capacity as usize) }
    }

    fn move_to_head(&mut self, index: usize) {
        let tmp = self.cache.remove(index);
        self.cache.insert(0, tmp);
    }

    pub(crate) fn get(&mut self, key: i32) -> i32 {
        if let Some(idx) = self.cache.iter().position(|entry| { entry.key == key }) {
            let value = self.cache[idx].value;
            self.move_to_head(idx);
            return value;
        }
        -1
    }

    pub(crate) fn put(&mut self, key: i32, value: i32) {
        if let Some(idx) = self.cache.iter().position(|entry| entry.key == key) {
            self.cache[idx].value = value;
            self.move_to_head(idx);
            return;
        }

        let capacity = self.cache.capacity();
        if self.cache.len() >= capacity {
            self.cache.remove(capacity - 1);
        }
        self.cache.insert(0, Entry { key, value });
    }
}
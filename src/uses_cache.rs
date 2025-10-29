pub struct LRUCache {
    cache: Vec<Entry>,
}

struct Entry {
    key: i32,
    value: i32,
    uses: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub(crate) fn new(capacity: i32) -> Self {
        LRUCache { cache: Vec::with_capacity(capacity as usize) }
    }


    fn move_in_cache(&mut self, mut index: usize) {
        while index > 0 {
            if self.cache[index].uses < self.cache[index - 1].uses {
                break;
            }
            self.cache.swap(index, index - 1);
            index -= 1;
        }
    }

    pub(crate) fn get(&mut self, key: i32) -> i32 {
        if let Some(idx) = self.cache.iter().position(|entry| { entry.key == key }) {
            self.cache[idx].uses += 1;
            let value = self.cache[idx].value;
            self.move_in_cache(idx);
            return value;
        }
        -1
    }

    pub(crate) fn put(&mut self, key: i32, value: i32) {
        if let Some(idx) = self.cache.iter().position(|entry| entry.key == key) {
            self.cache[idx].uses += 1;
            self.cache[idx].value = value;
            self.move_in_cache(idx);
            return;
        }

        let len = &self.cache.len();
        if *len == self.cache.capacity() {
            self.cache[*len - 1] = Entry { key, value, uses: 1 };
        } else {
            self.cache.push(Entry { key, value, uses: 1 });
        }
    }
}
use std::collections::HashMap;

// a simple LRU cache
// it is a simple implementation of a LRU cache
// you can specify a max size for the cache
// you can retrieve and store a value.
// if the cache is full, the oldest value will be removed

pub struct LruCache {
  capacity: usize,
  map: HashMap<String, String>,
  list: Vec<String>,
}

impl LruCache {
    pub fn new(max_size: usize) -> Self {
        LruCache {
            capacity: max_size,
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let value = self.map.get(key)?.clone();
        
        self.refresh(key);

        Some(value.clone())
    }

    fn refresh(&mut self, key: &str) {
        let item_position = self.list.iter().position(|list_key| list_key == key);
        if let Some(item_position) = item_position {
          self.list.remove(item_position);
          self.list.push(key.to_owned());
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        if self.list.len() == self.capacity {
            let oldest_key = self.list.remove(0);
            self.map.remove(&oldest_key);
        }

        self.list.push(key.to_owned());
        self.map.insert(key.to_owned(), value.to_owned());
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut cache = LruCache::new(2);
        cache.set("key1", "value1");
        cache.set("key2", "value2");
        // getting key1 later should make it the most recent value
        assert_eq!(cache.get("key2"), Some("value2".to_string()));
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        
        // setting a new value should remove the oldest value
        cache.set("key3", "value3");
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None); 
        assert_eq!(cache.get("key3"), Some("value3".to_string()));
    }
}
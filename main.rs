use std::{collections::HashMap, cell::RefCell};
use thiserror::Error;

// a simple LRU cache
// it is a simple implementation of a LRU cache
// you can specify a max size for the cache
// you can retrieve and store a value.
// if the cache is full, the oldest value will be removed

type MyBytes = Vec<u8>;
pub struct LruCache {
  capacity: usize,
  map: HashMap<String, MyBytes>,
  list: RefCell<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum CacheError {
  #[error("conversion to bytes failed: {0}")]
  ConversionFailed(String),
}


pub trait TryIntoBytes {
  fn try_into_bytes(self) -> Result<Vec<u8>, CacheError>;
}

pub trait TryFromBytes {
  fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, CacheError>
    where Self: Sized;
}

// impl <T> IntoBytes for T 
//   where T: Into<Vec<u8>>
// {
//   fn into_bytes(self) -> Vec<u8> {
//     self.into()
//   }
// }

impl TryIntoBytes for String {
  fn try_into_bytes(self) -> Result<Vec<u8>, CacheError> {
    Ok(self.into_bytes())
  }
}

impl TryFromBytes for String {
  fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, CacheError> {
    Ok(String::from_utf8(bytes)
      .map_err(|e| CacheError::ConversionFailed(e.to_string()))?)
  }
}

impl TryIntoBytes for i32 {
  fn try_into_bytes(self) -> Result<Vec<u8>, CacheError> {
    Ok(self.to_be_bytes().to_vec())
  }
}

impl TryFromBytes for i32 {
  fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, CacheError> {
    Ok(i32::from_be_bytes(bytes.try_into().unwrap()))
  }
}

impl LruCache {
    pub fn new(max_size: usize) -> Self {
        LruCache {
            capacity: max_size,
            map: HashMap::new(),
            list: RefCell::new(Vec::new()),
        }
    }

    pub fn get<V: Clone + TryFromBytes>(&self, key: &str) -> Option<V> {
        let value = self.map.get(key)?.clone();
        
        self.refresh(key);

        Some(value.clone())
    }

    fn refresh(&self, key: &str) {
        let item_position = self.list.borrow()
          .iter()
          .position(|list_key| list_key == key);

        if let Some(item_position) = item_position {
          let mut list = self.list.borrow_mut();
          list.remove(item_position);
          list.push(key.to_owned());
        }
    }

    pub fn set<V: Clone + TryIntoBytes>(&mut self, key: &str, value: &V) {
        if self.list.borrow().len() == self.capacity {
            let oldest_key = self.list.borrow_mut().remove(0);
            self.map.remove(&oldest_key);
        }

        self.list.borrow_mut().push(key.to_owned());
        self.map.insert(key.to_owned(), value.into_bytes());
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
}

impl TryIntoBytes for Person {
  fn try_into_bytes(self) -> Result<Vec<u8>, CacheError> {
    let mut bytes = Vec::new();
    bytes.extend(self.name.try_into_bytes()?);
    bytes.extend(self.age.try_into_bytes()?);
    Ok(bytes)
  }
}

impl TryFromBytes for Person {
  fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, CacheError> {
    let name = String::try_from_bytes(bytes[0..bytes.len() - 1].to_vec())?;
    let age = i32::try_from_bytes(bytes[bytes.len() - 1..].to_vec())?;
    Ok(Person { name, age })
  }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_strings() {
        let mut cache = LruCache::new(2);
        cache.set("key1", &"value1".to_string());
        cache.set("key2", &"value2".to_string());
        // getting key1 later should make it the most recent value
        assert_eq!(cache.get("key2"), Some("value2".to_string()));
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        
        // setting a new value should remove the oldest value
        cache.set("key3", &"value3".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None); 
        assert_eq!(cache.get("key3"), Some("value3".to_string()));
    }

    #[test]
    fn it_works_for_numbers() {
        let mut cache = LruCache::new(2);
        cache.set("key1", &1);
        cache.set("key2", &2);
        // getting key1 later should make it the most recent value
        assert_eq!(cache.get("key2"), Some(2));
        assert_eq!(cache.get("key1"), Some(1));
    }

    #[test]
    fn it_works_for_structs() {
        let mut cache = LruCache::new(2);
        cache.set("key1", &Person { name: "John".to_string(), age: 20 });
        cache.set("key2", &Person { name: "Jane".to_string(), age: 21 });
        // getting key1 later should make it the most recent value
        assert_eq!(cache.get("key2").try_from_byte<Person>(), Some(Person { name: "Jane".to_string(), age: 21 }));
        assert_eq!(cache.get("key1"), Some(Person { name: "John".to_string(), age: 20 }));
    }
}
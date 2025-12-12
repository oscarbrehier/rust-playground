use crate::Iter;
use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct HashSet<T> {
    buckets: Vec<Vec<T>>,
    size: usize,
}

fn create_buckets<T>(size: usize) -> Vec<Vec<T>> {
    std::iter::repeat_with(Vec::new).take(size).collect()
}

impl<T: Hash + Eq + fmt::Debug> fmt::Debug for HashSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T: Hash + Eq + Clone> Clone for HashSet<T> {
    fn clone(&self) -> Self {
        Self {
            buckets: self.buckets.clone(),
            size: self.size,
        }
    }
}

impl<T: Hash + Eq> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq> FromIterator<T> for HashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for item in iter {
            set.insert(item);
        }

        set
    }
}

impl<T> HashSet<T>
where
    T: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: create_buckets::<T>(16),
            size: 0,
        }
    }

    fn hash(&self, value: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    pub fn insert(&mut self, value: T) -> bool {
        if (self.size + 1) * 4 > self.buckets.len() * 3 {
            self.resize();
        }

        let index = self.hash(&value);
        let bucket = &mut self.buckets[index];

        if bucket.iter().any(|v| v == &value) {
            return false;
        }

        bucket.push(value);
        self.size += 1;

        true
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Hash + Eq + ?Sized,
        T: Borrow<Q>,
    {
        let index = {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            (hasher.finish() as usize) % self.buckets.len()
        };
        self.buckets[index].iter().any(|v| v.borrow() == value)
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: Hash + Eq + ?Sized,
        T: Borrow<Q>,
    {
        let index = {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            (hasher.finish() as usize) % self.buckets.len()
        };

        let bucket = &mut self.buckets[index];

        if let Some(pos) = bucket.iter().position(|v| v.borrow() == value) {
            bucket.remove(pos);
            self.size -= 1;
            return true;
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.size = 0
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let mut new_buckets = create_buckets::<T>(new_capacity);

        for bucket in &mut self.buckets {
            for value in std::mem::take(bucket) {
                let mut hasher = DefaultHasher::new();
                value.hash(&mut hasher);
                let new_index = (hasher.finish() as usize) % new_capacity;
                new_buckets[new_index].push(value);
            }
        }

        self.buckets = new_buckets
    }

    pub fn iter(&self) -> Iter<'_, T> {
        let mut bucket_iter = self.buckets.iter();
        let current_bucket = bucket_iter.next().map(|b| b.iter());

        Iter {
            bucket_iter,
            current_bucket,
        }
    }
}

#[test]
fn test_insert_and_contains() {
    let mut set = HashSet::new();
    assert!(set.insert(42));
    assert!(!set.insert(42)); // Duplicate
    assert!(set.contains(&42));
    assert!(!set.contains(&99));
}
#[test]
fn test_remove() {
    let mut set = HashSet::new();
    set.insert(42);
    assert!(set.remove(&42));
    assert!(!set.remove(&42)); // Already removed
    assert!(!set.contains(&42));
}

#[test]
fn test_size_tracking() {
    let mut set = HashSet::new();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
    set.insert(1);
    set.insert(2);
    assert_eq!(set.len(), 2);
    set.remove(&1);
    assert_eq!(set.len(), 1);
    set.clear();
    assert_eq!(set.len(), 0);
    assert_eq!(set.capacity(), 16); // Capacity unchanged
}

#[test]
fn test_resize() {
    let mut set = HashSet::new();
    let initial_capacity = set.capacity();
    // Insert enough to trigger resize (> 0.75 * 16 = 12)
    for i in 0..20 {
        set.insert(i);
    }
    assert!(set.capacity() > initial_capacity);
    assert_eq!(set.len(), 20);
    // Verify all elements still present
    for i in 0..20 {
        assert!(set.contains(&i));
    }
}

#[test]
fn test_iterator() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let mut collected: Vec<_> = set.iter().copied().collect();
    collected.sort();
    assert_eq!(collected, vec![1, 2, 3]);
}
#[test]
fn test_for_loop() {
    let mut set = HashSet::new();
    set.insert("hello");
    set.insert("world");
    let mut count = 0;
    for item in set.iter() {
        count += 1;
        assert!(item == &"hello" || item == &"world");
    }
    assert_eq!(count, 2);
}

#[test]
fn test_debug() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    let debug_str = format!("{:?}", set);
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
}
#[test]
fn test_clone() {
    let mut set1 = HashSet::new();
    set1.insert(42);
    let mut set2 = set1.clone();
    set2.insert(99);
    assert!(set1.contains(&42));
    assert!(!set1.contains(&99)); // Independent copy
    assert!(set2.contains(&42));
    assert!(set2.contains(&99));
}
#[test]
fn test_from_iterator() {
    let set: HashSet<_> = vec![1, 2, 2, 3].into_iter().collect();
    assert_eq!(set.len(), 3); // Duplicates removed
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(set.contains(&3));
}
#[test]
fn test_default() {
    let set: HashSet<i32> = Default::default();
    assert!(set.is_empty());
}

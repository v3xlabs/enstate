use std::collections::HashSet;
use std::hash::Hash;

pub fn dedup_ord<T: Clone + Hash + Eq>(src: &[T]) -> Vec<T> {
    let mut set = HashSet::new();

    let mut copy = src.to_vec();
    copy.retain(|item| set.insert(item.clone()));

    copy
}

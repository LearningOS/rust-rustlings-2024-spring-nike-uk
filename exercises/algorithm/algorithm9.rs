/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if right_idx > self.count || (self.comparator)(&self.items[left_idx], &self.items[right_idx])
        {
            left_idx
        } else {
            right_idx
        }
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {
            let parent_idx = self.parent_idx(idx);
            let item = self.items.remove(idx);
            self.items.insert(parent_idx, item);
            idx = parent_idx;
        }
    }
    
    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let smallest_child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[idx]) {
                self.items.swap(smallest_child_idx, idx);
                idx = smallest_child_idx;
            } else {
                break;
            }
        }
    }    
    
}

impl<T: Clone> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let max_item = self.items[1].clone();
        self.items.swap(1, self.count);
        self.count -= 1;
        self.items.pop();
        self.bubble_down(1);
        Some(max_item)
    }
}

pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}

/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Ord,
{
    items: Vec<T>,  // 使用 Vec 来存储堆元素
    comparator: fn(&T, &T) -> bool,  // 比较函数，用于区分最小堆和最大堆
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    // 创建新堆的构造函数，需要一个比较器来定义是最大堆还是最小堆
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![T::default()],  // 索引从 1 开始
            comparator,
        }
    }

    // 返回堆的元素数量
    pub fn len(&self) -> usize {
        self.items.len() - 1  // 减去索引0
    }

    // 检查堆是否为空
    pub fn is_empty(&self) -> bool {
        self.items.len() == 1
    }

    // 添加元素到堆中
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        let mut idx = self.items.len() - 1;
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[idx / 2]) {
            self.items.swap(idx, idx / 2);
            idx /= 2;
        }
    }

    // 提取堆顶元素并重新调整堆
    pub fn pop(&mut self) -> Option<T> {
        if self.items.len() > 1 {
            let result = Some(self.items.swap_remove(1));
            self.heapify(1);
            result
        } else {
            None
        }
    }

    // 内部调整堆的函数
    fn heapify(&mut self, idx: usize) {
        let mut largest = idx;
        let left = 2 * idx;
        let right = 2 * idx + 1;
        let len = self.items.len();

        if left < len && (self.comparator)(&self.items[left], &self.items[largest]) {
            largest = left;
        }

        if right < len && (self.comparator)(&self.items[right], &self.items[largest]) {
            largest = right;
        }

        if largest != idx {
            self.items.swap(idx, largest);
            self.heapify(largest);
        }
    }

    // 创建一个新的最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    // 创建一个新的最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        assert!(heap.is_empty());
        heap.add(5);
        heap.add(3);
        heap.add(8);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(8));
        assert!(heap.is_empty());
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        assert!(heap.is_empty());
        heap.add(5);
        heap.add(3);
        heap.add(8);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));
        assert!(heap.is_empty());
    }
}

//! 提供栈结构实现的分配器 [`Segtree_allocator`]

use super::super::Allocator;
use alloc::{vec, vec::Vec};

/// 每一页有4096bit
/// Bitmap 中的位数（4K）
const BITMAP_SIZE: usize = 4096;

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
/// 
pub struct SegtreeAllocator {
    list: Vec<(usize, usize)>,
}

impl Allocator for SegtreeAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.list.pop() {
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            Some(start)
        } else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        self.list.push((index, index + 1));
    }
}

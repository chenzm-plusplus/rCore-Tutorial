//! 提供栈结构实现的分配器 [`Segtree_allocator`]

use super::super::Allocator;
use alloc::{vec, vec::Vec};
use super::Segtree;


/// 每一页有4096bit
/// Bitmap 中的位数（4K）
const BITMAP_SIZE: usize = 4096;

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
/// 
pub struct SegtreeAllocator {
    list: Vec<Segtree>,
}

impl Allocator for SegtreeAllocator {
    //capacity表示一共有多少个可分配的页面
    fn new(capacity: usize) -> Self {
        // 这里对list做初始化，先填进去4*capacity个SegNode
        let mut v = Vec::new();
        // for i in 1..4*capacity+1{
        //     let node = SegNode{
        //         left_child: 0,
        //         right_child: 0,
        //         value: false,
        //     };
        //     v.push(node);
        // }
        // SegtreeAllocator::test();
        // //build_tree(1,1,capacity);
        // SegtreeAllocator::build_tree(1,1,capacity);
        Self {
            list: v,
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        // if let Some((start, end)) = self.list.pop() {
        //     if end - start > 1 {
        //         self.list.push((start + 1, end));
        //     }
        //     Some(start)
        // } else {
        //     None
        // }
        None
    }

    fn dealloc(&mut self, index: usize) {
        //self.list.push((index, index + 1));
    }
}
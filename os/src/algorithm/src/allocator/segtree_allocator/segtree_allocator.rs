//! 提供栈结构实现的分配器 [`Segtree_allocator`]

use super::super::Allocator;
use alloc::{vec, vec::Vec};


/// 每一页有4096bit
/// Bitmap 中的位数（4K）
const BITMAP_SIZE: usize = 4096;

/// 线段树的节点
pub struct SegNode {
    left_child: usize,
    right_child: usize,
    value: bool,//false表示儿子们当中还有未被分配的节点，true表示儿子们当中的节点以及被分配完了
}

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
/// 
pub struct SegtreeAllocator {
    list: Vec<SegNode>,
}

impl Allocator for SegtreeAllocator {
    //capacity表示一共有多少个可分配的页面
    fn new(capacity: usize) -> Self {
        // 这里对list做初始化，先填进去4*capacity个SegNode
        let mut v = Vec::new();
        for i in 1..4*capacity+1{
            let node = SegNode{
                left_child: 0,
                right_child: 0,
                value: false,
            };
            v.push(node);
        }
        SegtreeAllocator::test();
        //build_tree(1,1,capacity);
        Self {
            //list: vec![(0, capacity)],
            //println!("SegtreeAllocator instruction...");
            // list: vec![node;4*capacity],
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

impl SegtreeAllocator{
    /// initlize tree
    /// i:节点编号
    /// left：当前节点的左孩子是谁
    /// right：当前节点的右孩子是谁
    /// root number: 1
    fn build_tree(&mut self, i:usize, left:usize, right:usize){
        self.list[i].left_child=left;
        self.list[i].right_child=right;
        self.list[i].value=false;
        if(left==right){
            return
        }
        let mid=(left+right)/2;
        SegtreeAllocator::build_tree(self,i*2,left,mid);
        SegtreeAllocator::build_tree(self,i*2+1,mid+1,right);
    }
    fn test(){
        //println!("test pass!");
    }
}